'use client';

import React, { useState, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { Input } from '@/components/ui/Input';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/Dialog';
import {
  Search,
  Filter,
  ArrowUpDown,
  FileText,
  TrendingUp,
  TrendingDown,
  GitCompare,
  Code,
  X,
  ArrowLeft,
  ArrowRight,
  Download,
} from 'lucide-react';
import { FunctionMatch } from '@/services/comparisonService';

// Import the UnifiedDiffView component from BeyondCompareFunctionDiff
import { UnifiedDiffView } from './BeyondCompareFunctionDiff';

interface FunctionCentricDiffViewProps {
  functionMatches: FunctionMatch[];
  onFunctionSelect?: (match: FunctionMatch) => void;
}

export function FunctionCentricDiffView({
  functionMatches,
  onFunctionSelect,
}: FunctionCentricDiffViewProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [filterType, setFilterType] = useState<string>('all');
  const [sortBy, setSortBy] = useState<'magnitude' | 'name' | 'similarity'>('magnitude');
  const [selectedMatch, setSelectedMatch] = useState<FunctionMatch | null>(null);
  const [showDetailModal, setShowDetailModal] = useState(false);

  // Sort and filter functions by change magnitude (most changed first)
  const sortedFunctions = useMemo(() => {
    let filtered = functionMatches;

    // Apply search filter
    if (searchTerm) {
      const term = searchTerm.toLowerCase();
      filtered = filtered.filter(
        (match) =>
          match.sourceFunction?.name.toLowerCase().includes(term) ||
          match.targetFunction?.name.toLowerCase().includes(term) ||
          match.sourceFunction?.filePath.toLowerCase().includes(term) ||
          match.targetFunction?.filePath.toLowerCase().includes(term)
      );
    }

    // Apply type filter
    if (filterType !== 'all') {
      filtered = filtered.filter((match) => {
        switch (filterType) {
          case 'modified':
            return match.matchType === 'modified' || match.matchType === 'similar';
          case 'added':
            return match.matchType === 'added';
          case 'deleted':
            return match.matchType === 'deleted';
          case 'moved':
            return match.matchType === 'moved' || match.changes?.moved;
          case 'renamed':
            return match.matchType === 'renamed' || match.changes?.renamed;
          default:
            return true;
        }
      });
    }

    // Sort by selected criteria
    const sorted = [...filtered].sort((a, b) => {
      switch (sortBy) {
        case 'magnitude':
          // Sort by change magnitude (highest first)
          const magA = a.changeMagnitude ?? (1 - a.similarity);
          const magB = b.changeMagnitude ?? (1 - b.similarity);
          return magB - magA;
        case 'similarity':
          // Sort by similarity (lowest first = most changed)
          return a.similarity - b.similarity;
        case 'name':
          // Sort alphabetically by function name
          const nameA = a.sourceFunction?.name || a.targetFunction?.name || '';
          const nameB = b.sourceFunction?.name || b.targetFunction?.name || '';
          return nameA.localeCompare(nameB);
        default:
          return 0;
      }
    });

    return sorted;
  }, [functionMatches, searchTerm, filterType, sortBy]);

  const getChangeTypeColor = (matchType: string) => {
    switch (matchType) {
      case 'added':
        return 'bg-green-100 text-green-800 border-green-300';
      case 'deleted':
        return 'bg-red-100 text-red-800 border-red-300';
      case 'modified':
      case 'similar':
        return 'bg-yellow-100 text-yellow-800 border-yellow-300';
      case 'moved':
        return 'bg-blue-100 text-blue-800 border-blue-300';
      case 'renamed':
        return 'bg-purple-100 text-purple-800 border-purple-300';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-300';
    }
  };

  const getMagnitudeColor = (magnitude: number) => {
    if (magnitude >= 0.7) return 'text-red-600 font-bold';
    if (magnitude >= 0.4) return 'text-orange-600 font-semibold';
    if (magnitude >= 0.2) return 'text-yellow-600';
    return 'text-green-600';
  };

  const getFilePathDisplay = (filePath?: string) => {
    if (!filePath) return '';
    const parts = filePath.split('/');
    return parts.length > 2 ? `.../${parts.slice(-2).join('/')}` : filePath;
  };

  const exportToJSON = () => {
    const exportData = {
      exportDate: new Date().toISOString(),
      filter: filterType,
      sortBy: sortBy,
      searchTerm: searchTerm,
      totalFunctions: sortedFunctions.length,
      functions: sortedFunctions.map((match) => ({
        name: match.sourceFunction?.name || match.targetFunction?.name,
        matchType: match.matchType,
        similarity: match.similarity,
        changeMagnitude: match.changeMagnitude ?? (1 - match.similarity),
        sourceFunction: match.sourceFunction ? {
          name: match.sourceFunction.name,
          filePath: match.sourceFunction.filePath,
          startLine: match.sourceFunction.startLine,
          endLine: match.sourceFunction.endLine,
          signature: match.sourceFunction.signature,
          content: match.sourceFunction.content,
          lineCount: match.sourceFunction.lineCount,
        } : null,
        targetFunction: match.targetFunction ? {
          name: match.targetFunction.name,
          filePath: match.targetFunction.filePath,
          startLine: match.targetFunction.startLine,
          endLine: match.targetFunction.endLine,
          signature: match.targetFunction.signature,
          content: match.targetFunction.content,
          lineCount: match.targetFunction.lineCount,
        } : null,
        changes: match.changes,
      })),
    };

    const blob = new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `function-diff-export-${filterType}-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-4">
      {/* Controls */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex flex-col md:flex-row gap-4">
            {/* Search */}
            <div className="flex-1">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <Input
                  type="text"
                  placeholder="Search functions or files..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            </div>

            {/* Filter */}
            <div className="flex items-center gap-2">
              <Filter className="w-4 h-4 text-gray-500" />
              <select
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Changes ({functionMatches.length})</option>
                <option value="modified">
                  Modified ({functionMatches.filter((m) => m.matchType === 'modified' || m.matchType === 'similar').length})
                </option>
                <option value="added">Added ({functionMatches.filter((m) => m.matchType === 'added').length})</option>
                <option value="deleted">Deleted ({functionMatches.filter((m) => m.matchType === 'deleted').length})</option>
                <option value="moved">Moved ({functionMatches.filter((m) => m.matchType === 'moved' || m.changes?.moved).length})</option>
                <option value="renamed">Renamed ({functionMatches.filter((m) => m.matchType === 'renamed' || m.changes?.renamed).length})</option>
              </select>
            </div>

            {/* Sort */}
            <div className="flex items-center gap-2">
              <ArrowUpDown className="w-4 h-4 text-gray-500" />
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as any)}
                className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="magnitude">By Change Magnitude</option>
                <option value="similarity">By Similarity</option>
                <option value="name">By Name</option>
              </select>
            </div>

            {/* Export Button */}
            <Button
              onClick={exportToJSON}
              variant="outline"
              className="flex items-center gap-2"
              disabled={sortedFunctions.length === 0}
            >
              <Download className="w-4 h-4" />
              Export JSON ({sortedFunctions.length})
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Function List */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Code className="w-5 h-5" />
            Functions Sorted by Change Magnitude
            <span className="text-sm font-normal text-gray-500">
              ({sortedFunctions.length} {sortedFunctions.length === 1 ? 'function' : 'functions'})
            </span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2 max-h-[600px] overflow-y-auto">
            {sortedFunctions.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                No functions match the current filter
              </div>
            ) : (
              sortedFunctions.map((match, index) => {
                const functionName = match.sourceFunction?.name || match.targetFunction?.name || 'Unknown';
                const magnitude = match.changeMagnitude ?? (1 - match.similarity);
                const sourceFile = match.sourceFunction?.filePath;
                const targetFile = match.targetFunction?.filePath;

                return (
                  <div
                    key={index}
                    className="p-4 border rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
                    onClick={() => {
                      setSelectedMatch(match);
                      setShowDetailModal(true);
                      onFunctionSelect?.(match);
                    }}
                  >
                    <div className="flex items-start justify-between gap-4">
                      {/* Function Info */}
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 mb-2">
                          <span className="font-mono font-semibold text-lg truncate">{functionName}</span>
                          <Badge className={`${getChangeTypeColor(match.matchType)} text-xs`}>
                            {match.matchType}
                          </Badge>
                        </div>

                        {/* File paths */}
                        <div className="flex items-center gap-2 text-sm text-gray-600">
                          {sourceFile && (
                            <div className="flex items-center gap-1">
                              <FileText className="w-3 h-3" />
                              <span className="truncate" title={sourceFile}>
                                {getFilePathDisplay(sourceFile)}
                              </span>
                            </div>
                          )}
                          {sourceFile && targetFile && sourceFile !== targetFile && (
                            <span className="text-gray-400">→</span>
                          )}
                          {targetFile && sourceFile !== targetFile && (
                            <div className="flex items-center gap-1">
                              <FileText className="w-3 h-3" />
                              <span className="truncate" title={targetFile}>
                                {getFilePathDisplay(targetFile)}
                              </span>
                            </div>
                          )}
                        </div>

                        {/* Line numbers */}
                        {(match.sourceFunction || match.targetFunction) && (
                          <div className="flex items-center gap-4 mt-1 text-xs text-gray-500">
                            {match.sourceFunction && (
                              <span>
                                Source: L{match.sourceFunction.startLine}-{match.sourceFunction.endLine}
                              </span>
                            )}
                            {match.targetFunction && (
                              <span>
                                Target: L{match.targetFunction.startLine}-{match.targetFunction.endLine}
                              </span>
                            )}
                          </div>
                        )}
                      </div>

                      {/* Metrics */}
                      <div className="flex flex-col items-end gap-1 text-sm">
                        <div className={`font-mono ${getMagnitudeColor(magnitude)}`}>
                          {(magnitude * 100).toFixed(0)}% changed
                        </div>
                        <div className="text-gray-500">
                          {(match.similarity * 100).toFixed(0)}% similar
                        </div>
                      </div>
                    </div>
                  </div>
                );
              })
            )}
          </div>
        </CardContent>
      </Card>

      {/* Detailed Function Diff Modal */}
      <Dialog open={showDetailModal} onOpenChange={setShowDetailModal}>
        <DialogContent className="max-w-[95vw] w-[95vw] max-h-[95vh] overflow-hidden flex flex-col p-0">
          {selectedMatch && (
            <>
              {/* Header */}
              <div className="bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 text-white px-8 py-6 flex-shrink-0">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-4">
                      <div className="p-2 bg-white/10 rounded-lg backdrop-blur-sm">
                        <GitCompare className="w-6 h-6" />
                      </div>
                      <div>
                        <h2 className="text-2xl font-bold tracking-tight">
                          {selectedMatch.sourceFunction?.name || selectedMatch.targetFunction?.name}
                        </h2>
                        <p className="text-slate-300 text-sm mt-1">Function Comparison Analysis</p>
                      </div>
                    </div>

                    {/* Status Badges */}
                    <div className="flex items-center gap-3 flex-wrap mt-4">
                      <Badge className={`${getChangeTypeColor(selectedMatch.matchType)} px-3 py-1 text-sm font-medium`}>
                        {selectedMatch.matchType.toUpperCase()}
                      </Badge>
                      <div className="flex items-center gap-2 bg-white/10 px-3 py-1 rounded-full backdrop-blur-sm">
                        <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                        <span className="text-sm font-medium">
                          {(selectedMatch.similarity * 100).toFixed(1)}% Similarity
                        </span>
                      </div>
                    </div>
                  </div>

                  <Button
                    onClick={() => setShowDetailModal(false)}
                    variant="ghost"
                    size="sm"
                    className="text-white hover:bg-white/10 -mt-2 -mr-2"
                  >
                    <X className="w-5 h-5" />
                  </Button>
                </div>
              </div>

              {/* Content */}
              <div className="flex-1 overflow-auto p-8 bg-slate-50">
                <div className="space-y-6">
                  {/* Function Details */}
                  <div className="grid grid-cols-2 gap-6">
                    {/* Source Function */}
                    <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
                      <div className="bg-gradient-to-r from-slate-700 to-slate-600 px-5 py-3 flex items-center gap-2">
                        <ArrowLeft className="w-4 h-4 text-white" />
                        <h3 className="font-semibold text-white">Source Function</h3>
                      </div>
                      {selectedMatch.sourceFunction ? (
                        <div className="p-5 space-y-3">
                          <div className="flex items-start justify-between">
                            <div className="flex-1">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Function Name</div>
                              <div className="font-mono text-lg font-bold text-slate-900">{selectedMatch.sourceFunction.name}</div>
                            </div>
                          </div>
                          <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                            <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">File Path</div>
                            <code className="text-xs font-mono text-slate-800 break-all">
                              {selectedMatch.sourceFunction.filePath}
                            </code>
                          </div>
                          <div className="grid grid-cols-2 gap-3">
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Lines</div>
                              <div className="text-xl font-bold text-slate-900">
                                {selectedMatch.sourceFunction.startLine}-{selectedMatch.sourceFunction.endLine}
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : (
                        <div className="p-5 text-center text-slate-500">Function deleted</div>
                      )}
                    </div>

                    {/* Target Function */}
                    <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
                      <div className="bg-gradient-to-r from-blue-600 to-blue-500 px-5 py-3 flex items-center gap-2">
                        <ArrowRight className="w-4 h-4 text-white" />
                        <h3 className="font-semibold text-white">Target Function</h3>
                      </div>
                      {selectedMatch.targetFunction ? (
                        <div className="p-5 space-y-3">
                          <div className="flex items-start justify-between">
                            <div className="flex-1">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Function Name</div>
                              <div className="font-mono text-lg font-bold text-slate-900">{selectedMatch.targetFunction.name}</div>
                            </div>
                          </div>
                          <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                            <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">File Path</div>
                            <code className="text-xs font-mono text-slate-800 break-all">
                              {selectedMatch.targetFunction.filePath}
                            </code>
                          </div>
                          <div className="grid grid-cols-2 gap-3">
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Lines</div>
                              <div className="text-xl font-bold text-slate-900">
                                {selectedMatch.targetFunction.startLine}-{selectedMatch.targetFunction.endLine}
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : (
                        <div className="p-5 text-center text-slate-500">Function added</div>
                      )}
                    </div>
                  </div>

                  {/* Code Comparison */}
                  <UnifiedDiffView
                    sourceContent={selectedMatch.sourceFunction?.content || ''}
                    targetContent={selectedMatch.targetFunction?.content || ''}
                    sourceFilePath={selectedMatch.sourceFunction?.filePath || ''}
                    targetFilePath={selectedMatch.targetFunction?.filePath || ''}
                  />
                </div>
              </div>
            </>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}

