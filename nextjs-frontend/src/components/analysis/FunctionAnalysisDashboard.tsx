'use client';

import React, { useState, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Badge } from '@/components/ui/Badge';
import { 
  Search, 
  Filter, 
  TrendingUp, 
  TrendingDown, 
  ArrowRight,
  GitBranch,
  FileText,
  BarChart3,
  PieChart,
  Activity,
  Zap,
  Target
} from 'lucide-react';
import { GraphMatchResult, FunctionMatch, MatchType } from '@/types';

interface FunctionAnalysisDashboardProps {
  data?: GraphMatchResult;
  onFunctionSelect?: (functionId: string) => void;
}

interface AnalysisMetrics {
  totalFunctions: number;
  addedFunctions: number;
  removedFunctions: number;
  modifiedFunctions: number;
  movedFunctions: number;
  renamedFunctions: number;
  overallSimilarity: number;
  highConfidenceMatches: number;
  lowConfidenceMatches: number;
  complexityChanges: number;
}

export function FunctionAnalysisDashboard({ data, onFunctionSelect }: FunctionAnalysisDashboardProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedMatchType, setSelectedMatchType] = useState<MatchType | 'all'>('all');
  const [sortBy, setSortBy] = useState<'similarity' | 'confidence' | 'name'>('similarity');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  // Calculate comprehensive metrics
  const metrics: AnalysisMetrics = useMemo(() => {
    if (!data) {
      return {
        totalFunctions: 0,
        addedFunctions: 0,
        removedFunctions: 0,
        modifiedFunctions: 0,
        movedFunctions: 0,
        renamedFunctions: 0,
        overallSimilarity: 0,
        highConfidenceMatches: 0,
        lowConfidenceMatches: 0,
        complexityChanges: 0
      };
    }

    const highConfidenceMatches = data.matches.filter(m => m.confidence > 0.8).length;
    const lowConfidenceMatches = data.matches.filter(m => m.confidence <= 0.5).length;

    return {
      totalFunctions: data.matches.length + data.additions.length + data.deletions.length,
      addedFunctions: data.additions.length,
      removedFunctions: data.deletions.length,
      modifiedFunctions: data.matches.length,
      movedFunctions: data.moves.length,
      renamedFunctions: data.renames.length,
      overallSimilarity: data.overall_similarity,
      highConfidenceMatches,
      lowConfidenceMatches,
      complexityChanges: data.dependency_changes.length
    };
  }, [data]);

  // Filter and sort matches
  const filteredMatches = useMemo(() => {
    if (!data) return [];

    let matches = data.matches.filter(match => {
      // Filter by search query
      if (searchQuery) {
        const query = searchQuery.toLowerCase();
        return match.source_id.toLowerCase().includes(query) ||
               match.target_id.toLowerCase().includes(query);
      }
      return true;
    });

    // Filter by match type
    if (selectedMatchType !== 'all') {
      matches = matches.filter(match => match.match_type === selectedMatchType);
    }

    // Sort matches
    matches.sort((a, b) => {
      let comparison = 0;
      
      switch (sortBy) {
        case 'similarity':
          comparison = a.similarity.overall_similarity - b.similarity.overall_similarity;
          break;
        case 'confidence':
          comparison = a.confidence - b.confidence;
          break;
        case 'name':
          comparison = a.source_id.localeCompare(b.source_id);
          break;
      }
      
      return sortOrder === 'desc' ? -comparison : comparison;
    });

    return matches;
  }, [data, searchQuery, selectedMatchType, sortBy, sortOrder]);

  // Get match type color
  const getMatchTypeColor = (matchType: MatchType) => {
    switch (matchType) {
      case 'Exact': return 'bg-green-100 text-green-800 border-green-200';
      case 'Similar': return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'Renamed': return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'Moved': return 'bg-purple-100 text-purple-800 border-purple-200';
      case 'MovedAndRenamed': return 'bg-orange-100 text-orange-800 border-orange-200';
      case 'Refactored': return 'bg-red-100 text-red-800 border-red-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  // Get confidence level
  const getConfidenceLevel = (confidence: number) => {
    if (confidence > 0.8) return { label: 'High', color: 'text-green-600' };
    if (confidence > 0.5) return { label: 'Medium', color: 'text-yellow-600' };
    return { label: 'Low', color: 'text-red-600' };
  };

  return (
    <div className="space-y-6">
      {/* Metrics Overview */}
      <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-blue-600" />
              <div>
                <div className="text-2xl font-bold">{metrics.totalFunctions}</div>
                <div className="text-xs text-muted-foreground">Total Functions</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <TrendingUp className="w-4 h-4 text-green-600" />
              <div>
                <div className="text-2xl font-bold text-green-600">{metrics.addedFunctions}</div>
                <div className="text-xs text-muted-foreground">Added</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <TrendingDown className="w-4 h-4 text-red-600" />
              <div>
                <div className="text-2xl font-bold text-red-600">{metrics.removedFunctions}</div>
                <div className="text-xs text-muted-foreground">Removed</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <FileText className="w-4 h-4 text-yellow-600" />
              <div>
                <div className="text-2xl font-bold text-yellow-600">{metrics.modifiedFunctions}</div>
                <div className="text-xs text-muted-foreground">Modified</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <ArrowRight className="w-4 h-4 text-purple-600" />
              <div>
                <div className="text-2xl font-bold text-purple-600">{metrics.movedFunctions}</div>
                <div className="text-xs text-muted-foreground">Moved</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <GitBranch className="w-4 h-4 text-orange-600" />
              <div>
                <div className="text-2xl font-bold text-orange-600">{metrics.renamedFunctions}</div>
                <div className="text-xs text-muted-foreground">Renamed</div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-2">
              <Target className="w-4 h-4 text-blue-600" />
              <div>
                <div className="text-2xl font-bold text-blue-600">
                  {(metrics.overallSimilarity * 100).toFixed(1)}%
                </div>
                <div className="text-xs text-muted-foreground">Similarity</div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Confidence Distribution */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="w-5 h-5" />
            Confidence Distribution
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-3 gap-4">
            <div className="text-center">
              <div className="text-3xl font-bold text-green-600">{metrics.highConfidenceMatches}</div>
              <div className="text-sm text-muted-foreground">High Confidence (>80%)</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-yellow-600">
                {metrics.modifiedFunctions - metrics.highConfidenceMatches - metrics.lowConfidenceMatches}
              </div>
              <div className="text-sm text-muted-foreground">Medium Confidence (50-80%)</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-red-600">{metrics.lowConfidenceMatches}</div>
              <div className="text-sm text-muted-foreground">Low Confidence (<50%)</div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Function Matches Analysis */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <Zap className="w-5 h-5" />
              Function Matches Analysis
            </CardTitle>
            <div className="flex items-center gap-2">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                <Input
                  placeholder="Search functions..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-10 w-64"
                />
              </div>
              <select
                value={selectedMatchType}
                onChange={(e) => setSelectedMatchType(e.target.value as MatchType | 'all')}
                className="px-3 py-2 border rounded-md text-sm"
              >
                <option value="all">All Types</option>
                <option value="Exact">Exact</option>
                <option value="Similar">Similar</option>
                <option value="Renamed">Renamed</option>
                <option value="Moved">Moved</option>
                <option value="MovedAndRenamed">Moved & Renamed</option>
                <option value="Refactored">Refactored</option>
              </select>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-3 max-h-96 overflow-y-auto">
            {filteredMatches.map((match, index) => {
              const confidenceLevel = getConfidenceLevel(match.confidence);
              
              return (
                <div 
                  key={index}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 cursor-pointer transition-colors"
                  onClick={() => onFunctionSelect?.(match.source_id)}
                >
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-2">
                      <span className="font-medium truncate">{match.source_id}</span>
                      <ArrowRight className="w-4 h-4 text-muted-foreground flex-shrink-0" />
                      <span className="font-medium truncate">{match.target_id}</span>
                    </div>
                    
                    <div className="flex items-center gap-3 text-sm">
                      <Badge className={getMatchTypeColor(match.match_type)}>
                        {match.match_type}
                      </Badge>
                      
                      <span className="text-muted-foreground">
                        Similarity: {(match.similarity.overall_similarity * 100).toFixed(1)}%
                      </span>
                      
                      <span className={`font-medium ${confidenceLevel.color}`}>
                        {confidenceLevel.label} Confidence ({(match.confidence * 100).toFixed(1)}%)
                      </span>
                    </div>
                  </div>
                  
                  <div className="flex items-center gap-2">
                    <div className="text-right text-sm">
                      <div className="font-medium">
                        {(match.similarity.overall_similarity * 100).toFixed(1)}%
                      </div>
                      <div className="text-muted-foreground">Similarity</div>
                    </div>
                  </div>
                </div>
              );
            })}
            
            {filteredMatches.length === 0 && (
              <div className="text-center py-8 text-muted-foreground">
                <FileText className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No function matches found</p>
                {searchQuery && <p className="text-sm">Try adjusting your search query</p>}
              </div>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
