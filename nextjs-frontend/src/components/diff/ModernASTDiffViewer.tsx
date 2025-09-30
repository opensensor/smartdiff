'use client';

import React, { useState, useEffect, useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { 
  Code, 
  GitCompare, 
  Zap, 
  Eye, 
  EyeOff,
  ChevronDown,
  ChevronRight,
  Info
} from 'lucide-react';
import { astDiffService, ASTDiffResult, ASTDiffLine, FunctionDiffRequest } from '@/services/astDiffService';

interface ModernASTDiffViewerProps {
  sourceFunction: {
    name: string;
    content: string;
    filePath: string;
  };
  targetFunction: {
    name: string;
    content: string;
    filePath: string;
  };
  language?: string;
  onClose?: () => void;
}

export function ModernASTDiffViewer({
  sourceFunction,
  targetFunction,
  language = 'auto',
  onClose
}: ModernASTDiffViewerProps) {
  const [diffResult, setDiffResult] = useState<ASTDiffResult | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showUnchanged, setShowUnchanged] = useState(true); // Show unchanged by default
  const [showASTOperations, setShowASTOperations] = useState(false);
  const [contextLines, setContextLines] = useState(3);

  useEffect(() => {
    generateDiff();
  }, [sourceFunction, targetFunction, language]);

  const generateDiff = async () => {
    setIsLoading(true);
    setError(null);

    console.log('Generating diff for:', { sourceFunction, targetFunction, language });

    try {
      const request: FunctionDiffRequest = {
        sourceFunction,
        targetFunction,
        language,
      };

      // Try AST diff first, fallback to simple diff
      let result: ASTDiffResult;
      try {
        console.log('Attempting AST diff...');
        result = await astDiffService.generateFunctionDiff(request);
        console.log('AST diff successful:', result);

        // If AST diff returns empty lines, fallback to simple diff
        if (!result.lines || result.lines.length === 0) {
          console.warn('AST diff returned no lines, falling back to simple diff');
          result = astDiffService.generateSimpleDiff(sourceFunction.content, targetFunction.content);
        }
      } catch (astError) {
        console.warn('AST diff failed, falling back to simple diff:', astError);
        result = astDiffService.generateSimpleDiff(sourceFunction.content, targetFunction.content);
        console.log('Simple diff result:', result);
      }

      setDiffResult(result);
    } catch (err: any) {
      console.error('Diff generation failed:', err);
      setError(err.message || 'Failed to generate diff');
    } finally {
      setIsLoading(false);
    }
  };

  const filteredLines = useMemo(() => {
    if (!diffResult) {
      console.log('No diffResult available');
      return [];
    }

    console.log('Filtering lines from diffResult:', diffResult);
    let lines = diffResult.lines;
    console.log('Initial lines:', lines);

    // Filter out unchanged lines if not showing them
    if (!showUnchanged) {
      lines = lines.filter(line => line.type !== 'unchanged');
      console.log('Filtered lines (no unchanged):', lines);
    } else {
      // Add context lines around changes
      const changedIndices = new Set<number>();
      let hasChanges = false;

      lines.forEach((line, index) => {
        if (line.type !== 'unchanged') {
          hasChanges = true;
          for (let i = Math.max(0, index - contextLines); i <= Math.min(lines.length - 1, index + contextLines); i++) {
            changedIndices.add(i);
          }
        }
      });

      // If there are no changes, show all lines
      if (!hasChanges) {
        console.log('No changes found, showing all lines');
        // lines remains unchanged
      } else {
        lines = lines.filter((_, index) => changedIndices.has(index));
        console.log('Filtered lines (with context):', lines);
      }
    }

    console.log('Final filtered lines:', lines);
    return lines;
  }, [diffResult, showUnchanged, contextLines]);

  const getLineTypeColor = (type: string) => {
    switch (type) {
      case 'added': return 'bg-green-100 border-l-4 border-green-500';
      case 'deleted': return 'bg-red-100 border-l-4 border-red-500';
      case 'modified': return 'border-l-4 border-yellow-500'; // No background for modified
      case 'unchanged': return ''; // No background or border for unchanged
      default: return '';
    }
  };

  const getLineTypeIcon = (type: string) => {
    switch (type) {
      case 'added': return <span className="text-green-600 font-bold">+</span>;
      case 'deleted': return <span className="text-red-600 font-bold">-</span>;
      case 'modified': return <span className="text-yellow-600 font-bold">~</span>;
      case 'unchanged': return <span className="text-gray-400"> </span>;
      default: return null;
    }
  };

  if (isLoading) {
    return (
      <Card className="w-full">
        <CardContent className="p-8 text-center">
          <div className="animate-spin w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full mx-auto mb-4"></div>
          <p className="text-gray-600">Generating AST-powered diff...</p>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card className="w-full">
        <CardContent className="p-8 text-center">
          <div className="text-red-500 mb-4">
            <GitCompare className="w-12 h-12 mx-auto mb-2" />
            <p className="font-medium">Diff Generation Failed</p>
            <p className="text-sm text-gray-600 mt-2">{error}</p>
          </div>
          <Button onClick={generateDiff} variant="outline">
            Try Again
          </Button>
        </CardContent>
      </Card>
    );
  }

  if (!diffResult) return null;

  return (
    <div className="w-full h-full flex flex-col">
      {/* Header with controls */}
      <Card className="flex-shrink-0">
        <CardHeader className="pb-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <Zap className="w-5 h-5 text-blue-500" />
              <CardTitle className="text-lg">
                AST-Powered Function Diff
              </CardTitle>
              <Badge variant="outline" className="bg-blue-50 text-blue-700">
                {sourceFunction.name} → {targetFunction.name}
              </Badge>
            </div>
            {onClose && (
              <Button onClick={onClose} variant="ghost" size="sm" className="text-xl font-bold">
                ×
              </Button>
            )}
          </div>

          {/* Summary stats */}
          <div className="flex items-center gap-4 mt-4">
            <Badge variant="outline" className="bg-green-50 text-green-700">
              +{diffResult.summary.addedLines} Added
            </Badge>
            <Badge variant="outline" className="bg-red-50 text-red-700">
              -{diffResult.summary.deletedLines} Deleted
            </Badge>
            <Badge variant="outline" className="bg-yellow-50 text-yellow-700">
              ~{diffResult.summary.modifiedLines} Modified
            </Badge>
            <Badge variant="outline" className="bg-gray-50 text-gray-700">
              ={diffResult.summary.unchangedLines} Unchanged
            </Badge>
            {diffResult.summary.structuralChanges > 0 && (
              <Badge variant="outline" className="bg-purple-50 text-purple-700">
                🏗️ {diffResult.summary.structuralChanges} Structural
              </Badge>
            )}
          </div>

          {/* Controls */}
          <div className="flex items-center gap-4 mt-4">
            <Button
              variant={showUnchanged ? 'default' : 'outline'}
              size="sm"
              onClick={() => setShowUnchanged(!showUnchanged)}
              className="flex items-center gap-2"
            >
              {showUnchanged ? <Eye className="w-4 h-4" /> : <EyeOff className="w-4 h-4" />}
              {showUnchanged ? 'Hide' : 'Show'} Unchanged
            </Button>

            {diffResult.astOperations.length > 0 && (
              <Button
                variant={showASTOperations ? 'default' : 'outline'}
                size="sm"
                onClick={() => setShowASTOperations(!showASTOperations)}
                className="flex items-center gap-2"
              >
                {showASTOperations ? <ChevronDown className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
                AST Operations ({diffResult.astOperations.length})
              </Button>
            )}

            <div className="flex items-center gap-2">
              <label className="text-sm font-medium">Context:</label>
              <input
                type="range"
                min="0"
                max="10"
                value={contextLines}
                onChange={(e) => setContextLines(parseInt(e.target.value))}
                className="w-20"
              />
              <span className="text-sm text-gray-600">{contextLines} lines</span>
            </div>
          </div>
        </CardHeader>
      </Card>

      {/* AST Operations Panel */}
      {showASTOperations && diffResult.astOperations.length > 0 && (
        <Card className="flex-shrink-0">
          <CardHeader>
            <CardTitle className="text-sm flex items-center gap-2">
              <Code className="w-4 h-4" />
              AST Operations
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {diffResult.astOperations.map((op, index) => (
                <div key={index} className="flex items-center gap-3 p-2 bg-gray-50 rounded">
                  <Badge
                    variant="outline"
                    className={
                      op.impact === 'high' ? 'bg-red-50 text-red-700' :
                      op.impact === 'medium' ? 'bg-yellow-50 text-yellow-700' :
                      'bg-green-50 text-green-700'
                    }
                  >
                    {op.type}
                  </Badge>
                  <span className="font-mono text-sm text-gray-600">{op.nodeType}</span>
                  <span className="text-sm">{op.description}</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Diff content - Takes remaining space */}
      <Card className="flex-1 flex flex-col min-h-0">
        <CardContent className="p-0 flex-1 overflow-auto">
          {/* Header row */}
          <div className="sticky top-0 bg-gray-100 border-b border-gray-200 px-3 py-2 font-mono text-xs text-gray-600 flex items-center gap-2">
            <div className="min-w-[100px] flex-shrink-0">
              <div className="flex gap-1">
                <span className="w-8 text-right">Src</span>
                <span className="w-8 text-right">Tgt</span>
              </div>
            </div>
            <div className="flex-1">Code</div>
          </div>

          <div className="font-mono text-sm">
            {filteredLines.map((line, index) => (
              <div
                key={index}
                className={`flex items-start gap-3 p-3 hover:bg-gray-50 ${getLineTypeColor(line.type)}`}
              >
                <div className="flex items-center gap-2 min-w-[100px] flex-shrink-0">
                  {getLineTypeIcon(line.type)}
                  <div className="flex gap-1 text-xs text-gray-500 font-mono">
                    <span className="w-8 text-right">
                      {line.sourceLineNumber || ''}
                    </span>
                    <span className="w-8 text-right">
                      {line.targetLineNumber || ''}
                    </span>
                  </div>
                </div>

                <div className="flex-1 min-w-0">
                  {line.type === 'modified' ? (
                    <div className="space-y-1 w-full">
                      <div className="text-red-800 bg-red-200 px-3 py-1 font-mono text-sm rounded">
                        - {line.sourceContent}
                      </div>
                      <div className="text-green-800 bg-green-200 px-3 py-1 font-mono text-sm rounded">
                        + {line.targetContent}
                      </div>
                    </div>
                  ) : line.type === 'added' ? (
                    <div className="text-green-800 bg-green-200 px-3 py-1 font-mono text-sm rounded w-full">
                      + {line.targetContent}
                    </div>
                  ) : line.type === 'deleted' ? (
                    <div className="text-red-800 bg-red-200 px-3 py-1 font-mono text-sm rounded w-full">
                      - {line.sourceContent}
                    </div>
                  ) : (
                    <div className="px-3 py-1 font-mono text-sm text-gray-800 w-full">
                      {line.sourceContent || line.targetContent}
                    </div>
                  )}

                  {/* Show semantic changes if any */}
                  {line.semanticChanges && line.semanticChanges.length > 0 && (
                    <div className="mt-1 flex items-center gap-2">
                      <Info className="w-3 h-3 text-blue-500" />
                      <div className="flex gap-1">
                        {line.semanticChanges.map((change, i) => (
                          <Badge key={i} variant="outline" className="text-xs bg-blue-50 text-blue-700">
                            {change}
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                </div>

                {/* Similarity indicator */}
                {line.similarity !== undefined && line.similarity < 1.0 && (
                  <div className="text-xs text-gray-500">
                    {(line.similarity * 100).toFixed(0)}%
                  </div>
                )}
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
