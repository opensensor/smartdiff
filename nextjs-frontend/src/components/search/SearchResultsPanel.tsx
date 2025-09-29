'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { 
  X, 
  ExternalLink, 
  Copy, 
  Download,
  Filter,
  SortAsc,
  SortDesc,
  Grid,
  List,
  Eye
} from 'lucide-react';

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

interface SearchResultsPanelProps {
  results: SearchResult[];
  isVisible: boolean;
  onClose: () => void;
  onResultClick: (result: SearchResult) => void;
  onExportResults?: () => void;
}

export function SearchResultsPanel({ 
  results, 
  isVisible, 
  onClose, 
  onResultClick,
  onExportResults 
}: SearchResultsPanelProps) {
  const [sortBy, setSortBy] = useState<'relevance' | 'similarity' | 'confidence' | 'name'>('relevance');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [viewMode, setViewMode] = useState<'list' | 'grid'>('list');
  const [selectedResults, setSelectedResults] = useState<Set<string>>(new Set());

  if (!isVisible) return null;

  // Sort results
  const sortedResults = [...results].sort((a, b) => {
    let comparison = 0;
    
    switch (sortBy) {
      case 'similarity':
        comparison = (a.similarity || 0) - (b.similarity || 0);
        break;
      case 'confidence':
        comparison = (a.confidence || 0) - (b.confidence || 0);
        break;
      case 'name':
        comparison = a.title.localeCompare(b.title);
        break;
      case 'relevance':
      default:
        // Relevance based on type and similarity
        const getRelevanceScore = (result: SearchResult) => {
          let score = 0;
          if (result.type === 'match') score += 3;
          else if (result.type === 'function') score += 2;
          else score += 1;
          
          if (result.similarity) score += result.similarity * 2;
          if (result.confidence) score += result.confidence * 2;
          
          return score;
        };
        comparison = getRelevanceScore(a) - getRelevanceScore(b);
        break;
    }
    
    return sortOrder === 'desc' ? -comparison : comparison;
  });

  // Group results by type
  const groupedResults = sortedResults.reduce((groups, result) => {
    const type = result.type;
    if (!groups[type]) groups[type] = [];
    groups[type].push(result);
    return groups;
  }, {} as Record<string, SearchResult[]>);

  const handleSelectResult = (resultId: string) => {
    const newSelection = new Set(selectedResults);
    if (newSelection.has(resultId)) {
      newSelection.delete(resultId);
    } else {
      newSelection.add(resultId);
    }
    setSelectedResults(newSelection);
  };

  const handleSelectAll = () => {
    if (selectedResults.size === results.length) {
      setSelectedResults(new Set());
    } else {
      setSelectedResults(new Set(results.map(r => r.id)));
    }
  };

  const copyResultToClipboard = (result: SearchResult) => {
    const text = `${result.title}\n${result.content}\nLocation: ${result.location}`;
    navigator.clipboard.writeText(text);
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'function': return '🔧';
      case 'file': return '📄';
      case 'match': return '🔗';
      default: return '📋';
    }
  };

  const getTypeLabel = (type: string) => {
    switch (type) {
      case 'function': return 'Functions';
      case 'file': return 'Files';
      case 'match': return 'Matches';
      default: return 'Other';
    }
  };

  return (
    <div className="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm">
      <div className="absolute right-0 top-0 h-full w-full max-w-2xl bg-background border-l shadow-xl">
        <Card className="h-full flex flex-col rounded-none border-0">
          <CardHeader className="border-b">
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2">
                <Eye className="w-5 h-5" />
                Search Results ({results.length})
              </CardTitle>
              <Button variant="ghost" size="sm" onClick={onClose}>
                <X className="w-4 h-4" />
              </Button>
            </div>
            
            {/* Controls */}
            <div className="flex items-center justify-between gap-4">
              <div className="flex items-center gap-2">
                <select
                  value={sortBy}
                  onChange={(e) => setSortBy(e.target.value as any)}
                  className="px-3 py-1 border rounded text-sm"
                >
                  <option value="relevance">Relevance</option>
                  <option value="similarity">Similarity</option>
                  <option value="confidence">Confidence</option>
                  <option value="name">Name</option>
                </select>
                
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
                >
                  {sortOrder === 'asc' ? <SortAsc className="w-4 h-4" /> : <SortDesc className="w-4 h-4" />}
                </Button>
                
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setViewMode(viewMode === 'list' ? 'grid' : 'list')}
                >
                  {viewMode === 'list' ? <Grid className="w-4 h-4" /> : <List className="w-4 h-4" />}
                </Button>
              </div>
              
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={handleSelectAll}>
                  {selectedResults.size === results.length ? 'Deselect All' : 'Select All'}
                </Button>
                
                {onExportResults && (
                  <Button variant="outline" size="sm" onClick={onExportResults}>
                    <Download className="w-4 h-4 mr-1" />
                    Export
                  </Button>
                )}
              </div>
            </div>
          </CardHeader>

          <CardContent className="flex-1 overflow-auto p-0">
            {viewMode === 'list' ? (
              <div className="divide-y">
                {Object.entries(groupedResults).map(([type, typeResults]) => (
                  <div key={type}>
                    {/* Type Header */}
                    <div className="sticky top-0 bg-muted/80 backdrop-blur-sm px-4 py-2 border-b">
                      <div className="flex items-center gap-2">
                        <span className="text-lg">{getTypeIcon(type)}</span>
                        <span className="font-medium">{getTypeLabel(type)}</span>
                        <Badge variant="outline">{typeResults.length}</Badge>
                      </div>
                    </div>
                    
                    {/* Results */}
                    {typeResults.map((result) => (
                      <div
                        key={result.id}
                        className={`p-4 hover:bg-muted/50 cursor-pointer transition-colors border-l-2 ${
                          selectedResults.has(result.id) ? 'border-l-primary bg-primary/5' : 'border-l-transparent'
                        }`}
                        onClick={() => onResultClick(result)}
                      >
                        <div className="flex items-start gap-3">
                          <input
                            type="checkbox"
                            checked={selectedResults.has(result.id)}
                            onChange={() => handleSelectResult(result.id)}
                            onClick={(e) => e.stopPropagation()}
                            className="mt-1"
                          />
                          
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-1">
                              <h4 className="font-medium truncate">{result.title}</h4>
                              {result.subtitle && (
                                <Badge variant="outline" className="text-xs">
                                  {result.subtitle}
                                </Badge>
                              )}
                            </div>
                            
                            <p className="text-sm text-muted-foreground mb-2 line-clamp-2">
                              {result.content}
                            </p>
                            
                            <div className="flex items-center justify-between">
                              <div className="flex items-center gap-4 text-xs text-muted-foreground">
                                <span>📍 {result.location}</span>
                                
                                {result.similarity && (
                                  <span>🎯 {(result.similarity * 100).toFixed(1)}%</span>
                                )}
                                
                                {result.confidence && (
                                  <span>✅ {(result.confidence * 100).toFixed(1)}%</span>
                                )}
                              </div>
                              
                              <div className="flex items-center gap-1">
                                <Button
                                  variant="ghost"
                                  size="sm"
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    copyResultToClipboard(result);
                                  }}
                                  className="h-6 w-6 p-0"
                                >
                                  <Copy className="w-3 h-3" />
                                </Button>
                                
                                <Button
                                  variant="ghost"
                                  size="sm"
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    onResultClick(result);
                                  }}
                                  className="h-6 w-6 p-0"
                                >
                                  <ExternalLink className="w-3 h-3" />
                                </Button>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                ))}
              </div>
            ) : (
              /* Grid View */
              <div className="p-4">
                {Object.entries(groupedResults).map(([type, typeResults]) => (
                  <div key={type} className="mb-6">
                    <div className="flex items-center gap-2 mb-3">
                      <span className="text-lg">{getTypeIcon(type)}</span>
                      <span className="font-medium">{getTypeLabel(type)}</span>
                      <Badge variant="outline">{typeResults.length}</Badge>
                    </div>
                    
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                      {typeResults.map((result) => (
                        <Card
                          key={result.id}
                          className={`cursor-pointer hover:shadow-md transition-all ${
                            selectedResults.has(result.id) ? 'ring-2 ring-primary' : ''
                          }`}
                          onClick={() => onResultClick(result)}
                        >
                          <CardContent className="p-3">
                            <div className="flex items-start gap-2 mb-2">
                              <input
                                type="checkbox"
                                checked={selectedResults.has(result.id)}
                                onChange={() => handleSelectResult(result.id)}
                                onClick={(e) => e.stopPropagation()}
                                className="mt-1"
                              />
                              <div className="flex-1 min-w-0">
                                <h4 className="font-medium truncate text-sm">{result.title}</h4>
                                {result.subtitle && (
                                  <Badge variant="outline" className="text-xs mt-1">
                                    {result.subtitle}
                                  </Badge>
                                )}
                              </div>
                            </div>
                            
                            <p className="text-xs text-muted-foreground mb-2 line-clamp-2">
                              {result.content}
                            </p>
                            
                            <div className="flex items-center justify-between text-xs">
                              <span className="text-muted-foreground truncate">
                                📍 {result.location}
                              </span>
                              
                              {(result.similarity || result.confidence) && (
                                <div className="flex gap-2">
                                  {result.similarity && (
                                    <span>🎯 {(result.similarity * 100).toFixed(0)}%</span>
                                  )}
                                  {result.confidence && (
                                    <span>✅ {(result.confidence * 100).toFixed(0)}%</span>
                                  )}
                                </div>
                              )}
                            </div>
                          </CardContent>
                        </Card>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            )}
            
            {results.length === 0 && (
              <div className="flex items-center justify-center h-full text-muted-foreground">
                <div className="text-center">
                  <Eye className="w-12 h-12 mx-auto mb-4 opacity-50" />
                  <p>No search results to display</p>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
