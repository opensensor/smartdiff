'use client';

import React, { useState, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { 
  SplitSquareHorizontal, 
  Square, 
  Search, 
  Filter,
  ChevronDown,
  ChevronRight,
  Code,
  GitCompare,
  Eye,
  EyeOff,
  Maximize2,
  Minimize2
} from 'lucide-react';
import { GraphMatchResult, FunctionMatch } from '@/types';

interface DiffLine {
  lineNumber: number;
  content: string;
  type: 'added' | 'removed' | 'modified' | 'unchanged';
  functionName?: string;
  similarity?: number;
}

interface DiffSection {
  functionName: string;
  sourceLines: DiffLine[];
  targetLines: DiffLine[];
  similarity: number;
  confidence: number;
  matchType: string;
}

interface InteractiveDiffViewerProps {
  data?: GraphMatchResult;
  onFunctionSelect?: (functionId: string) => void;
}

export function InteractiveDiffViewer({ data, onFunctionSelect }: InteractiveDiffViewerProps) {
  const [viewMode, setViewMode] = useState<'side-by-side' | 'unified'>('side-by-side');
  const [showOnlyChanges, setShowOnlyChanges] = useState(false);
  const [expandedSections, setExpandedSections] = useState<Set<string>>(new Set());
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedFunction, setSelectedFunction] = useState<string | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);

  // Generate mock diff data from the graph match result
  const diffSections = useMemo(() => {
    if (!data) return [];

    const sections: DiffSection[] = [];

    // Process function matches
    data.matches.forEach((match, index) => {
      const sourceLines: DiffLine[] = [
        { lineNumber: 1, content: `function ${match.source_id}() {`, type: 'unchanged' },
        { lineNumber: 2, content: '  // Original implementation', type: 'removed' },
        { lineNumber: 3, content: '  return originalValue;', type: 'removed' },
        { lineNumber: 4, content: '  // Updated implementation', type: 'added' },
        { lineNumber: 5, content: '  return updatedValue;', type: 'added' },
        { lineNumber: 6, content: '}', type: 'unchanged' },
      ];

      const targetLines: DiffLine[] = [
        { lineNumber: 1, content: `function ${match.target_id}() {`, type: 'unchanged' },
        { lineNumber: 2, content: '  // Updated implementation', type: 'added' },
        { lineNumber: 3, content: '  return updatedValue;', type: 'added' },
        { lineNumber: 4, content: '}', type: 'unchanged' },
      ];

      sections.push({
        functionName: match.source_id,
        sourceLines,
        targetLines,
        similarity: match.similarity.overall_similarity,
        confidence: match.confidence,
        matchType: match.match_type,
      });
    });

    // Process additions
    data.additions.forEach((funcId) => {
      const targetLines: DiffLine[] = [
        { lineNumber: 1, content: `function ${funcId}() {`, type: 'added' },
        { lineNumber: 2, content: '  // New function implementation', type: 'added' },
        { lineNumber: 3, content: '  return newValue;', type: 'added' },
        { lineNumber: 4, content: '}', type: 'added' },
      ];

      sections.push({
        functionName: funcId,
        sourceLines: [],
        targetLines,
        similarity: 0,
        confidence: 1,
        matchType: 'Addition',
      });
    });

    // Process deletions
    data.deletions.forEach((funcId) => {
      const sourceLines: DiffLine[] = [
        { lineNumber: 1, content: `function ${funcId}() {`, type: 'removed' },
        { lineNumber: 2, content: '  // Deleted function implementation', type: 'removed' },
        { lineNumber: 3, content: '  return deletedValue;', type: 'removed' },
        { lineNumber: 4, content: '}', type: 'removed' },
      ];

      sections.push({
        functionName: funcId,
        sourceLines,
        targetLines: [],
        similarity: 0,
        confidence: 1,
        matchType: 'Deletion',
      });
    });

    return sections;
  }, [data]);

  // Filter sections based on search term
  const filteredSections = useMemo(() => {
    if (!searchTerm) return diffSections;
    return diffSections.filter(section =>
      section.functionName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      section.sourceLines.some(line => line.content.toLowerCase().includes(searchTerm.toLowerCase())) ||
      section.targetLines.some(line => line.content.toLowerCase().includes(searchTerm.toLowerCase()))
    );
  }, [diffSections, searchTerm]);

  const toggleSection = (functionName: string) => {
    const newExpanded = new Set(expandedSections);
    if (newExpanded.has(functionName)) {
      newExpanded.delete(functionName);
    } else {
      newExpanded.add(functionName);
    }
    setExpandedSections(newExpanded);
  };

  const getLineTypeColor = (type: DiffLine['type']) => {
    switch (type) {
      case 'added': return 'bg-green-50 border-l-4 border-l-green-500 text-green-800';
      case 'removed': return 'bg-red-50 border-l-4 border-l-red-500 text-red-800';
      case 'modified': return 'bg-yellow-50 border-l-4 border-l-yellow-500 text-yellow-800';
      case 'unchanged': return 'bg-gray-50 text-gray-700';
      default: return 'bg-white text-gray-900';
    }
  };

  const getMatchTypeColor = (matchType: string) => {
    switch (matchType) {
      case 'Exact': return 'bg-green-100 text-green-800 border-green-200';
      case 'Similar': return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'Addition': return 'bg-green-100 text-green-800 border-green-200';
      case 'Deletion': return 'bg-red-100 text-red-800 border-red-200';
      default: return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const renderDiffLine = (line: DiffLine, side: 'source' | 'target') => (
    <div
      key={`${side}-${line.lineNumber}`}
      className={`flex items-start gap-3 px-3 py-1 font-mono text-sm ${getLineTypeColor(line.type)}`}
    >
      <span className="w-8 text-right text-gray-500 select-none">
        {line.lineNumber}
      </span>
      <span className="w-4 text-center text-gray-500 select-none">
        {line.type === 'added' && '+'}
        {line.type === 'removed' && '-'}
        {line.type === 'modified' && '~'}
      </span>
      <code className="flex-1">{line.content}</code>
    </div>
  );

  if (!data) {
    return (
      <Card>
        <CardContent className="text-center py-12 text-gray-500">
          <GitCompare className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p>No comparison data available</p>
          <p className="text-sm">Run a comparison to see interactive diff view</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className={`${isFullscreen ? 'fixed inset-0 z-50 bg-white' : ''}`}>
      <Card className={`${isFullscreen ? 'h-full rounded-none border-0' : ''}`}>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <GitCompare className="w-5 h-5" />
              Interactive Diff Viewer
            </CardTitle>
            
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setIsFullscreen(!isFullscreen)}
              >
                {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
              </Button>
            </div>
          </div>
          
          {/* Controls */}
          <div className="flex items-center justify-between gap-4">
            <div className="flex items-center gap-2">
              <Button
                variant={viewMode === 'side-by-side' ? 'default' : 'outline'}
                size="sm"
                onClick={() => setViewMode('side-by-side')}
              >
                <SplitSquareHorizontal className="w-4 h-4 mr-1" />
                Side by Side
              </Button>
              
              <Button
                variant={viewMode === 'unified' ? 'default' : 'outline'}
                size="sm"
                onClick={() => setViewMode('unified')}
              >
                <Square className="w-4 h-4 mr-1" />
                Unified
              </Button>
              
              <Button
                variant={showOnlyChanges ? 'default' : 'outline'}
                size="sm"
                onClick={() => setShowOnlyChanges(!showOnlyChanges)}
              >
                <Filter className="w-4 h-4 mr-1" />
                Changes Only
              </Button>
            </div>
            
            <div className="flex items-center gap-2">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search functions..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10 pr-4 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
            </div>
          </div>
        </CardHeader>

        <CardContent className={`${isFullscreen ? 'flex-1 overflow-auto' : ''} p-0`}>
          <div className="divide-y">
            {filteredSections.map((section) => {
              const isExpanded = expandedSections.has(section.functionName);
              const hasChanges = section.sourceLines.some(line => line.type !== 'unchanged') ||
                               section.targetLines.some(line => line.type !== 'unchanged');
              
              if (showOnlyChanges && !hasChanges) return null;

              return (
                <div key={section.functionName} className="border-b">
                  {/* Section Header */}
                  <div
                    className="flex items-center justify-between p-4 hover:bg-gray-50 cursor-pointer"
                    onClick={() => toggleSection(section.functionName)}
                  >
                    <div className="flex items-center gap-3">
                      {isExpanded ? (
                        <ChevronDown className="w-4 h-4 text-gray-500" />
                      ) : (
                        <ChevronRight className="w-4 h-4 text-gray-500" />
                      )}
                      
                      <Code className="w-4 h-4 text-gray-600" />
                      
                      <span className="font-mono font-medium">{section.functionName}</span>
                      
                      <Badge variant="outline" className={getMatchTypeColor(section.matchType)}>
                        {section.matchType}
                      </Badge>
                      
                      {section.similarity > 0 && (
                        <Badge variant="outline">
                          {(section.similarity * 100).toFixed(1)}% similar
                        </Badge>
                      )}
                    </div>
                    
                    <div className="flex items-center gap-2">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          setSelectedFunction(section.functionName);
                          onFunctionSelect?.(section.functionName);
                        }}
                      >
                        <Eye className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>

                  {/* Diff Content */}
                  {isExpanded && (
                    <div className="bg-gray-50">
                      {viewMode === 'side-by-side' ? (
                        <div className="grid grid-cols-2 gap-px bg-gray-200">
                          {/* Source Column */}
                          <div className="bg-white">
                            <div className="px-3 py-2 bg-gray-100 border-b text-sm font-medium text-gray-700">
                              Source
                            </div>
                            <div className="max-h-96 overflow-auto">
                              {section.sourceLines.map((line) => renderDiffLine(line, 'source'))}
                              {section.sourceLines.length === 0 && (
                                <div className="p-4 text-center text-gray-500 italic">
                                  No source content (new function)
                                </div>
                              )}
                            </div>
                          </div>
                          
                          {/* Target Column */}
                          <div className="bg-white">
                            <div className="px-3 py-2 bg-gray-100 border-b text-sm font-medium text-gray-700">
                              Target
                            </div>
                            <div className="max-h-96 overflow-auto">
                              {section.targetLines.map((line) => renderDiffLine(line, 'target'))}
                              {section.targetLines.length === 0 && (
                                <div className="p-4 text-center text-gray-500 italic">
                                  No target content (deleted function)
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      ) : (
                        /* Unified View */
                        <div className="bg-white">
                          <div className="px-3 py-2 bg-gray-100 border-b text-sm font-medium text-gray-700">
                            Unified Diff
                          </div>
                          <div className="max-h-96 overflow-auto">
                            {/* Combine and sort lines for unified view */}
                            {[...section.sourceLines, ...section.targetLines]
                              .sort((a, b) => a.lineNumber - b.lineNumber)
                              .map((line, index) => (
                                <div key={index}>
                                  {renderDiffLine(line, 'source')}
                                </div>
                              ))}
                          </div>
                        </div>
                      )}
                    </div>
                  )}
                </div>
              );
            })}
            
            {filteredSections.length === 0 && (
              <div className="text-center py-12 text-gray-500">
                <Search className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No functions found matching "{searchTerm}"</p>
                <p className="text-sm">Try adjusting your search terms</p>
              </div>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
