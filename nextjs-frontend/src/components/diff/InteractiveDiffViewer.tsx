'use client';

import React, { useState, useMemo, useEffect } from 'react';
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
import { ComparisonResult } from '@/services/comparisonService';
import { diffService, DiffLine as ServiceDiffLine, FileDiff } from '@/services/diffService';

interface DiffSection {
  id: string;
  title: string;
  sourcePath?: string;
  targetPath?: string;
  fileDiff?: FileDiff;
  similarity: number;
  matchType: string;
  expanded: boolean;
}

interface InteractiveDiffViewerProps {
  data?: ComparisonResult;
  onFunctionSelect?: (functionId: string) => void;
}

export function InteractiveDiffViewer({ data, onFunctionSelect }: InteractiveDiffViewerProps) {
  const [viewMode, setViewMode] = useState<'side-by-side' | 'unified'>('side-by-side');
  const [showOnlyChanges, setShowOnlyChanges] = useState(false);
  const [expandedSections, setExpandedSections] = useState<Set<string>>(new Set());
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedFunction, setSelectedFunction] = useState<string | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [loadedDiffs, setLoadedDiffs] = useState<Map<string, FileDiff>>(new Map());

  // Generate diff data from the real comparison result
  const diffSections = useMemo(() => {
    if (!data || !data.fileChanges) return [];

    const sections: DiffSection[] = [];

    // Process file changes to create diff sections
    data.fileChanges.forEach((fileChange, index) => {
      const sourcePath = fileChange.sourcePath;
      const targetPath = fileChange.targetPath;

      // Handle different types of file changes
      if (fileChange.type === 'modified' && sourcePath && targetPath) {
        // Modified files - can show real diff
        const title = sourcePath.split('/').pop() || targetPath.split('/').pop() || `File ${index + 1}`;

        sections.push({
          id: `file-${index}`,
          title,
          sourcePath,
          targetPath,
          similarity: fileChange.similarity || 0,
          matchType: fileChange.type,
          expanded: index < 3, // Expand first 3 sections by default
        });
      } else if (fileChange.type === 'added' && targetPath) {
        // Added files - show as new file
        const title = `${targetPath.split('/').pop()} (added)`;

        sections.push({
          id: `file-${index}`,
          title,
          sourcePath: undefined,
          targetPath,
          similarity: 0,
          matchType: fileChange.type,
          expanded: index < 3,
        });
      } else if (fileChange.type === 'deleted' && sourcePath) {
        // Deleted files - show as deleted file
        const title = `${sourcePath.split('/').pop()} (deleted)`;

        sections.push({
          id: `file-${index}`,
          title,
          sourcePath,
          targetPath: undefined,
          similarity: 0,
          matchType: fileChange.type,
          expanded: index < 3,
        });
      }
    });

    return sections;
  }, [data]);

  // Load file diff when section is expanded
  const loadFileDiff = async (section: DiffSection) => {
    if (loadedDiffs.has(section.id)) {
      return;
    }

    try {
      // Handle different types of file changes
      if (section.matchType === 'modified' && section.sourcePath && section.targetPath) {
        // Modified files - get real diff
        const fileDiff = await diffService.getFileDiff(section.sourcePath, section.targetPath, {
          contextLines: 3,
          ignoreWhitespace: false,
          caseSensitive: true,
        });

        setLoadedDiffs(prev => new Map(prev).set(section.id, fileDiff));
      } else {
        // Added or deleted files - create synthetic diff
        const sourceContent = section.sourcePath ? await diffService.getFileContent(section.sourcePath) : '';
        const targetContent = section.targetPath ? await diffService.getFileContent(section.targetPath) : '';

        const lines = diffService.generateDiffLines(sourceContent, targetContent);
        const stats = diffService.calculateDiffStats(lines);

        const syntheticDiff: FileDiff = {
          sourcePath: section.sourcePath || '',
          targetPath: section.targetPath || '',
          sourceContent,
          targetContent,
          lines,
          stats,
        };

        setLoadedDiffs(prev => new Map(prev).set(section.id, syntheticDiff));
      }
    } catch (error) {
      console.error('Failed to load file diff:', error);

      // Create an error placeholder diff
      const errorDiff: FileDiff = {
        sourcePath: section.sourcePath || '',
        targetPath: section.targetPath || '',
        sourceContent: '',
        targetContent: '',
        lines: [{
          lineNumber: 1,
          content: `Error loading file content: ${error}`,
          type: 'unchanged',
          oldLineNumber: 1,
          newLineNumber: 1,
        }],
        stats: { additions: 0, deletions: 0, modifications: 0 },
      };

      setLoadedDiffs(prev => new Map(prev).set(section.id, errorDiff));
    }
  };

  // Load diffs for initially expanded sections
  useEffect(() => {
    diffSections.forEach(section => {
      if (section.expanded) {
        loadFileDiff(section);
      }
    });
  }, [diffSections]);

  // Filter sections based on search term
  const filteredSections = useMemo(() => {
    if (!searchTerm) return diffSections;
    return diffSections.filter(section => {
      const titleMatch = section.title.toLowerCase().includes(searchTerm.toLowerCase());
      const pathMatch = section.sourcePath?.toLowerCase().includes(searchTerm.toLowerCase()) ||
                       section.targetPath?.toLowerCase().includes(searchTerm.toLowerCase());

      // Also search in loaded diff content
      const diff = loadedDiffs.get(section.id);
      const contentMatch = diff?.lines.some(line =>
        line.content.toLowerCase().includes(searchTerm.toLowerCase())
      );

      return titleMatch || pathMatch || contentMatch;
    });
  }, [diffSections, searchTerm, loadedDiffs]);

  const toggleSection = (sectionId: string) => {
    const newExpanded = new Set(expandedSections);
    if (newExpanded.has(sectionId)) {
      newExpanded.delete(sectionId);
    } else {
      newExpanded.add(sectionId);
      // Load diff when expanding
      const section = diffSections.find(s => s.id === sectionId);
      if (section) {
        loadFileDiff(section);
      }
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

  const renderDiffLine = (line: ServiceDiffLine, side: 'source' | 'target' | 'unified', index: number) => {
    const lineNumber = side === 'unified'
      ? (line.oldLineNumber || line.newLineNumber || index + 1)
      : side === 'source'
        ? (line.oldLineNumber || index + 1)
        : (line.newLineNumber || index + 1);

    const prefix = side === 'unified'
      ? line.type === 'added' ? '+' : line.type === 'removed' ? '-' : ' '
      : '';

    return (
      <div
        key={`${side}-${index}`}
        className={`flex items-start gap-3 px-3 py-1 font-mono text-sm ${getLineTypeColor(line.type)}`}
      >
        <span className="w-8 text-right text-gray-500 select-none">
          {lineNumber}
        </span>
        {side === 'unified' && (
          <span className="w-4 text-gray-500 select-none">
            {prefix}
          </span>
        )}
        <span className="flex-1 whitespace-pre-wrap break-all">
          {line.content}
        </span>
      </div>
    );
  };

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
              const isExpanded = expandedSections.has(section.id);
              const diff = loadedDiffs.get(section.id);
              const hasChanges = diff?.lines.some(line => line.type !== 'unchanged') ?? true;

              if (showOnlyChanges && !hasChanges) return null;

              return (
                <div key={section.id} className="border-b">
                  {/* Section Header */}
                  <div
                    className="flex items-center justify-between p-4 hover:bg-gray-50 cursor-pointer"
                    onClick={() => toggleSection(section.id)}
                  >
                    <div className="flex items-center gap-3">
                      {isExpanded ? (
                        <ChevronDown className="w-4 h-4 text-gray-500" />
                      ) : (
                        <ChevronRight className="w-4 h-4 text-gray-500" />
                      )}

                      <Code className="w-4 h-4 text-gray-600" />

                      <span className="font-mono font-medium">{section.title}</span>

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
                          setSelectedFunction(section.id);
                          onFunctionSelect?.(section.id);
                        }}
                      >
                        <Eye className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>

                  {/* Diff Content */}
                  {isExpanded && (
                    <div className="bg-gray-50">
                      {diff ? (
                        <>
                          {/* File paths */}
                          <div className="px-4 py-2 bg-gray-100 border-b text-xs text-gray-600">
                            <div className="flex justify-between">
                              <span>Source: {diff.sourcePath}</span>
                              <span>Target: {diff.targetPath}</span>
                            </div>
                            <div className="mt-1 flex gap-4">
                              <span className="text-green-600">+{diff.stats.additions}</span>
                              <span className="text-red-600">-{diff.stats.deletions}</span>
                              {diff.stats.modifications > 0 && (
                                <span className="text-blue-600">~{diff.stats.modifications}</span>
                              )}
                            </div>
                          </div>

                          {viewMode === 'side-by-side' ? (
                            <div className="grid grid-cols-2 gap-px bg-gray-200">
                              {/* Source Column */}
                              <div className="bg-white">
                                <div className="px-3 py-2 bg-gray-100 border-b text-sm font-medium text-gray-700">
                                  Source
                                </div>
                                <div className="max-h-96 overflow-auto">
                                  {diff.lines
                                    .filter(line => line.type !== 'added')
                                    .map((line, index) => renderDiffLine(line, 'source', index))}
                                </div>
                              </div>

                              {/* Target Column */}
                              <div className="bg-white">
                                <div className="px-3 py-2 bg-gray-100 border-b text-sm font-medium text-gray-700">
                                  Target
                                </div>
                                <div className="max-h-96 overflow-auto">
                                  {diff.lines
                                    .filter(line => line.type !== 'removed')
                                    .map((line, index) => renderDiffLine(line, 'target', index))}
                                </div>
                              </div>
                            </div>
                          ) : (
                            <div className="bg-white">
                              <div className="max-h-96 overflow-auto">
                                {diff.lines.map((line, index) => renderDiffLine(line, 'unified', index))}
                              </div>
                            </div>
                          )}
                        </>
                      ) : (
                        <div className="p-4 text-center text-gray-500">
                          Loading diff...
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
