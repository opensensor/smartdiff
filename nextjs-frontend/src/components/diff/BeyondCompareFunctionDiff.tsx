'use client';

import React, { useState, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/Dialog';
import {
  ArrowRight,
  ArrowLeft,
  ArrowUpDown,
  Code,
  GitCompare,
  Eye,
  X,
  ChevronRight,
  ChevronDown,
  Search,
  Filter
} from 'lucide-react';
import { FunctionMatch, FunctionInfo } from '@/services/comparisonService';

interface FunctionItem {
  id: string;
  name: string;
  signature: string;
  content: string;
  lineCount: number;
  complexity?: number;
  filePath: string;
  startLine?: number;
  endLine?: number;
}

interface FunctionPair {
  id: string;
  sourceFunction?: FunctionItem;
  targetFunction?: FunctionItem;
  matchType: 'identical' | 'similar' | 'renamed' | 'moved' | 'added' | 'deleted';
  similarity: number;
  changes?: {
    signatureChanged: boolean;
    bodyChanged: boolean;
    moved: boolean;
    renamed: boolean;
  };
}

interface BeyondCompareFunctionDiffProps {
  functionMatches: FunctionMatch[];
  onFunctionSelect?: (pair: FunctionPair) => void;
}

export function BeyondCompareFunctionDiff({ functionMatches, onFunctionSelect }: BeyondCompareFunctionDiffProps) {
  const [selectedPair, setSelectedPair] = useState<FunctionPair | null>(null);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [expandedFiles, setExpandedFiles] = useState<Set<string>>(new Set());
  const [filterType, setFilterType] = useState<string>('all');
  const [searchTerm, setSearchTerm] = useState('');

  // Convert function matches to function pairs and apply filtering
  const { functionPairs, fileGroups, filteredPairs } = useMemo(() => {
    const pairs: FunctionPair[] = [];
    const groups = new Map<string, FunctionPair[]>();

    functionMatches.forEach((match, index) => {
      const sourceFunc = match.sourceFunction ? {
        id: `source-${match.sourceFunction.name}-${index}`,
        name: match.sourceFunction.name,
        signature: match.sourceFunction.signature || `${match.sourceFunction.name}()`,
        content: match.sourceFunction.content || '',
        lineCount: match.sourceFunction.content?.split('\n').length || 0,
        complexity: match.sourceFunction.complexity,
        filePath: match.sourceFunction.filePath,
        startLine: match.sourceFunction.startLine,
        endLine: match.sourceFunction.endLine
      } : undefined;

      const targetFunc = match.targetFunction ? {
        id: `target-${match.targetFunction.name}-${index}`,
        name: match.targetFunction.name,
        signature: match.targetFunction.signature || `${match.targetFunction.name}()`,
        content: match.targetFunction.content || '',
        lineCount: match.targetFunction.content?.split('\n').length || 0,
        complexity: match.targetFunction.complexity,
        filePath: match.targetFunction.filePath,
        startLine: match.targetFunction.startLine,
        endLine: match.targetFunction.endLine
      } : undefined;

      const pair: FunctionPair = {
        id: `pair-${index}`,
        sourceFunction: sourceFunc,
        targetFunction: targetFunc,
        matchType: match.type,
        similarity: match.similarity,
        changes: match.changes
      };

      pairs.push(pair);
    });

    // Apply filtering
    let filtered = pairs;

    // Filter by search term
    if (searchTerm) {
      filtered = filtered.filter(pair => {
        const sourceName = pair.sourceFunction?.name.toLowerCase() || '';
        const targetName = pair.targetFunction?.name.toLowerCase() || '';
        const sourceFile = pair.sourceFunction?.filePath.toLowerCase() || '';
        const targetFile = pair.targetFunction?.filePath.toLowerCase() || '';
        const term = searchTerm.toLowerCase();

        return sourceName.includes(term) || targetName.includes(term) ||
               sourceFile.includes(term) || targetFile.includes(term);
      });
    }

    // Filter by change type
    if (filterType !== 'all') {
      filtered = filtered.filter(pair => {
        switch (filterType) {
          case 'added': return pair.matchType === 'added';
          case 'deleted': return pair.matchType === 'deleted';
          case 'modified': return pair.matchType === 'similar' || pair.changes?.bodyChanged || pair.changes?.signatureChanged;
          case 'moved': return pair.matchType === 'moved' || pair.changes?.moved;
          case 'renamed': return pair.matchType === 'renamed' || pair.changes?.renamed;
          case 'unchanged': return pair.matchType === 'identical';
          default: return true;
        }
      });
    }

    // Group filtered pairs by file
    const filteredGroups = new Map<string, FunctionPair[]>();
    filtered.forEach(pair => {
      const fileName = pair.sourceFunction?.filePath || pair.targetFunction?.filePath || 'Unknown';
      const fileKey = fileName.split('/').pop() || fileName;

      if (!filteredGroups.has(fileKey)) {
        filteredGroups.set(fileKey, []);
      }
      filteredGroups.get(fileKey)!.push(pair);
    });

    return { functionPairs: pairs, fileGroups: filteredGroups, filteredPairs: filtered };
  }, [functionMatches, searchTerm, filterType]);

  // Auto-expand first few files for better UX
  React.useEffect(() => {
    const fileNames = Array.from(fileGroups.keys()).slice(0, 3); // Expand first 3 files
    setExpandedFiles(new Set(fileNames));
  }, [fileGroups]);

  const toggleFileExpansion = (fileName: string) => {
    const newExpanded = new Set(expandedFiles);
    if (newExpanded.has(fileName)) {
      newExpanded.delete(fileName);
    } else {
      newExpanded.add(fileName);
    }
    setExpandedFiles(newExpanded);
  };

  const handleFunctionClick = (pair: FunctionPair) => {
    setSelectedPair(pair);
    setShowDetailModal(true);
    onFunctionSelect?.(pair);
  };

  const getMatchTypeColor = (matchType: string) => {
    switch (matchType) {
      case 'identical': return 'bg-gray-100 text-gray-800';
      case 'similar': return 'bg-blue-100 text-blue-800';
      case 'renamed': return 'bg-purple-100 text-purple-800';
      case 'moved': return 'bg-indigo-100 text-indigo-800';
      case 'added': return 'bg-green-100 text-green-800';
      case 'deleted': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getConnectionIcon = (pair: FunctionPair) => {
    if (!pair.sourceFunction && pair.targetFunction) {
      return <ArrowRight className="w-4 h-4 text-green-600" />;
    } else if (pair.sourceFunction && !pair.targetFunction) {
      return <X className="w-4 h-4 text-red-600" />;
    } else if (pair.changes?.moved) {
      return <ArrowUpDown className="w-4 h-4 text-purple-600" />;
    } else if (pair.changes?.renamed) {
      return <ArrowRight className="w-4 h-4 text-blue-600" />;
    } else {
      return <ArrowRight className="w-4 h-4 text-gray-600" />;
    }
  };

  return (
    <div className="h-full flex flex-col">
      <Card className="flex-1">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <GitCompare className="w-5 h-5" />
              Function Comparison - Beyond Compare Style
            </CardTitle>

            <div className="flex items-center gap-2">
              <Badge variant="outline" className="bg-green-50 text-green-700">
                +{functionPairs.filter(p => p.matchType === 'added').length} Added
              </Badge>
              <Badge variant="outline" className="bg-red-50 text-red-700">
                -{functionPairs.filter(p => p.matchType === 'deleted').length} Deleted
              </Badge>
              <Badge variant="outline" className="bg-yellow-50 text-yellow-700">
                ~{functionPairs.filter(p => p.matchType === 'similar' || p.changes?.bodyChanged).length} Modified
              </Badge>
              <Badge variant="outline" className="bg-gray-50 text-gray-700">
                ={functionPairs.filter(p => p.matchType === 'identical').length} Unchanged
              </Badge>
            </div>
          </div>

          {/* Filter Controls */}
          <div className="flex items-center gap-4 mt-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search functions..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="flex items-center gap-2">
              <Filter className="w-4 h-4 text-gray-500" />
              <select
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Changes ({functionPairs.length})</option>
                <option value="added">Added ({functionPairs.filter(p => p.matchType === 'added').length})</option>
                <option value="deleted">Deleted ({functionPairs.filter(p => p.matchType === 'deleted').length})</option>
                <option value="modified">Modified ({functionPairs.filter(p => p.matchType === 'similar' || p.changes?.bodyChanged).length})</option>
                <option value="moved">Moved ({functionPairs.filter(p => p.matchType === 'moved' || p.changes?.moved).length})</option>
                <option value="renamed">Renamed ({functionPairs.filter(p => p.matchType === 'renamed' || p.changes?.renamed).length})</option>
                <option value="unchanged">Unchanged ({functionPairs.filter(p => p.matchType === 'identical').length})</option>
              </select>
            </div>

            <div className="text-sm text-gray-600">
              Showing {filteredPairs.length} of {functionPairs.length} functions
            </div>

            {/* Quick Filter Buttons */}
            <div className="flex items-center gap-1">
              <Button
                variant={filterType === 'added' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'added' ? 'all' : 'added')}
                className="text-xs"
              >
                +Added
              </Button>
              <Button
                variant={filterType === 'deleted' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'deleted' ? 'all' : 'deleted')}
                className="text-xs"
              >
                -Deleted
              </Button>
              <Button
                variant={filterType === 'modified' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'modified' ? 'all' : 'modified')}
                className="text-xs"
              >
                ~Modified
              </Button>
            </div>
          </div>
        </CardHeader>

        <CardContent className="p-0">
          <div className="grid grid-cols-3 h-[600px]">
            {/* Source Functions */}
            <div className="border-r bg-red-50">
              <div className="p-3 bg-red-100 border-b font-medium text-red-800">
                Source Functions
              </div>
              <div className="overflow-auto h-full">
                {Array.from(fileGroups.entries()).map(([fileName, pairs]) => (
                  <div key={fileName} className="border-b">
                    <div 
                      className="p-2 bg-gray-50 cursor-pointer hover:bg-gray-100 flex items-center gap-2"
                      onClick={() => toggleFileExpansion(fileName)}
                    >
                      {expandedFiles.has(fileName) ? 
                        <ChevronDown className="w-4 h-4" /> : 
                        <ChevronRight className="w-4 h-4" />
                      }
                      <span className="text-sm font-medium">{fileName}</span>
                      <Badge variant="outline" className="text-xs">
                        {pairs.length}
                      </Badge>
                    </div>
                    
                    {expandedFiles.has(fileName) && (
                      <div className="space-y-1 p-2">
                        {pairs.map(pair => pair.sourceFunction && (
                          <div
                            key={pair.sourceFunction.id}
                            className="p-2 rounded cursor-pointer hover:bg-red-100 border border-red-200"
                            onClick={() => handleFunctionClick(pair)}
                          >
                            <div className="font-mono text-sm font-medium">
                              {pair.sourceFunction.name}
                            </div>
                            <div className="text-xs text-gray-600 truncate">
                              {pair.sourceFunction.signature}
                            </div>
                            <div className="flex items-center gap-2 mt-1">
                              <Badge variant="outline" className="text-xs">
                                {pair.sourceFunction.lineCount} lines
                              </Badge>
                              {pair.sourceFunction.complexity && (
                                <Badge variant="outline" className="text-xs">
                                  C: {pair.sourceFunction.complexity}
                                </Badge>
                              )}
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Connection/Mapping Column */}
            <div className="bg-gray-50 border-r">
              <div className="p-3 bg-gray-100 border-b font-medium text-gray-800">
                Mapping
              </div>
              <div className="overflow-auto h-full">
                {Array.from(fileGroups.entries()).map(([fileName, pairs]) => (
                  <div key={fileName} className="border-b">
                    <div className="p-2 bg-gray-50">
                      <span className="text-sm font-medium">{fileName}</span>
                    </div>
                    
                    {expandedFiles.has(fileName) && (
                      <div className="space-y-1 p-2">
                        {pairs.map(pair => (
                          <div
                            key={pair.id}
                            className="p-2 rounded cursor-pointer hover:bg-gray-100 border border-gray-200 flex items-center justify-center"
                            onClick={() => handleFunctionClick(pair)}
                          >
                            <div className="flex items-center gap-2">
                              {getConnectionIcon(pair)}
                              <Badge className={getMatchTypeColor(pair.matchType)}>
                                {pair.matchType}
                              </Badge>
                              {pair.similarity < 1 && (
                                <span className="text-xs text-gray-600">
                                  {(pair.similarity * 100).toFixed(0)}%
                                </span>
                              )}
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Target Functions */}
            <div className="bg-green-50">
              <div className="p-3 bg-green-100 border-b font-medium text-green-800">
                Target Functions
              </div>
              <div className="overflow-auto h-full">
                {Array.from(fileGroups.entries()).map(([fileName, pairs]) => (
                  <div key={fileName} className="border-b">
                    <div className="p-2 bg-gray-50">
                      <span className="text-sm font-medium">{fileName}</span>
                    </div>
                    
                    {expandedFiles.has(fileName) && (
                      <div className="space-y-1 p-2">
                        {pairs.map(pair => pair.targetFunction && (
                          <div
                            key={pair.targetFunction.id}
                            className="p-2 rounded cursor-pointer hover:bg-green-100 border border-green-200"
                            onClick={() => handleFunctionClick(pair)}
                          >
                            <div className="font-mono text-sm font-medium">
                              {pair.targetFunction.name}
                            </div>
                            <div className="text-xs text-gray-600 truncate">
                              {pair.targetFunction.signature}
                            </div>
                            <div className="flex items-center gap-2 mt-1">
                              <Badge variant="outline" className="text-xs">
                                {pair.targetFunction.lineCount} lines
                              </Badge>
                              {pair.targetFunction.complexity && (
                                <Badge variant="outline" className="text-xs">
                                  C: {pair.targetFunction.complexity}
                                </Badge>
                              )}
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Detailed Function Diff Modal */}
      <Dialog open={showDetailModal} onOpenChange={setShowDetailModal}>
        <DialogContent className="max-w-7xl max-h-[95vh] overflow-auto">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <Code className="w-5 h-5" />
              Function Diff: {selectedPair?.sourceFunction?.name || selectedPair?.targetFunction?.name}
            </DialogTitle>
          </DialogHeader>
          
          {selectedPair && (
            <div className="space-y-4">
              {/* Function metadata */}
              <div className="grid grid-cols-2 gap-4 p-4 bg-gray-50 rounded-lg">
                <div>
                  <h4 className="font-medium mb-2 text-red-700">Source Function</h4>
                  {selectedPair.sourceFunction ? (
                    <div className="space-y-1 text-sm">
                      <p><strong>Name:</strong> {selectedPair.sourceFunction.name}</p>
                      <p><strong>Signature:</strong> <code className="text-xs bg-white px-1 rounded">{selectedPair.sourceFunction.signature}</code></p>
                      <p><strong>Lines:</strong> {selectedPair.sourceFunction.lineCount}</p>
                      <p><strong>File:</strong> {selectedPair.sourceFunction.filePath.split('/').pop()}</p>
                      {selectedPair.sourceFunction.complexity && (
                        <p><strong>Complexity:</strong> {selectedPair.sourceFunction.complexity}</p>
                      )}
                    </div>
                  ) : (
                    <p className="text-gray-500 italic">Function not present in source</p>
                  )}
                </div>
                
                <div>
                  <h4 className="font-medium mb-2 text-green-700">Target Function</h4>
                  {selectedPair.targetFunction ? (
                    <div className="space-y-1 text-sm">
                      <p><strong>Name:</strong> {selectedPair.targetFunction.name}</p>
                      <p><strong>Signature:</strong> <code className="text-xs bg-white px-1 rounded">{selectedPair.targetFunction.signature}</code></p>
                      <p><strong>Lines:</strong> {selectedPair.targetFunction.lineCount}</p>
                      <p><strong>File:</strong> {selectedPair.targetFunction.filePath.split('/').pop()}</p>
                      {selectedPair.targetFunction.complexity && (
                        <p><strong>Complexity:</strong> {selectedPair.targetFunction.complexity}</p>
                      )}
                    </div>
                  ) : (
                    <p className="text-gray-500 italic">Function not present in target</p>
                  )}
                </div>
              </div>

              {/* Side-by-side function content */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <h4 className="font-medium mb-2 text-red-700">Source Code</h4>
                  <div className="bg-red-50 rounded-lg p-4 max-h-96 overflow-auto">
                    <pre className="font-mono text-sm whitespace-pre-wrap">
                      {selectedPair.sourceFunction?.content || 'No source content'}
                    </pre>
                  </div>
                </div>
                
                <div>
                  <h4 className="font-medium mb-2 text-green-700">Target Code</h4>
                  <div className="bg-green-50 rounded-lg p-4 max-h-96 overflow-auto">
                    <pre className="font-mono text-sm whitespace-pre-wrap">
                      {selectedPair.targetFunction?.content || 'No target content'}
                    </pre>
                  </div>
                </div>
              </div>

              {/* Change summary */}
              <div className="p-4 bg-blue-50 rounded-lg">
                <h4 className="font-medium mb-2">Change Summary</h4>
                <div className="flex items-center gap-4 text-sm">
                  <Badge className={getMatchTypeColor(selectedPair.matchType)}>
                    {selectedPair.matchType}
                  </Badge>
                  <span>Similarity: {(selectedPair.similarity * 100).toFixed(1)}%</span>
                  {selectedPair.changes && (
                    <div className="flex gap-2">
                      {selectedPair.changes.signatureChanged && <Badge variant="outline">Signature Changed</Badge>}
                      {selectedPair.changes.bodyChanged && <Badge variant="outline">Body Changed</Badge>}
                      {selectedPair.changes.moved && <Badge variant="outline">Moved</Badge>}
                      {selectedPair.changes.renamed && <Badge variant="outline">Renamed</Badge>}
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
