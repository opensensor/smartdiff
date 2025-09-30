'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { DirectoryPicker } from '@/components/filesystem/DirectoryPicker';
import { FunctionGraphViewer } from '@/components/graph/FunctionGraphViewer';
import { BeyondCompareFunctionDiff } from '@/components/diff/BeyondCompareFunctionDiff';
import { comparisonService, ComparisonResult, ComparisonService } from '@/services/comparisonService';
import { useResponsive } from '@/hooks/useResponsive';
import { useQuery } from '@tanstack/react-query';

export default function HomePage() {
  const [sourceDirectory, setSourceDirectory] = useState('');
  const [targetDirectory, setTargetDirectory] = useState('');

  // Load saved directories on mount
  useEffect(() => {
    const savedSource = localStorage.getItem('smartdiff-source-directory');
    const savedTarget = localStorage.getItem('smartdiff-target-directory');

    if (savedSource) setSourceDirectory(savedSource);
    if (savedTarget) setTargetDirectory(savedTarget);
  }, []);
  const [activeView, setActiveView] = useState<'summary' | 'graph' | 'interactive' | 'analysis' | 'diff'>('summary');
  const [isComparing, setIsComparing] = useState(false);
  const [comparisonResult, setComparisonResult] = useState<ComparisonResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [similarityThreshold, setSimilarityThreshold] = useState(0.7);
  const [showOnlyChanges, setShowOnlyChanges] = useState(true);

  const handleSimilarityThresholdChange = async (newThreshold: number) => {
    setSimilarityThreshold(newThreshold);

    // Re-run comparison with new threshold if we have directories selected
    if (sourceDirectory && targetDirectory && !isComparing) {
      setIsComparing(true);
      setError(null);

      try {
        const result = await comparisonService.analyzeDirectories(
          sourceDirectory,
          targetDirectory,
          {
            includeHiddenFiles: false,
            functionSimilarityThreshold: newThreshold,
            enableDeepAnalysis: true
          }
        );

        setComparisonResult(result);
      } catch (err: any) {
        console.error('Re-comparison failed:', err);
        setError(err.message || 'Failed to re-analyze with new threshold.');
      } finally {
        setIsComparing(false);
      }
    }
  };

  const handleStartComparison = async () => {
    if (!sourceDirectory || !targetDirectory) {
      setError('Please select both source and target directories');
      return;
    }

    setIsComparing(true);
    setError(null);

    try {
      const result = await comparisonService.analyzeDirectories(
        sourceDirectory,
        targetDirectory,
        {
          includeHiddenFiles: false,
          functionSimilarityThreshold: similarityThreshold,
          enableDeepAnalysis: true
        }
      );

      setComparisonResult(result);
      setActiveView('summary');
    } catch (err: any) {
      console.error('Comparison failed:', err);
      setError(err.message || 'Failed to compare directories. Please try again.');
    } finally {
      setIsComparing(false);
    }
  };

  const handleReset = () => {
    setSourceDirectory('');
    setTargetDirectory('');
    localStorage.removeItem('smartdiff-source-directory');
    localStorage.removeItem('smartdiff-target-directory');
    setComparisonResult(null);
    setError(null);
    setActiveView('summary');
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <h2 className="text-lg font-semibold text-gray-900">Smart Diff</h2>
            <span className="text-sm text-gray-500">v0.1.0</span>
          </div>
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-bold text-gray-900">Advanced code comparison with graph-based function matching</h1>
            <button
              onClick={handleReset}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
            >
              New Comparison
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 p-6 overflow-auto">
        <div className="w-full max-w-[98vw] mx-auto">
              <Card>
                <CardHeader>
                  <CardTitle>Directory Comparison Setup</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Source Directory</label>
                    <div className="flex gap-2">
                      <Input
                        type="text"
                        placeholder="/path/to/source"
                        value={sourceDirectory}
                        onChange={(e) => {
                          setSourceDirectory(e.target.value);
                          localStorage.setItem('smartdiff-source-directory', e.target.value);
                        }}
                        className="flex-1"
                      />
                      <DirectoryPicker
                        value={sourceDirectory}
                        onChange={(path) => {
                          setSourceDirectory(path);
                          localStorage.setItem('smartdiff-source-directory', path);
                        }}
                        placeholder="Select source directory..."
                      >
                        <Button variant="outline">
                          📁 Browse
                        </Button>
                      </DirectoryPicker>
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Target Directory</label>
                    <div className="flex gap-2">
                      <Input
                        type="text"
                        placeholder="/path/to/target"
                        value={targetDirectory}
                        onChange={(e) => {
                          setTargetDirectory(e.target.value);
                          localStorage.setItem('smartdiff-target-directory', e.target.value);
                        }}
                        className="flex-1"
                      />
                      <DirectoryPicker
                        value={targetDirectory}
                        onChange={(path) => {
                          setTargetDirectory(path);
                          localStorage.setItem('smartdiff-target-directory', path);
                        }}
                        placeholder="Select target directory..."
                      >
                        <Button variant="outline">
                          📁 Browse
                        </Button>
                      </DirectoryPicker>
                    </div>
                  </div>

                  <div className="flex gap-2 pt-4">
                    <Button
                      onClick={handleStartComparison}
                      disabled={isComparing || !sourceDirectory || !targetDirectory}
                    >
                      {isComparing ? '⏳ Comparing...' : '▶️ Start Comparison'}
                    </Button>
                    <Button
                      variant="outline"
                      onClick={handleReset}
                      disabled={isComparing}
                    >
                      🔄 Reset
                    </Button>
                  </div>

                  {error && (
                    <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded-md text-red-700 text-sm">
                      {error}
                    </div>
                  )}
                </CardContent>
              </Card>

              <Card className="mt-6">
                <CardHeader>
                  <CardTitle>Comparison Results</CardTitle>

                  {/* Tab Navigation */}
                  <div className="flex gap-2 mt-4">
                    <Button
                      variant={activeView === 'summary' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setActiveView('summary')}
                    >
                      📊 Summary
                    </Button>
                    <Button
                      variant={activeView === 'graph' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setActiveView('graph')}
                    >
                      🕸️ D3 Graph
                    </Button>
                    <Button
                      variant={activeView === 'interactive' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setActiveView('interactive')}
                    >
                      🔗 Interactive
                    </Button>
                    <Button
                      variant={activeView === 'analysis' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setActiveView('analysis')}
                    >
                      📈 Analysis
                    </Button>
                    <Button
                      variant={activeView === 'diff' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setActiveView('diff')}
                    >
                      🔍 Diff Viewer
                    </Button>
                  </div>
                </CardHeader>
                <CardContent>
                  {activeView === 'summary' && (
                    <div>
                      {comparisonResult ? (
                        <div className="space-y-6">
                          {/* Summary Statistics */}
                          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div className="text-center p-4 bg-blue-50 rounded-lg">
                              <div className="text-2xl font-bold text-blue-600">{comparisonResult.summary.totalFiles}</div>
                              <div className="text-sm text-blue-600">Total Files</div>
                            </div>
                            <div className="text-center p-4 bg-green-50 rounded-lg">
                              <div className="text-2xl font-bold text-green-600">{comparisonResult.summary.addedFiles}</div>
                              <div className="text-sm text-green-600">Added Files</div>
                            </div>
                            <div className="text-center p-4 bg-red-50 rounded-lg">
                              <div className="text-2xl font-bold text-red-600">{comparisonResult.summary.deletedFiles}</div>
                              <div className="text-sm text-red-600">Deleted Files</div>
                            </div>
                            <div className="text-center p-4 bg-purple-50 rounded-lg">
                              <div className="text-2xl font-bold text-purple-600">{comparisonResult.summary.modifiedFiles}</div>
                              <div className="text-sm text-purple-600">Modified Files</div>
                            </div>
                          </div>

                          {/* Function Statistics */}
                          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div className="text-center p-4 bg-indigo-50 rounded-lg">
                              <div className="text-2xl font-bold text-indigo-600">{comparisonResult.summary.totalFunctions}</div>
                              <div className="text-sm text-indigo-600">Total Functions</div>
                            </div>
                            <div className="text-center p-4 bg-emerald-50 rounded-lg">
                              <div className="text-2xl font-bold text-emerald-600">{comparisonResult.summary.addedFunctions}</div>
                              <div className="text-sm text-emerald-600">Added Functions</div>
                            </div>
                            <div className="text-center p-4 bg-rose-50 rounded-lg">
                              <div className="text-2xl font-bold text-rose-600">{comparisonResult.summary.deletedFunctions}</div>
                              <div className="text-sm text-rose-600">Deleted Functions</div>
                            </div>
                            <div className="text-center p-4 bg-amber-50 rounded-lg">
                              <div className="text-2xl font-bold text-amber-600">{comparisonResult.summary.movedFunctions}</div>
                              <div className="text-sm text-amber-600">Moved/Renamed</div>
                            </div>
                          </div>

                          {/* Analysis Time and Insights */}
                          <div className="bg-gray-50 rounded-lg p-4">
                            <div className="flex items-center justify-between mb-2">
                              <span className="text-sm font-medium text-gray-700">Analysis Time</span>
                              <span className="text-sm text-gray-600">
                                {ComparisonService.formatAnalysisTime(comparisonResult.analysisTime)}
                              </span>
                            </div>
                            <div className="flex items-center justify-between">
                              <span className="text-sm font-medium text-gray-700">Overall Similarity</span>
                              <span className="text-sm text-gray-600">
                                {(ComparisonService.calculateOverallSimilarity(comparisonResult) * 100).toFixed(1)}%
                              </span>
                            </div>
                          </div>

                          {/* Function Matches */}
                          <div>
                            <div className="flex items-center justify-between mb-3">
                              <h3 className="text-lg font-semibold">
                                Function Matches ({comparisonResult.functionMatches.filter(match =>
                                  !showOnlyChanges || (match.matchType || match.type) !== 'identical'
                                ).length} of {comparisonResult.functionMatches.length} total)
                              </h3>
                              <label className="flex items-center gap-2 text-sm">
                                <input
                                  type="checkbox"
                                  checked={showOnlyChanges}
                                  onChange={(e) => setShowOnlyChanges(e.target.checked)}
                                  className="rounded"
                                />
                                Show only changes
                              </label>
                            </div>
                            <div className="max-h-96 overflow-y-auto border rounded-lg">
                              <table className="w-full">
                                <thead className="bg-gray-50 sticky top-0">
                                  <tr>
                                    <th className="text-left p-3 font-medium text-gray-700">Source Function</th>
                                    <th className="text-left p-3 font-medium text-gray-700">Target Function</th>
                                    <th className="text-center p-3 font-medium text-gray-700">Similarity</th>
                                    <th className="text-center p-3 font-medium text-gray-700">Status</th>
                                  </tr>
                                </thead>
                                <tbody>
                                  {comparisonResult.functionMatches
                                    .filter(match => !showOnlyChanges || (match.matchType || match.type) !== 'identical')
                                    .map((match, index) => (
                                    <tr key={index} className="border-t hover:bg-gray-50">
                                      <td className="p-3 font-mono text-sm">
                                        {match.sourceFunction?.name || '-'}
                                      </td>
                                      <td className="p-3 font-mono text-sm">
                                        {match.targetFunction?.name || '-'}
                                      </td>
                                      <td className="p-3 text-center text-sm text-gray-600">
                                        {(match.similarity * 100).toFixed(1)}%
                                      </td>
                                      <td className="p-3 text-center">
                                        <span className={`px-2 py-1 rounded text-xs ${
                                          (match.matchType || match.type) === 'identical' ? 'bg-gray-100 text-gray-800' :
                                          (match.matchType || match.type) === 'similar' ? 'bg-blue-100 text-blue-800' :
                                          (match.matchType || match.type) === 'renamed' ? 'bg-purple-100 text-purple-800' :
                                          (match.matchType || match.type) === 'moved' ? 'bg-indigo-100 text-indigo-800' :
                                          (match.matchType || match.type) === 'added' ? 'bg-green-100 text-green-800' :
                                          'bg-red-100 text-red-800'
                                        }`}>
                                          {match.matchType || match.type}
                                        </span>
                                      </td>
                                    </tr>
                                  ))}
                                </tbody>
                              </table>
                            </div>
                          </div>

                          {/* File Changes */}
                          <div>
                            <div className="flex items-center justify-between mb-3">
                              <h3 className="text-lg font-semibold">
                                File Changes ({comparisonResult.fileChanges.filter(change =>
                                  !showOnlyChanges || change.type !== 'unchanged'
                                ).length} of {comparisonResult.fileChanges.length} total)
                              </h3>
                            </div>
                            <div className="max-h-96 overflow-y-auto border rounded-lg">
                              <table className="w-full">
                                <thead className="bg-gray-50 sticky top-0">
                                  <tr>
                                    <th className="text-left p-3 font-medium text-gray-700">File Path</th>
                                    <th className="text-center p-3 font-medium text-gray-700">Similarity</th>
                                    <th className="text-center p-3 font-medium text-gray-700">Status</th>
                                  </tr>
                                </thead>
                                <tbody>
                                  {comparisonResult.fileChanges
                                    .filter(change => !showOnlyChanges || change.type !== 'unchanged')
                                    .map((change, index) => (
                                    <tr key={index} className="border-t hover:bg-gray-50">
                                      <td className="p-3 font-mono text-sm">
                                        {change.sourcePath || change.targetPath}
                                      </td>
                                      <td className="p-3 text-center text-sm text-gray-600">
                                        {change.similarity ? `${(change.similarity * 100).toFixed(1)}%` : '-'}
                                      </td>
                                      <td className="p-3 text-center">
                                        <span className={`px-2 py-1 rounded text-xs ${
                                          change.type === 'unchanged' ? 'bg-gray-100 text-gray-800' :
                                          change.type === 'modified' ? 'bg-blue-100 text-blue-800' :
                                          change.type === 'added' ? 'bg-green-100 text-green-800' :
                                          change.type === 'deleted' ? 'bg-red-100 text-red-800' :
                                          'bg-purple-100 text-purple-800'
                                        }`}>
                                          {change.type}
                                        </span>
                                      </td>
                                    </tr>
                                  ))}
                                </tbody>
                              </table>
                            </div>
                          </div>
                        </div>
                      ) : (
                        <div className="text-center py-12 text-gray-500">
                          <p>📋 Select directories and start comparison to see results</p>
                        </div>
                      )}
                    </div>
                  )}

                  {activeView === 'graph' && (
                    <FunctionGraphViewer
                      data={comparisonResult}
                      onNodeSelect={(node) => console.log('Node selected:', node)}
                    />
                  )}

                  {activeView === 'interactive' && (
                    <div className="text-center py-12 text-gray-500">
                      <p>🔗 Interactive graph will appear here</p>
                    </div>
                  )}

                  {activeView === 'analysis' && (
                    <div className="text-center py-12 text-gray-500">
                      <p>📈 Function analysis dashboard will appear here</p>
                    </div>
                  )}

                  {activeView === 'diff' && comparisonResult && (
                    <BeyondCompareFunctionDiff
                      functionMatches={comparisonResult.functionMatches}
                      fileChanges={comparisonResult.fileChanges}
                      summary={comparisonResult.summary}
                      onFunctionSelect={(pair) => console.log('Function pair selected:', pair)}
                      similarityThreshold={similarityThreshold}
                      onSimilarityThresholdChange={handleSimilarityThresholdChange}
                    />
                  )}
                </CardContent>
              </Card>
        </div>
      </main>
    </div>
  );
}
