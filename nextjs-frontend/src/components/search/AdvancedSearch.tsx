'use client';

import React, { useState, useCallback, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Badge } from '@/components/ui/Badge';
import { 
  Search, 
  Filter, 
  X, 
  FileText, 
  Code, 
  GitBranch,
  Clock,
  MapPin,
  Hash,
  Regex,
  CaseSensitive,
  Whole Word,
  Settings
} from 'lucide-react';
import { api } from '@/api/client';
import { GraphMatchResult, FunctionMatch } from '@/types';

interface SearchFilters {
  searchType: 'name' | 'content' | 'signature' | 'all';
  matchType: 'exact' | 'partial' | 'regex' | 'fuzzy';
  caseSensitive: boolean;
  wholeWord: boolean;
  includeComments: boolean;
  fileExtensions: string[];
  similarityThreshold: number;
  confidenceThreshold: number;
}

interface SearchResult {
  id: string;
  type: 'function' | 'file' | 'match';
  title: string;
  subtitle?: string;
  content: string;
  location: string;
  similarity?: number;
  confidence?: number;
  highlights: string[];
  metadata: Record<string, any>;
}

interface AdvancedSearchProps {
  data?: GraphMatchResult;
  onResultSelect?: (result: SearchResult) => void;
  onFilterChange?: (filters: SearchFilters) => void;
}

export function AdvancedSearch({ data, onResultSelect, onFilterChange }: AdvancedSearchProps) {
  const [query, setQuery] = useState('');
  const [isExpanded, setIsExpanded] = useState(false);
  const [filters, setFilters] = useState<SearchFilters>({
    searchType: 'all',
    matchType: 'partial',
    caseSensitive: false,
    wholeWord: false,
    includeComments: false,
    fileExtensions: [],
    similarityThreshold: 0.5,
    confidenceThreshold: 0.5,
  });

  // Search through the comparison data
  const searchResults = useMemo(() => {
    if (!data || !query.trim()) return [];

    const results: SearchResult[] = [];
    const searchTerm = filters.caseSensitive ? query : query.toLowerCase();

    // Helper function to check if text matches search criteria
    const matchesSearch = (text: string): boolean => {
      const searchText = filters.caseSensitive ? text : text.toLowerCase();
      
      switch (filters.matchType) {
        case 'exact':
          return filters.wholeWord 
            ? new RegExp(`\\b${searchTerm}\\b`).test(searchText)
            : searchText === searchTerm;
        case 'partial':
          return filters.wholeWord
            ? new RegExp(`\\b${searchTerm}\\b`).test(searchText)
            : searchText.includes(searchTerm);
        case 'regex':
          try {
            const flags = filters.caseSensitive ? 'g' : 'gi';
            return new RegExp(searchTerm, flags).test(searchText);
          } catch {
            return false;
          }
        case 'fuzzy':
          // Simple fuzzy matching - could be enhanced with a proper fuzzy search library
          return searchText.includes(searchTerm) || 
                 levenshteinDistance(searchText, searchTerm) <= 2;
        default:
          return searchText.includes(searchTerm);
      }
    };

    // Search function matches
    if (filters.searchType === 'all' || filters.searchType === 'name') {
      data.matches.forEach(match => {
        if (match.similarity.overall_similarity >= filters.similarityThreshold &&
            match.confidence >= filters.confidenceThreshold) {
          
          const sourceMatches = matchesSearch(match.source_id);
          const targetMatches = matchesSearch(match.target_id);
          
          if (sourceMatches || targetMatches) {
            results.push({
              id: `match-${match.source_id}-${match.target_id}`,
              type: 'match',
              title: `${match.source_id} → ${match.target_id}`,
              subtitle: `${match.match_type} match`,
              content: `Similarity: ${(match.similarity.overall_similarity * 100).toFixed(1)}%`,
              location: 'Function Match',
              similarity: match.similarity.overall_similarity,
              confidence: match.confidence,
              highlights: [
                sourceMatches ? match.source_id : '',
                targetMatches ? match.target_id : ''
              ].filter(Boolean),
              metadata: { match }
            });
          }
        }
      });
    }

    // Search additions
    if (filters.searchType === 'all' || filters.searchType === 'name') {
      data.additions.forEach(funcId => {
        if (matchesSearch(funcId)) {
          results.push({
            id: `addition-${funcId}`,
            type: 'function',
            title: funcId,
            subtitle: 'Added Function',
            content: 'New function added in target',
            location: funcId.split('::')[0] || 'Unknown',
            highlights: [funcId],
            metadata: { type: 'addition', functionId: funcId }
          });
        }
      });
    }

    // Search deletions
    if (filters.searchType === 'all' || filters.searchType === 'name') {
      data.deletions.forEach(funcId => {
        if (matchesSearch(funcId)) {
          results.push({
            id: `deletion-${funcId}`,
            type: 'function',
            title: funcId,
            subtitle: 'Removed Function',
            content: 'Function removed from source',
            location: funcId.split('::')[0] || 'Unknown',
            highlights: [funcId],
            metadata: { type: 'deletion', functionId: funcId }
          });
        }
      });
    }

    // Search moves
    data.moves.forEach(move => {
      if (matchesSearch(move.function_id) || 
          matchesSearch(move.source_file) || 
          matchesSearch(move.target_file)) {
        results.push({
          id: `move-${move.function_id}`,
          type: 'function',
          title: move.function_id,
          subtitle: 'Moved Function',
          content: `Moved from ${move.source_file} to ${move.target_file}`,
          location: move.target_file,
          similarity: move.similarity,
          confidence: move.confidence,
          highlights: [move.function_id],
          metadata: { type: 'move', move }
        });
      }
    });

    // Search renames
    data.renames.forEach(rename => {
      if (matchesSearch(rename.old_name) || matchesSearch(rename.new_name)) {
        results.push({
          id: `rename-${rename.function_id}`,
          type: 'function',
          title: `${rename.old_name} → ${rename.new_name}`,
          subtitle: 'Renamed Function',
          content: `Function renamed with ${(rename.similarity * 100).toFixed(1)}% similarity`,
          location: 'Function Rename',
          similarity: rename.similarity,
          confidence: rename.confidence,
          highlights: [rename.old_name, rename.new_name],
          metadata: { type: 'rename', rename }
        });
      }
    });

    return results.slice(0, 50); // Limit results for performance
  }, [data, query, filters]);

  // Simple Levenshtein distance for fuzzy matching
  const levenshteinDistance = (str1: string, str2: string): number => {
    const matrix = Array(str2.length + 1).fill(null).map(() => Array(str1.length + 1).fill(null));
    
    for (let i = 0; i <= str1.length; i++) matrix[0][i] = i;
    for (let j = 0; j <= str2.length; j++) matrix[j][0] = j;
    
    for (let j = 1; j <= str2.length; j++) {
      for (let i = 1; i <= str1.length; i++) {
        const indicator = str1[i - 1] === str2[j - 1] ? 0 : 1;
        matrix[j][i] = Math.min(
          matrix[j][i - 1] + 1,
          matrix[j - 1][i] + 1,
          matrix[j - 1][i - 1] + indicator
        );
      }
    }
    
    return matrix[str2.length][str1.length];
  };

  const handleFilterChange = useCallback((newFilters: Partial<SearchFilters>) => {
    const updatedFilters = { ...filters, ...newFilters };
    setFilters(updatedFilters);
    onFilterChange?.(updatedFilters);
  }, [filters, onFilterChange]);

  const getResultIcon = (result: SearchResult) => {
    switch (result.type) {
      case 'function': return <Code className="w-4 h-4" />;
      case 'file': return <FileText className="w-4 h-4" />;
      case 'match': return <GitBranch className="w-4 h-4" />;
      default: return <Search className="w-4 h-4" />;
    }
  };

  const getResultTypeColor = (result: SearchResult) => {
    if (result.metadata.type === 'addition') return 'text-green-600';
    if (result.metadata.type === 'deletion') return 'text-red-600';
    if (result.metadata.type === 'move') return 'text-blue-600';
    if (result.metadata.type === 'rename') return 'text-orange-600';
    return 'text-gray-600';
  };

  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2">
          <Search className="w-5 h-5" />
          Advanced Search
        </CardTitle>
        
        {/* Main Search Input */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input
            placeholder="Search functions, files, and changes..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="pl-10 pr-10"
          />
          {query && (
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setQuery('')}
              className="absolute right-1 top-1/2 transform -translate-y-1/2 h-6 w-6 p-0"
            >
              <X className="w-4 h-4" />
            </Button>
          )}
        </div>

        {/* Quick Filters */}
        <div className="flex items-center gap-2 flex-wrap">
          <Button
            variant={filters.caseSensitive ? "default" : "outline"}
            size="sm"
            onClick={() => handleFilterChange({ caseSensitive: !filters.caseSensitive })}
          >
            <CaseSensitive className="w-4 h-4 mr-1" />
            Aa
          </Button>
          
          <Button
            variant={filters.wholeWord ? "default" : "outline"}
            size="sm"
            onClick={() => handleFilterChange({ wholeWord: !filters.wholeWord })}
          >
            <Whole Word className="w-4 h-4 mr-1" />
            Word
          </Button>
          
          <Button
            variant={filters.matchType === 'regex' ? "default" : "outline"}
            size="sm"
            onClick={() => handleFilterChange({ 
              matchType: filters.matchType === 'regex' ? 'partial' : 'regex' 
            })}
          >
            <Regex className="w-4 h-4 mr-1" />
            Regex
          </Button>

          <Button
            variant="outline"
            size="sm"
            onClick={() => setIsExpanded(!isExpanded)}
          >
            <Settings className="w-4 h-4 mr-1" />
            Filters
          </Button>
        </div>

        {/* Advanced Filters */}
        {isExpanded && (
          <div className="space-y-4 p-4 border rounded-lg bg-muted/50">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">Search Type</label>
                <select
                  value={filters.searchType}
                  onChange={(e) => handleFilterChange({ searchType: e.target.value as any })}
                  className="w-full mt-1 px-3 py-2 border rounded-md text-sm"
                >
                  <option value="all">All</option>
                  <option value="name">Function Names</option>
                  <option value="content">Content</option>
                  <option value="signature">Signatures</option>
                </select>
              </div>
              
              <div>
                <label className="text-sm font-medium">Match Type</label>
                <select
                  value={filters.matchType}
                  onChange={(e) => handleFilterChange({ matchType: e.target.value as any })}
                  className="w-full mt-1 px-3 py-2 border rounded-md text-sm"
                >
                  <option value="partial">Partial</option>
                  <option value="exact">Exact</option>
                  <option value="regex">Regex</option>
                  <option value="fuzzy">Fuzzy</option>
                </select>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">
                  Similarity Threshold: {(filters.similarityThreshold * 100).toFixed(0)}%
                </label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={filters.similarityThreshold}
                  onChange={(e) => handleFilterChange({ similarityThreshold: parseFloat(e.target.value) })}
                  className="w-full mt-1"
                />
              </div>
              
              <div>
                <label className="text-sm font-medium">
                  Confidence Threshold: {(filters.confidenceThreshold * 100).toFixed(0)}%
                </label>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={filters.confidenceThreshold}
                  onChange={(e) => handleFilterChange({ confidenceThreshold: parseFloat(e.target.value) })}
                  className="w-full mt-1"
                />
              </div>
            </div>
          </div>
        )}
      </CardHeader>

      <CardContent className="flex-1 overflow-auto p-0">
        {/* Results Count */}
        {query && (
          <div className="px-4 py-2 border-b bg-muted/30">
            <span className="text-sm text-muted-foreground">
              {searchResults.length} result{searchResults.length !== 1 ? 's' : ''} found
            </span>
          </div>
        )}

        {/* Search Results */}
        <div className="divide-y">
          {searchResults.map((result) => (
            <div
              key={result.id}
              className="p-4 hover:bg-muted/50 cursor-pointer transition-colors"
              onClick={() => onResultSelect?.(result)}
            >
              <div className="flex items-start gap-3">
                <div className={`mt-1 ${getResultTypeColor(result)}`}>
                  {getResultIcon(result)}
                </div>
                
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <h4 className="font-medium truncate">{result.title}</h4>
                    {result.subtitle && (
                      <Badge variant="outline" className="text-xs">
                        {result.subtitle}
                      </Badge>
                    )}
                  </div>
                  
                  <p className="text-sm text-muted-foreground mb-2">{result.content}</p>
                  
                  <div className="flex items-center gap-4 text-xs text-muted-foreground">
                    <div className="flex items-center gap-1">
                      <MapPin className="w-3 h-3" />
                      <span>{result.location}</span>
                    </div>
                    
                    {result.similarity && (
                      <div className="flex items-center gap-1">
                        <Hash className="w-3 h-3" />
                        <span>{(result.similarity * 100).toFixed(1)}% similarity</span>
                      </div>
                    )}
                    
                    {result.confidence && (
                      <div className="flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        <span>{(result.confidence * 100).toFixed(1)}% confidence</span>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          ))}
          
          {query && searchResults.length === 0 && (
            <div className="p-8 text-center text-muted-foreground">
              <Search className="w-12 h-12 mx-auto mb-4 opacity-50" />
              <p>No results found for "{query}"</p>
              <p className="text-sm">Try adjusting your search terms or filters</p>
            </div>
          )}
          
          {!query && (
            <div className="p-8 text-center text-muted-foreground">
              <Search className="w-12 h-12 mx-auto mb-4 opacity-50" />
              <p>Enter a search term to find functions, files, and changes</p>
              <p className="text-sm">Use advanced filters for more precise results</p>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
