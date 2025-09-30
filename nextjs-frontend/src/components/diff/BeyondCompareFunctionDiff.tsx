'use client';

import React, { useState, useMemo, useEffect } from 'react';
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
  Filter,
  Zap
} from 'lucide-react';
import { FunctionMatch, FunctionInfo } from '@/services/comparisonService';
import { ModernASTDiffViewer } from './ModernASTDiffViewer';

// Diff View Component with unified and side-by-side modes
interface DiffViewProps {
  sourceContent: string;
  targetContent: string;
  sourceFilePath: string;
  targetFilePath: string;
}

interface LineDiff {
  sourceLineNum: number | null;
  targetLineNum: number | null;
  sourceContent: string;
  targetContent: string;
  type: 'unchanged' | 'added' | 'deleted' | 'modified';
}

type ViewMode = 'unified' | 'side-by-side';

type DiffAlgorithm = 'lcs' | 'ast';

const UnifiedDiffView: React.FC<DiffViewProps> = ({
  sourceContent,
  targetContent,
  sourceFilePath,
  targetFilePath,
}) => {
  const [lineDiffs, setLineDiffs] = React.useState<LineDiff[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [viewMode, setViewMode] = React.useState<ViewMode>('unified');
  const [diffAlgorithm, setDiffAlgorithm] = React.useState<DiffAlgorithm>('lcs');
  const scrollContainerRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    const fetchDiff = async () => {
      try {
        setLoading(true);

        // Call the Rust backend AST diff API
        const response = await fetch('http://localhost:8080/api/ast-diff', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            source_content: sourceContent,
            target_content: targetContent,
            source_file_path: sourceFilePath,
            target_file_path: targetFilePath,
            language: 'auto',
            options: {
              enable_semantic_analysis: diffAlgorithm === 'ast',
              enable_structural_analysis: diffAlgorithm === 'ast',
              generate_line_mapping: true,
              diff_algorithm: diffAlgorithm,
              use_tree_edit_distance: diffAlgorithm === 'ast',
              use_hungarian_matching: diffAlgorithm === 'ast',
            },
          }),
        });

        if (!response.ok) {
          throw new Error('Failed to fetch diff');
        }

        const data = await response.json();

        // Convert line mappings to our format
        const diffs: LineDiff[] = data.line_mappings.map((mapping: any) => ({
          sourceLineNum: mapping.source_line,
          targetLineNum: mapping.target_line,
          sourceContent: mapping.source_content || '',
          targetContent: mapping.target_content || '',
          type: mapping.change_type,
        }));

        setLineDiffs(diffs);
      } catch (error) {
        console.error('Error fetching diff:', error);
        // Fallback to simple line-by-line comparison
        const sourceLines = sourceContent.split('\n');
        const targetLines = targetContent.split('\n');
        const maxLines = Math.max(sourceLines.length, targetLines.length);

        const fallbackDiffs: LineDiff[] = [];
        for (let i = 0; i < maxLines; i++) {
          const sourceLine = sourceLines[i] || '';
          const targetLine = targetLines[i] || '';

          let type: LineDiff['type'] = 'unchanged';
          if (i >= sourceLines.length) {
            type = 'added';
          } else if (i >= targetLines.length) {
            type = 'deleted';
          } else if (sourceLine !== targetLine) {
            type = 'modified';
          }

          fallbackDiffs.push({
            sourceLineNum: i < sourceLines.length ? i + 1 : null,
            targetLineNum: i < targetLines.length ? i + 1 : null,
            sourceContent: sourceLine,
            targetContent: targetLine,
            type,
          });
        }

        setLineDiffs(fallbackDiffs);
      } finally {
        setLoading(false);
      }
    };

    if (sourceContent || targetContent) {
      fetchDiff();
    }
  }, [sourceContent, targetContent, sourceFilePath, targetFilePath, diffAlgorithm]);

  const getLineBackgroundColor = (type: LineDiff['type']) => {
    switch (type) {
      case 'added':
        return 'bg-green-900/20';
      case 'deleted':
        return 'bg-red-900/20';
      case 'modified':
        return 'bg-amber-900/20';
      default:
        return 'bg-slate-900';
    }
  };

  const getLineBorderColor = (type: LineDiff['type']) => {
    switch (type) {
      case 'added':
        return 'border-l-4 border-green-500';
      case 'deleted':
        return 'border-l-4 border-red-500';
      case 'modified':
        return 'border-l-4 border-amber-500';
      default:
        return '';
    }
  };

  if (loading) {
    return (
      <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        <div className="bg-slate-800 px-5 py-3 flex items-center gap-2">
          <Code className="w-4 h-4 text-white" />
          <h3 className="font-semibold text-white">Code Comparison</h3>
        </div>
        <div className="p-8 text-center text-slate-500">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-slate-500 mx-auto mb-2"></div>
          Analyzing differences...
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
      <div className="bg-slate-800 px-5 py-3 flex items-center gap-3">
        <Code className="w-4 h-4 text-white" />
        <h3 className="font-semibold text-white">Code Comparison</h3>

        {/* Algorithm Selector */}
        <div className="ml-4 flex items-center gap-1 bg-slate-700 rounded-lg p-1">
          <button
            onClick={() => setDiffAlgorithm('lcs')}
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
              diffAlgorithm === 'lcs'
                ? 'bg-blue-600 text-white shadow-sm'
                : 'text-slate-300 hover:text-white'
            }`}
            title="Fast line-based diff (like git diff)"
          >
            LCS
          </button>
          <button
            onClick={() => setDiffAlgorithm('ast')}
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
              diffAlgorithm === 'ast'
                ? 'bg-purple-600 text-white shadow-sm'
                : 'text-slate-300 hover:text-white'
            }`}
            title="AST-aware diff with Zhang-Shasha & Hungarian matching"
          >
            AST
          </button>
        </div>

        {/* View Mode Toggle */}
        <div className="flex items-center gap-1 bg-slate-700 rounded-lg p-1">
          <button
            onClick={() => setViewMode('unified')}
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
              viewMode === 'unified'
                ? 'bg-slate-900 text-white shadow-sm'
                : 'text-slate-300 hover:text-white'
            }`}
          >
            Unified
          </button>
          <button
            onClick={() => setViewMode('side-by-side')}
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
              viewMode === 'side-by-side'
                ? 'bg-slate-900 text-white shadow-sm'
                : 'text-slate-300 hover:text-white'
            }`}
          >
            Side-by-Side
          </button>
        </div>

        {/* Legend */}
        <div className="ml-auto flex gap-3 text-xs">
          <span className="flex items-center gap-1">
            <span className="w-3 h-3 bg-green-500 rounded"></span>
            <span className="text-slate-300">Added</span>
          </span>
          <span className="flex items-center gap-1">
            <span className="w-3 h-3 bg-red-500 rounded"></span>
            <span className="text-slate-300">Deleted</span>
          </span>
          <span className="flex items-center gap-1">
            <span className="w-3 h-3 bg-amber-500 rounded"></span>
            <span className="text-slate-300">Modified</span>
          </span>
        </div>
      </div>

      {/* Unified View */}
      {viewMode === 'unified' && (
        <div
          ref={scrollContainerRef}
          className="overflow-auto max-h-[600px]"
          style={{ scrollbarGutter: 'stable' }}
        >
          <div className="grid grid-cols-2 divide-x divide-slate-700">
            {/* Source Column */}
            <div className="bg-slate-900">
              <div className="sticky top-0 z-10 px-4 py-2 bg-slate-800 border-b border-slate-700">
                <span className="text-xs font-medium text-slate-300 uppercase tracking-wide">Source</span>
              </div>
              <div>
                {lineDiffs.map((diff, idx) => (
                  <div
                    key={`source-${idx}`}
                    className={`flex ${getLineBackgroundColor(diff.type === 'added' ? 'unchanged' : diff.type)} ${
                      diff.type === 'deleted' || diff.type === 'modified' ? getLineBorderColor(diff.type) : ''
                    }`}
                  >
                    <div className="w-12 flex-shrink-0 text-right pr-3 py-1 text-slate-500 text-xs font-mono select-none border-r border-slate-700">
                      {diff.sourceLineNum || ''}
                    </div>
                    <div className="flex-1 px-3 py-1">
                      <pre className="font-mono text-sm text-slate-100 whitespace-pre">
                        {diff.sourceContent || ' '}
                      </pre>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Target Column */}
            <div className="bg-slate-900">
              <div className="sticky top-0 z-10 px-4 py-2 bg-slate-800 border-b border-slate-700">
                <span className="text-xs font-medium text-slate-300 uppercase tracking-wide">Target</span>
              </div>
              <div>
                {lineDiffs.map((diff, idx) => (
                  <div
                    key={`target-${idx}`}
                    className={`flex ${getLineBackgroundColor(diff.type === 'deleted' ? 'unchanged' : diff.type)} ${
                      diff.type === 'added' || diff.type === 'modified' ? getLineBorderColor(diff.type) : ''
                    }`}
                  >
                    <div className="w-12 flex-shrink-0 text-right pr-3 py-1 text-slate-500 text-xs font-mono select-none border-r border-slate-700">
                      {diff.targetLineNum || ''}
                    </div>
                    <div className="flex-1 px-3 py-1">
                      <pre className="font-mono text-sm text-slate-100 whitespace-pre">
                        {diff.targetContent || ' '}
                      </pre>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Side-by-Side View */}
      {viewMode === 'side-by-side' && (
        <div className="grid grid-cols-2 divide-x divide-slate-700">
          {/* Source Code */}
          <div className="bg-slate-900">
            <div className="sticky top-0 z-10 px-4 py-2 bg-slate-800 border-b border-slate-700">
              <span className="text-xs font-medium text-slate-300 uppercase tracking-wide">Source</span>
            </div>
            <div className="overflow-auto max-h-[600px]">
              <div>
                {lineDiffs
                  .filter(diff => diff.sourceLineNum !== null)
                  .map((diff, idx) => (
                    <div
                      key={`source-sbs-${idx}`}
                      className={`flex ${getLineBackgroundColor(diff.type === 'added' ? 'unchanged' : diff.type)} ${
                        diff.type === 'deleted' || diff.type === 'modified' ? getLineBorderColor(diff.type) : ''
                      }`}
                    >
                      <div className="w-12 flex-shrink-0 text-right pr-3 py-1 text-slate-500 text-xs font-mono select-none border-r border-slate-700">
                        {diff.sourceLineNum}
                      </div>
                      <div className="flex-1 px-3 py-1">
                        <pre className="font-mono text-sm text-slate-100 whitespace-pre">
                          {diff.sourceContent}
                        </pre>
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          </div>

          {/* Target Code */}
          <div className="bg-slate-900">
            <div className="sticky top-0 z-10 px-4 py-2 bg-slate-800 border-b border-slate-700">
              <span className="text-xs font-medium text-slate-300 uppercase tracking-wide">Target</span>
            </div>
            <div className="overflow-auto max-h-[600px]">
              <div>
                {lineDiffs
                  .filter(diff => diff.targetLineNum !== null)
                  .map((diff, idx) => (
                    <div
                      key={`target-sbs-${idx}`}
                      className={`flex ${getLineBackgroundColor(diff.type === 'deleted' ? 'unchanged' : diff.type)} ${
                        diff.type === 'added' || diff.type === 'modified' ? getLineBorderColor(diff.type) : ''
                      }`}
                    >
                      <div className="w-12 flex-shrink-0 text-right pr-3 py-1 text-slate-500 text-xs font-mono select-none border-r border-slate-700">
                        {diff.targetLineNum}
                      </div>
                      <div className="flex-1 px-3 py-1">
                        <pre className="font-mono text-sm text-slate-100 whitespace-pre">
                          {diff.targetContent}
                        </pre>
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

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
  fileChanges?: any[]; // Add file changes to show all files
  summary?: any; // Add summary data for total counts
  onFunctionSelect?: (pair: FunctionPair) => void;
  onSimilarityThresholdChange?: (threshold: number) => void;
  similarityThreshold?: number;
}

export function BeyondCompareFunctionDiff({
  functionMatches,
  fileChanges = [],
  summary,
  onFunctionSelect,
  onSimilarityThresholdChange,
  similarityThreshold = 0.7
}: BeyondCompareFunctionDiffProps) {
  const [selectedPair, setSelectedPair] = useState<FunctionPair | null>(null);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [showModernDiff, setShowModernDiff] = useState(false);
  const [expandedFiles, setExpandedFiles] = useState<Set<string>>(new Set());
  const [filterType, setFilterType] = useState<string>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [headerHeights, setHeaderHeights] = useState<Map<number, number>>(new Map());

  // Convert function matches to function pairs and apply filtering
  const { functionPairs, fileGroups, filteredPairs, fileStatusMap, allFileGroups } = useMemo(() => {
    const pairs: FunctionPair[] = [];
    const groups = new Map<string, FunctionPair[]>();
    const allFileGroups = new Map<string, FunctionPair[]>(); // Track all functions per file (unfiltered)
    const statusMap = new Map<string, any>(); // Map file names to their change status

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
        matchType: match.matchType || match.type, // Use matchType if available, fallback to type
        similarity: match.similarity,
        changes: match.changes
      };

      pairs.push(pair);

      // Add to allFileGroups (unfiltered) for accurate function counts
      const fileName = match.sourceFunction?.filePath || match.targetFunction?.filePath || 'Unknown';
      const fileKey = fileName.split('/').pop() || fileName;
      if (!allFileGroups.has(fileKey)) {
        allFileGroups.set(fileKey, []);
      }
      allFileGroups.get(fileKey)!.push(pair);
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
        let shouldInclude = false;
        switch (filterType) {
          case 'added': shouldInclude = pair.matchType === 'added'; break;
          case 'deleted': shouldInclude = pair.matchType === 'deleted'; break;
          case 'modified': shouldInclude = pair.matchType === 'similar' || pair.matchType === 'modified' || pair.changes?.bodyChanged || pair.changes?.signatureChanged; break;
          case 'moved': shouldInclude = pair.matchType === 'moved' || pair.changes?.moved; break;
          case 'renamed': shouldInclude = pair.matchType === 'renamed' || pair.changes?.renamed; break;
          case 'unchanged': shouldInclude = pair.matchType === 'identical'; break;
          default: shouldInclude = true; break;
        }

        return shouldInclude;
      });
    }

    // Group filtered pairs by file
    const filteredGroups = new Map<string, FunctionPair[]>();

    // Count actual functions from functionMatches (both source and target)
    const actualFunctionCounts = new Map<string, Set<string>>();

    functionMatches.forEach(match => {
      if (match.sourceFunction) {
        const sourceFile = match.sourceFunction.filePath.split('/').pop() || match.sourceFunction.filePath;
        if (!actualFunctionCounts.has(sourceFile)) {
          actualFunctionCounts.set(sourceFile, new Set());
        }
        actualFunctionCounts.get(sourceFile)!.add(match.sourceFunction.name);
      }

      if (match.targetFunction) {
        const targetFile = match.targetFunction.filePath.split('/').pop() || match.targetFunction.filePath;
        if (!actualFunctionCounts.has(targetFile)) {
          actualFunctionCounts.set(targetFile, new Set());
        }
        actualFunctionCounts.get(targetFile)!.add(match.targetFunction.name);
      }
    });

    // First, add all files from fileChanges (even if they have no functions)
    fileChanges.forEach(fileChange => {
      const filePath = fileChange.sourcePath || fileChange.targetPath || 'Unknown';
      const fileKey = filePath.split('/').pop() || filePath;

      if (!filteredGroups.has(fileKey)) {
        filteredGroups.set(fileKey, []);
      }

      // Use actual count from function matches (0 if no functions found)
      const actualCount = actualFunctionCounts.get(fileKey)?.size || 0;

      // Store file status for display with function count
      statusMap.set(fileKey, {
        ...fileChange,
        functionCount: actualCount
      });
    });

    // Then add function pairs to their respective files
    filtered.forEach(pair => {
      const fileName = pair.sourceFunction?.filePath || pair.targetFunction?.filePath || 'Unknown';
      const fileKey = fileName.split('/').pop() || fileName;

      if (!filteredGroups.has(fileKey)) {
        filteredGroups.set(fileKey, []);
      }

      filteredGroups.get(fileKey)!.push(pair);
    });

    return {
      functionPairs: pairs,
      fileGroups: filteredGroups,
      filteredPairs: filtered,
      fileStatusMap: statusMap,
      allFileGroups: allFileGroups
    };
  }, [functionMatches, fileChanges, searchTerm, filterType]);

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

  const handleModernDiffClick = (pair: FunctionPair) => {
    setSelectedPair(pair);
    setShowModernDiff(true);
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
    switch (pair.matchType) {
      case 'added':
        return <ArrowRight className="w-4 h-4 text-green-600" />;
      case 'deleted':
        return <X className="w-4 h-4 text-red-600" />;
      case 'moved':
        return <ArrowUpDown className="w-4 h-4 text-purple-600" />;
      case 'renamed':
        return <ArrowRight className="w-4 h-4 text-blue-600" />;
      case 'similar':
      case 'modified':
        return <ArrowRight className="w-4 h-4 text-yellow-600" />;
      case 'identical':
        return <ArrowRight className="w-4 h-4 text-gray-600" />;
      default:
        return <ArrowRight className="w-4 h-4 text-gray-600" />;
    }
  };

  // Update header heights when component mounts or fileGroups changes
  useEffect(() => {
    // Small delay to ensure DOM is ready
    const timer = setTimeout(() => {
      const newHeights = new Map<number, number>();
      Array.from(fileGroups.keys()).forEach((_, index) => {
        const header = document.getElementById(`source-file-header-${index}`);
        if (header) {
          newHeights.set(index, header.offsetHeight);
        }
      });
      setHeaderHeights(newHeights);
    }, 0);

    return () => clearTimeout(timer);
  }, [fileGroups]);

  // Synchronize row heights across all three columns
  useEffect(() => {
    const timer = setTimeout(() => {
      functionPairs.forEach((pair) => {
        const sourceEl = document.getElementById(`source-${pair.id}`);
        const mappingEl = document.getElementById(`mapping-${pair.id}`);
        const targetEl = document.getElementById(`target-${pair.id}`);

        if (sourceEl && mappingEl && targetEl) {
          // Reset heights first
          sourceEl.style.minHeight = '';
          mappingEl.style.minHeight = '';
          targetEl.style.minHeight = '';

          // Get natural heights
          const sourceHeight = sourceEl.offsetHeight;
          const mappingHeight = mappingEl.offsetHeight;
          const targetHeight = targetEl.offsetHeight;

          // Set all to the maximum height
          const maxHeight = Math.max(sourceHeight, mappingHeight, targetHeight);
          sourceEl.style.minHeight = `${maxHeight}px`;
          mappingEl.style.minHeight = `${maxHeight}px`;
          targetEl.style.minHeight = `${maxHeight}px`;
        }
      });
    }, 100);

    return () => clearTimeout(timer);
  }, [functionPairs, expandedFiles]);

  return (
    <div className="h-full flex flex-col overflow-hidden w-full">
      <Card className="flex-1 flex flex-col overflow-hidden">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <GitCompare className="w-5 h-5" />
              Function Comparison - Beyond Compare Style
            </CardTitle>

            <div className="flex flex-col gap-2">
              {/* File Summary */}
              <div className="flex items-center gap-2 text-sm">
                <span className="font-medium">Files:</span>
                <Badge variant="outline" className="bg-blue-50 text-blue-700">
                  {Array.from(fileGroups.keys()).length} total
                </Badge>
                <Badge variant="outline" className="bg-green-50 text-green-700">
                  +{Array.from(fileStatusMap.values()).filter(f => f.type === 'added').length} Added
                </Badge>
                <Badge variant="outline" className="bg-red-50 text-red-700">
                  -{Array.from(fileStatusMap.values()).filter(f => f.type === 'deleted').length} Deleted
                </Badge>
                <Badge variant="outline" className="bg-yellow-50 text-yellow-700">
                  ~{Array.from(fileStatusMap.values()).filter(f => f.type === 'modified').length} Modified
                </Badge>
              </div>

              {/* Function Summary */}
              <div className="flex items-center gap-2 text-sm">
                <span className="font-medium">Functions:</span>
                <Badge variant="outline" className="bg-green-50 text-green-700">
                  +{functionPairs.filter(p => p.matchType === 'added').length} Added
                </Badge>
                <Badge variant="outline" className="bg-red-50 text-red-700">
                  -{functionPairs.filter(p => p.matchType === 'deleted').length} Deleted
                </Badge>
                <Badge variant="outline" className="bg-yellow-50 text-yellow-700">
                  ~{functionPairs.filter(p => p.matchType === 'similar' || p.matchType === 'modified' || p.changes?.bodyChanged).length} Modified
                </Badge>
                <Badge variant="outline" className="bg-gray-50 text-gray-700">
                  ={functionPairs.filter(p => p.matchType === 'identical').length} Unchanged
                </Badge>
              </div>
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
                <option value="modified">Modified ({functionPairs.filter(p => p.matchType === 'similar' || p.matchType === 'modified' || p.changes?.bodyChanged).length})</option>
                <option value="moved">Moved ({functionPairs.filter(p => p.matchType === 'moved' || p.changes?.moved).length})</option>
                <option value="renamed">Renamed ({functionPairs.filter(p => p.matchType === 'renamed' || p.changes?.renamed).length})</option>
                <option value="unchanged">Unchanged ({functionPairs.filter(p => p.matchType === 'identical').length})</option>
              </select>
            </div>

            <div className="text-sm text-gray-600">
              Showing {filteredPairs.length} of {functionPairs.length} functions
            </div>

            {/* Similarity Threshold Slider */}
            <div className="flex items-center gap-2 px-3 py-2 bg-gray-50 rounded-md">
              <label className="text-xs font-medium text-gray-700 whitespace-nowrap">
                Similarity:
              </label>
              <input
                type="range"
                min="0.1"
                max="0.99"
                step="0.05"
                value={similarityThreshold}
                onChange={(e) => onSimilarityThresholdChange?.(parseFloat(e.target.value))}
                className="w-20 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
              />
              <span className="text-xs font-mono text-gray-600 min-w-[3rem]">
                {(similarityThreshold * 100).toFixed(0)}%
              </span>
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

        <CardContent className="p-0 flex-1 overflow-hidden">
          {/* Shared scroll container for all three columns */}
          <div className="h-full overflow-auto">
            <div className="grid grid-cols-[5fr_2fr_5fr] divide-x divide-gray-200 min-h-full">
              {/* Source Functions */}
              <div className="bg-red-50 flex flex-col">
                <div className="p-3 bg-red-100 border-b font-medium text-red-800 sticky top-0 z-20">
                  Source Functions
                </div>
                <div>
                  {Array.from(fileGroups.keys()).map((fileName, fileIndex) => {
                    const pairs = fileGroups.get(fileName) || [];
                    return (
                    <div key={fileName} className="border-b border-red-200">
                      <div
                        id={`source-file-header-${fileIndex}`}
                        className="p-2 bg-red-50 cursor-pointer hover:bg-red-100 flex items-center gap-2 sticky top-[49px] z-10"
                        onClick={() => toggleFileExpansion(fileName)}
                      >
                        {expandedFiles.has(fileName) ?
                          <ChevronDown className="w-4 h-4" /> :
                          <ChevronRight className="w-4 h-4" />
                        }
                        <span className="text-sm font-medium">{fileName}</span>
                        <div className="flex items-center gap-2 ml-auto">
                          {fileStatusMap.get(fileName) && (
                            <Badge
                              variant="outline"
                              className={`text-xs ${
                                fileStatusMap.get(fileName).type === 'added' ? 'bg-green-100 text-green-800' :
                                fileStatusMap.get(fileName).type === 'deleted' ? 'bg-red-100 text-red-800' :
                                fileStatusMap.get(fileName).type === 'modified' ? 'bg-blue-100 text-blue-800' :
                                'bg-gray-100 text-gray-800'
                              }`}
                            >
                              {fileStatusMap.get(fileName).type}
                            </Badge>
                          )}
                          <Badge variant="outline" className="text-xs">
                            {pairs.length} fn
                          </Badge>
                        </div>
                      </div>

                      {expandedFiles.has(fileName) && (
                        <div className="p-2">
                          {pairs.length === 0 ? (
                            <div className="text-sm text-gray-500 italic p-2">
                              No function matches found for current filter.
                              Try changing the filter to "All Changes" to see more functions.
                            </div>
                          ) : (
                            <div className="space-y-1">
                              {pairs.map((pair, index) => (
                                <div
                                  key={pair.id}
                                  id={`source-${pair.id}`}
                                  data-pair-id={pair.id}
                                  className="p-2 border rounded cursor-pointer transition-colors hover:bg-red-200 flex flex-col"
                                  onClick={() => handleFunctionClick(pair)}
                                >
                                  {pair.sourceFunction ? (
                                    <>
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
                                        {pair.similarity !== undefined && (
                                          <Badge variant="outline" className="text-xs">
                                            {Math.round(pair.similarity * 100)}% match
                                          </Badge>
                                        )}
                                      </div>
                                      <div className="mt-auto pt-2">
                                        {(pair.changes?.bodyChanged || pair.changes?.signatureChanged) && (
                                          <div className="flex gap-1">
                                            <Button
                                              size="sm"
                                              variant="outline"
                                              className="text-xs h-6"
                                              onClick={(e) => {
                                                e.stopPropagation();
                                                handleDetailClick(pair);
                                              }}
                                            >
                                              <Eye className="w-3 h-3 mr-1" />
                                              Details
                                            </Button>
                                            <Button
                                              size="sm"
                                              variant="outline"
                                              className="text-xs h-6"
                                              onClick={(e) => {
                                                e.stopPropagation();
                                                handleModernDiffClick(pair);
                                              }}
                                            >
                                              <Zap className="w-3 h-3 mr-1" />
                                              AST Diff
                                            </Button>
                                          </div>
                                        )}
                                      </div>
                                    </>
                                  ) : (
                                    <div className="flex items-center justify-center h-full text-xs text-gray-400 italic">
                                      No source function
                                    </div>
                                  )}
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                    );
                  })}
                </div>
              </div>

              {/* Connection/Mapping Column */}
              <div className="bg-gray-50 flex flex-col">
                <div className="p-3 bg-gray-100 border-b font-medium text-gray-800 sticky top-0 z-20">
                  Mapping
                </div>
                <div>
                  {Array.from(fileGroups.keys()).map((fileName, fileIndex) => {
                    const pairs = fileGroups.get(fileName) || [];
                    const sourceHeight = headerHeights.get(fileIndex) || 56;

                    return (
                    <div key={fileName} className="border-b border-gray-200">
                      <div
                        className="p-2 bg-gray-50 cursor-pointer hover:bg-gray-100 sticky top-[49px] z-10 flex items-center justify-center gap-2"
                        style={{ minHeight: `${sourceHeight}px` }}
                        onClick={() => toggleFileExpansion(fileName)}
                      >
                        {expandedFiles.has(fileName) ?
                          <ChevronDown className="w-4 h-4 text-gray-600" /> :
                          <ChevronRight className="w-4 h-4 text-gray-600" />
                        }
                        <span className="text-sm font-medium text-gray-600">{fileName}</span>
                      </div>

                      {expandedFiles.has(fileName) && (
                        <div className="p-2">
                          {pairs.length === 0 ? (
                            <div className="p-4 text-center text-gray-500 text-sm">
                              No function mappings for current filter.
                            </div>
                          ) : (
                            <div className="space-y-1">
                              {pairs.map((pair, index) => (
                                <div
                                  key={pair.id}
                                  id={`mapping-${pair.id}`}
                                  data-pair-id={pair.id}
                                  className="p-2 rounded cursor-pointer hover:bg-gray-100 border border-gray-200 flex items-center justify-center"
                                  onClick={() => handleFunctionClick(pair)}
                                >
                                  <div className="flex flex-col items-center gap-2">
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
                      )}
                    </div>
                    );
                  })}
                </div>
              </div>

              {/* Target Functions */}
              <div className="bg-green-50 flex flex-col">
                <div className="p-3 bg-green-100 border-b font-medium text-green-800 sticky top-0 z-20">
                  Target Functions
                </div>
                <div>
                  {Array.from(fileGroups.keys()).map((fileName, fileIndex) => {
                    const pairs = fileGroups.get(fileName) || [];
                    const sourceHeight = headerHeights.get(fileIndex) || 56;

                    return (
                    <div key={fileName} className="border-b border-green-200">
                      <div
                        className="p-2 bg-green-50 cursor-pointer hover:bg-green-100 sticky top-[49px] z-10 flex items-center gap-2"
                        style={{ minHeight: `${sourceHeight}px` }}
                        onClick={() => toggleFileExpansion(fileName)}
                      >
                        {expandedFiles.has(fileName) ?
                          <ChevronDown className="w-4 h-4" /> :
                          <ChevronRight className="w-4 h-4" />
                        }
                        <span className="text-sm font-medium">{fileName}</span>
                      </div>

                      {expandedFiles.has(fileName) && (
                        <div className="p-2">
                          {pairs.length === 0 ? (
                            <div className="p-4 text-center text-gray-500 text-sm">
                              No function matches found for current filter.
                              <br />
                              <span className="text-xs">
                                File has {fileStatusMap.get(fileName)?.functionCount || 0} functions total.
                                <br />
                                Try changing the filter to "All Changes" to see more functions.
                              </span>
                            </div>
                          ) : (
                            <div className="space-y-1">
                              {pairs.map((pair, index) => (
                                <div
                                  key={pair.id}
                                  id={`target-${pair.id}`}
                                  data-pair-id={pair.id}
                                  className={`p-2 rounded cursor-pointer border flex flex-col ${
                                    pair.targetFunction
                                      ? 'hover:bg-green-100 border-green-200 bg-white'
                                      : 'border-dashed border-gray-300 bg-gray-50 opacity-30'
                                  }`}
                                  onClick={() => handleFunctionClick(pair)}
                                >
                                  {pair.targetFunction ? (
                                    <>
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
                                      <div className="mt-auto pt-2">
                                        {/* Action buttons */}
                                        {pair.sourceFunction && pair.targetFunction && (
                                          <div className="flex gap-1">
                                            <Button
                                              size="sm"
                                              variant="outline"
                                              className="text-xs h-6 px-2"
                                              onClick={(e) => {
                                                e.stopPropagation();
                                                handleModernDiffClick(pair);
                                              }}
                                            >
                                              <Zap className="w-3 h-3 mr-1" />
                                              AST Diff
                                            </Button>
                                          </div>
                                        )}
                                      </div>
                                    </>
                                  ) : (
                                    <div className="flex items-center justify-center h-full text-xs text-gray-400 italic">
                                      No target function
                                    </div>
                                  )}
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                    );
                  })}
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Detailed Function Diff Modal - Modern Wide Design */}
      <Dialog open={showDetailModal} onOpenChange={setShowDetailModal}>
        <DialogContent className="max-w-[95vw] w-[95vw] max-h-[95vh] overflow-hidden flex flex-col p-0">
          {selectedPair && (
            <>
              {/* Modern Header with Gradient */}
              <div className="bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 text-white px-8 py-6 flex-shrink-0">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-3 mb-3">
                      <div className="p-2 bg-white/10 rounded-lg backdrop-blur-sm">
                        <GitCompare className="w-6 h-6" />
                      </div>
                      <div>
                        <h2 className="text-2xl font-bold tracking-tight">
                          {selectedPair?.sourceFunction?.name || selectedPair?.targetFunction?.name}
                        </h2>
                        <p className="text-slate-300 text-sm mt-1">Function Comparison Analysis</p>
                      </div>
                    </div>

                    {/* Status Badges */}
                    <div className="flex items-center gap-3 flex-wrap">
                      <Badge
                        className={`${getMatchTypeColor(selectedPair.matchType)} px-3 py-1 text-sm font-medium`}
                      >
                        {selectedPair.matchType.toUpperCase()}
                      </Badge>

                      {selectedPair.similarity !== undefined && (
                        <div className="flex items-center gap-2 bg-white/10 px-3 py-1 rounded-full backdrop-blur-sm">
                          <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                          <span className="text-sm font-medium">
                            {selectedPair.matchType === 'identical' ? '100' : (selectedPair.similarity * 100).toFixed(1)}% Similarity
                          </span>
                        </div>
                      )}

                      {selectedPair.changes && (
                        <>
                          {selectedPair.changes.signatureChanged && (
                            <Badge variant="outline" className="bg-amber-500/20 text-amber-200 border-amber-400/30">
                              Signature Modified
                            </Badge>
                          )}
                          {selectedPair.changes.bodyChanged && (
                            <Badge variant="outline" className="bg-blue-500/20 text-blue-200 border-blue-400/30">
                              Implementation Changed
                            </Badge>
                          )}
                          {selectedPair.changes.moved && (
                            <Badge variant="outline" className="bg-purple-500/20 text-purple-200 border-purple-400/30">
                              Relocated
                            </Badge>
                          )}
                          {selectedPair.changes.renamed && (
                            <Badge variant="outline" className="bg-pink-500/20 text-pink-200 border-pink-400/30">
                              Renamed
                            </Badge>
                          )}
                        </>
                      )}
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

              {/* Content Area */}
              <div className="flex-1 overflow-auto bg-slate-50">
                <div className="p-8 space-y-6">
                  {/* Metadata Cards */}
                  <div className="grid grid-cols-2 gap-6">
                    {/* Source Function Card */}
                    <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
                      <div className="bg-gradient-to-r from-slate-700 to-slate-600 px-5 py-3 flex items-center gap-2">
                        <ArrowLeft className="w-4 h-4 text-white" />
                        <h3 className="font-semibold text-white">Source Function</h3>
                      </div>
                      {selectedPair.sourceFunction ? (
                        <div className="p-5 space-y-3">
                          <div className="flex items-start justify-between">
                            <div className="flex-1">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Function Name</div>
                              <div className="font-mono text-lg font-bold text-slate-900">{selectedPair.sourceFunction.name}</div>
                            </div>
                            {selectedPair.sourceFunction.complexity && (
                              <div className="text-right">
                                <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Complexity</div>
                                <div className="text-2xl font-bold text-slate-900">{selectedPair.sourceFunction.complexity}</div>
                              </div>
                            )}
                          </div>

                          <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                            <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">Signature</div>
                            <code className="text-xs font-mono text-slate-800 break-all">
                              {selectedPair.sourceFunction.signature}
                            </code>
                          </div>

                          <div className="grid grid-cols-2 gap-3">
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Lines</div>
                              <div className="text-xl font-bold text-slate-900">{selectedPair.sourceFunction.lineCount}</div>
                            </div>
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">File</div>
                              <div className="text-sm font-medium text-slate-900 truncate" title={selectedPair.sourceFunction.filePath}>
                                {selectedPair.sourceFunction.filePath.split('/').pop()}
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : (
                        <div className="p-8 text-center text-slate-400 italic">
                          Function not present in source
                        </div>
                      )}
                    </div>

                    {/* Target Function Card */}
                    <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
                      <div className="bg-gradient-to-r from-blue-600 to-blue-500 px-5 py-3 flex items-center gap-2">
                        <ArrowRight className="w-4 h-4 text-white" />
                        <h3 className="font-semibold text-white">Target Function</h3>
                      </div>
                      {selectedPair.targetFunction ? (
                        <div className="p-5 space-y-3">
                          <div className="flex items-start justify-between">
                            <div className="flex-1">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Function Name</div>
                              <div className="font-mono text-lg font-bold text-slate-900">{selectedPair.targetFunction.name}</div>
                            </div>
                            {selectedPair.targetFunction.complexity && (
                              <div className="text-right">
                                <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Complexity</div>
                                <div className="text-2xl font-bold text-slate-900">{selectedPair.targetFunction.complexity}</div>
                              </div>
                            )}
                          </div>

                          <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                            <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">Signature</div>
                            <code className="text-xs font-mono text-slate-800 break-all">
                              {selectedPair.targetFunction.signature}
                            </code>
                          </div>

                          <div className="grid grid-cols-2 gap-3">
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">Lines</div>
                              <div className="text-xl font-bold text-slate-900">{selectedPair.targetFunction.lineCount}</div>
                            </div>
                            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                              <div className="text-xs font-medium text-slate-500 uppercase tracking-wide mb-1">File</div>
                              <div className="text-sm font-medium text-slate-900 truncate" title={selectedPair.targetFunction.filePath}>
                                {selectedPair.targetFunction.filePath.split('/').pop()}
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : (
                        <div className="p-8 text-center text-slate-400 italic">
                          Function not present in target
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Unified Scroll Code Comparison */}
                  <UnifiedDiffView
                    sourceContent={selectedPair.sourceFunction?.content || ''}
                    targetContent={selectedPair.targetFunction?.content || ''}
                    sourceFilePath={selectedPair.sourceFunction?.filePath || ''}
                    targetFilePath={selectedPair.targetFunction?.filePath || ''}
                  />
                </div>
              </div>
            </>
          )}
        </DialogContent>
      </Dialog>

      {/* Modern AST Diff Viewer - Full Screen Modal */}
      {showModernDiff && selectedPair && selectedPair.sourceFunction && selectedPair.targetFunction && (
        <div className="fixed inset-0 bg-black bg-opacity-80 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg w-[95vw] h-[90vh] overflow-hidden flex flex-col shadow-2xl">
            <ModernASTDiffViewer
              sourceFunction={{
                name: selectedPair.sourceFunction.name,
                content: selectedPair.sourceFunction.content,
                filePath: selectedPair.sourceFunction.filePath || 'unknown',
              }}
              targetFunction={{
                name: selectedPair.targetFunction.name,
                content: selectedPair.targetFunction.content,
                filePath: selectedPair.targetFunction.filePath || 'unknown',
              }}
              language="auto"
              onClose={() => setShowModernDiff(false)}
            />
          </div>
        </div>
      )}
    </div>
  );
}
