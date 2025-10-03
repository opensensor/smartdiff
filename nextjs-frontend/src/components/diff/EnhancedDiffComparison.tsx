'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { FolderOpen, Play, RotateCcw, GitCompare, List, Grid } from 'lucide-react';
import { DirectoryPicker } from '@/components/filesystem/DirectoryPicker';
import { FunctionCentricDiffView } from './FunctionCentricDiffView';
import { BeyondCompareFunctionDiff } from './BeyondCompareFunctionDiff';
import { comparisonService, FunctionMatch } from '@/services/comparisonService';

export function EnhancedDiffComparison() {
  const [sourceDirectory, setSourceDirectory] = useState('');
  const [targetDirectory, setTargetDirectory] = useState('');
  const [isComparing, setIsComparing] = useState(false);
  const [functionMatches, setFunctionMatches] = useState<FunctionMatch[]>([]);
  const [summary, setSummary] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<'function-centric' | 'file-centric'>('function-centric');

  const handleCompare = async () => {
    if (!sourceDirectory || !targetDirectory) {
      return;
    }

    setIsComparing(true);
    setError(null);

    try {
      // Call Rust backend with advanced AST-based matching
      const result = await comparisonService.analyzeDirectories(
        sourceDirectory,
        targetDirectory,
        {
          functionSimilarityThreshold: 0.7,
          fileExtensions: ['js', 'ts', 'jsx', 'tsx', 'py', 'rs', 'java', 'cpp', 'c', 'h'],
        }
      );

      setSummary(result.summary);
      setFunctionMatches(result.functionMatches);
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
    setFunctionMatches([]);
    setSummary(null);
    setError(null);
  };

  const handleFunctionSelect = (match: FunctionMatch) => {
    console.log('Function selected:', match);
    // TODO: Open detailed diff view
  };

  return (
    <div className="p-6 space-y-6">
      {/* Configuration Panel */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <GitCompare className="w-5 h-5" />
            Enhanced Directory Comparison (AST-Powered)
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

          {error && (
            <div className="p-4 bg-red-50 border border-red-200 rounded-md text-red-800">
              {error}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Results Panel */}
      {summary && functionMatches.length > 0 && (
        <>
          {/* Summary Stats */}
          <Card>
            <CardHeader>
              <CardTitle>Comparison Summary</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-6 gap-4">
                <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                    {summary.totalFunctions || 0}
                  </div>
                  <div className="text-sm text-blue-600 dark:text-blue-400">Total Functions</div>
                </div>
                <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                    {summary.addedFunctions || 0}
                  </div>
                  <div className="text-sm text-green-600 dark:text-green-400">Added</div>
                </div>
                <div className="bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-red-600 dark:text-red-400">
                    {summary.deletedFunctions || 0}
                  </div>
                  <div className="text-sm text-red-600 dark:text-red-400">Deleted</div>
                </div>
                <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
                    {summary.modifiedFunctions || 0}
                  </div>
                  <div className="text-sm text-yellow-600 dark:text-yellow-400">Modified</div>
                </div>
                <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
                    {summary.renamedFunctions || 0}
                  </div>
                  <div className="text-sm text-purple-600 dark:text-purple-400">Renamed</div>
                </div>
                <div className="bg-indigo-50 dark:bg-indigo-900/20 p-4 rounded-lg">
                  <div className="text-2xl font-bold text-indigo-600 dark:text-indigo-400">
                    {summary.movedFunctions || 0}
                  </div>
                  <div className="text-sm text-indigo-600 dark:text-indigo-400">Moved</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* View Mode Toggle */}
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center gap-2">
                <span className="text-sm font-medium text-gray-700">View Mode:</span>
                <Button
                  variant={viewMode === 'function-centric' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setViewMode('function-centric')}
                >
                  <List className="w-4 h-4 mr-2" />
                  Function-Centric (Sorted by Change)
                </Button>
                <Button
                  variant={viewMode === 'file-centric' ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setViewMode('file-centric')}
                >
                  <Grid className="w-4 h-4 mr-2" />
                  File-Centric (Grouped by File)
                </Button>
              </div>
            </CardContent>
          </Card>

          {/* Function Diff View */}
          {viewMode === 'function-centric' ? (
            <FunctionCentricDiffView
              functionMatches={functionMatches}
              onFunctionSelect={handleFunctionSelect}
            />
          ) : (
            <BeyondCompareFunctionDiff
              functionMatches={functionMatches}
              summary={summary}
              onFunctionSelect={handleFunctionSelect}
            />
          )}
        </>
      )}

      {/* Empty State */}
      {!summary && !isComparing && (
        <Card>
          <CardContent className="py-12">
            <div className="text-center text-gray-500">
              <FolderOpen className="w-16 h-16 mx-auto mb-4 text-gray-400" />
              <p className="text-lg font-medium mb-2">No Comparison Yet</p>
              <p className="text-sm">
                Select source and target directories above and click "Start Comparison" to begin.
              </p>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

