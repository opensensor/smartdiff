'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { FolderOpen, Play, RotateCcw, GitCompare, BarChart3, Network, Activity } from 'lucide-react';
import { DirectoryPicker } from '@/components/filesystem/DirectoryPicker';
import { FunctionGraph } from '@/components/graph/FunctionGraph';
import { InteractiveGraph } from '@/components/graph/InteractiveGraph';
import { FunctionAnalysisDashboard } from '@/components/analysis/FunctionAnalysisDashboard';
import { api } from '@/api/client';
import { GraphMatchResult } from '@/types';

export function DiffComparison() {
  const [sourceDirectory, setSourceDirectory] = useState('');
  const [targetDirectory, setTargetDirectory] = useState('');
  const [isComparing, setIsComparing] = useState(false);
  const [comparisonResult, setComparisonResult] = useState<GraphMatchResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [activeView, setActiveView] = useState<'summary' | 'graph' | 'interactive' | 'analysis'>('summary');

  const handleCompare = async () => {
    if (!sourceDirectory || !targetDirectory) {
      return;
    }

    setIsComparing(true);
    setError(null);

    try {
      const result = await api.compareDirectories(sourceDirectory, targetDirectory, {
        recursive: true,
        includeHidden: false,
        fileExtensions: ['js', 'ts', 'jsx', 'tsx', 'py', 'rs', 'java', 'cpp', 'c', 'h']
      });
      setComparisonResult(result);
    } catch (error) {
      console.error('Comparison failed:', error);
      setError(error instanceof Error ? error.message : 'Comparison failed');
    } finally {
      setIsComparing(false);
    }
  };

  const handleReset = () => {
    setSourceDirectory('');
    setTargetDirectory('');
    setComparisonResult(null);
    setError(null);
  };

  return (
    <div className="p-6 space-y-6">
      {/* Configuration Panel */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <GitCompare className="w-5 h-5" />
            Directory Comparison Setup
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-sm font-medium text-foreground">
                Source Directory
              </label>
              <DirectoryPicker
                value={sourceDirectory}
                onChange={setSourceDirectory}
                placeholder="/path/to/source"
                allowFiles={false}
              />
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium text-foreground">
                Target Directory
              </label>
              <DirectoryPicker
                value={targetDirectory}
                onChange={setTargetDirectory}
                placeholder="/path/to/target"
                allowFiles={false}
              />
            </div>
          </div>
          
          <div className="flex gap-2">
            <Button 
              onClick={handleCompare}
              disabled={!sourceDirectory || !targetDirectory || isComparing}
              loading={isComparing}
            >
              <Play className="w-4 h-4 mr-2" />
              {isComparing ? 'Comparing...' : 'Start Comparison'}
            </Button>
            
            <Button variant="outline" onClick={handleReset}>
              <RotateCcw className="w-4 h-4 mr-2" />
              Reset
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Results Panel */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Comparison Results</CardTitle>
            {comparisonResult && (
              <div className="flex items-center gap-2">
                <Button
                  variant={activeView === 'summary' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setActiveView('summary')}
                >
                  <BarChart3 className="w-4 h-4 mr-2" />
                  Summary
                </Button>
                <Button
                  variant={activeView === 'graph' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setActiveView('graph')}
                >
                  <Network className="w-4 h-4 mr-2" />
                  D3 Graph
                </Button>
                <Button
                  variant={activeView === 'interactive' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setActiveView('interactive')}
                >
                  <GitCompare className="w-4 h-4 mr-2" />
                  Interactive
                </Button>
                <Button
                  variant={activeView === 'analysis' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setActiveView('analysis')}
                >
                  <Activity className="w-4 h-4 mr-2" />
                  Analysis
                </Button>
              </div>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="bg-destructive/10 border border-destructive/20 rounded-lg p-4 mb-4">
              <p className="text-destructive font-medium">Error</p>
              <p className="text-sm text-destructive/80">{error}</p>
            </div>
          )}

          {!comparisonResult && !error && (
            <div className="flex items-center justify-center h-64 text-muted-foreground">
              <div className="text-center">
                <GitCompare className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>Select directories and start comparison to see results</p>
              </div>
            </div>
          )}

          {comparisonResult && (
            <div className="space-y-6">
              {activeView === 'summary' && (
                <>
                  {/* Summary Stats */}
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                      <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                        {comparisonResult.additions.length}
                      </div>
                      <div className="text-sm text-green-600 dark:text-green-400">Added Functions</div>
                    </div>
                    <div className="bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
                      <div className="text-2xl font-bold text-red-600 dark:text-red-400">
                        {comparisonResult.deletions.length}
                      </div>
                      <div className="text-sm text-red-600 dark:text-red-400">Removed Functions</div>
                    </div>
                    <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg">
                      <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
                        {comparisonResult.matches.length}
                      </div>
                      <div className="text-sm text-yellow-600 dark:text-yellow-400">Modified Functions</div>
                    </div>
                    <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
                      <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                        {(comparisonResult.overall_similarity * 100).toFixed(1)}%
                      </div>
                      <div className="text-sm text-blue-600 dark:text-blue-400">Overall Similarity</div>
                    </div>
                  </div>

                  {/* Function Matches */}
                  {comparisonResult.matches.length > 0 && (
                    <div>
                      <h3 className="text-lg font-semibold mb-3">Function Matches</h3>
                      <div className="space-y-2 max-h-64 overflow-y-auto">
                        {comparisonResult.matches.slice(0, 10).map((match, index) => (
                          <div key={index} className="flex items-center justify-between p-3 bg-muted/50 rounded-lg">
                            <div className="flex-1">
                              <div className="font-medium">{match.source_id} → {match.target_id}</div>
                              <div className="text-sm text-muted-foreground">
                                Similarity: {(match.similarity.overall_similarity * 100).toFixed(1)}% |
                                Confidence: {(match.confidence * 100).toFixed(1)}% |
                                Type: {match.match_type}
                              </div>
                            </div>
                          </div>
                        ))}
                        {comparisonResult.matches.length > 10 && (
                          <div className="text-center text-sm text-muted-foreground">
                            ... and {comparisonResult.matches.length - 10} more matches
                          </div>
                        )}
                      </div>
                    </div>
                  )}
                </>
              )}

              {activeView === 'graph' && (
                <div className="h-[600px]">
                  <FunctionGraph
                    data={comparisonResult}
                    width={800}
                    height={600}
                    onNodeClick={(node) => console.log('Node clicked:', node)}
                  />
                </div>
              )}

              {activeView === 'interactive' && (
                <div className="h-[600px]">
                  <InteractiveGraph
                    data={comparisonResult}
                    onNodeClick={(nodeId) => console.log('Interactive node clicked:', nodeId)}
                  />
                </div>
              )}

              {activeView === 'analysis' && (
                <FunctionAnalysisDashboard
                  data={comparisonResult}
                  onFunctionSelect={(functionId) => console.log('Function selected:', functionId)}
                />
              )}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
