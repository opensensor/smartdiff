'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { DirectoryPicker } from '@/components/filesystem/DirectoryPicker';
import { InteractiveDiffViewer } from '@/components/diff/InteractiveDiffViewer';
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
          functionSimilarityThreshold: 0.3,
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
    setComparisonResult(null);
    setError(null);
    setActiveView('summary');
  };

  return (
    <div className="min-h-screen bg-gray-50" style={{backgroundColor: '#f9fafb'}}>
      <div className="flex h-screen">
        {/* Sidebar */}
        <div className="w-64 bg-white border-r border-gray-200 flex flex-col" style={{width: '256px', backgroundColor: 'white', borderRight: '1px solid #e5e7eb', display: 'flex', flexDirection: 'column'}}>
          <div className="p-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">Smart Diff</h2>
            <p className="text-sm text-gray-600">v0.1.0</p>
          </div>

          <nav className="flex-1 p-4">
            <ul className="space-y-2">
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>🏠</span>
                  Home
                </a>
              </li>
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>📁</span>
                  File Explorer
                </a>
              </li>
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>🔄</span>
                  Diff Viewer
                </a>
              </li>
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>🕸️</span>
                  Dependency Graph
                </a>
              </li>
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>📊</span>
                  Analysis
                </a>
              </li>
              <li>
                <a href="#" className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:bg-gray-100">
                  <span>⚙️</span>
                  Settings
                </a>
              </li>
            </ul>
          </nav>
        </div>

        {/* Main Content */}
        <div className="flex-1 flex flex-col">
          <header className="bg-white border-b border-gray-200 px-6 py-4">
            <div className="flex items-center justify-between">
              <div>
                <h1 className="text-2xl font-bold text-gray-900">Diff</h1>
                <p className="text-sm text-gray-600">
                  Advanced code comparison with graph-based function matching
                </p>
              </div>
              <div className="flex items-center gap-4">
                <button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors">
                  New Comparison
                </button>
              </div>
            </div>
          </header>

          <main className="flex-1 p-6 overflow-auto">
            <div className="max-w-4xl mx-auto">
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
                            <h3 className="text-lg font-semibold mb-3">Function Matches</h3>
                            <div className="space-y-2">
                              {comparisonResult.functionMatches.slice(0, 10).map((match, index) => (
                                <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                                  <div className="flex items-center gap-3">
                                    <span className="font-mono text-sm">
                                      {match.sourceFunction?.name || 'N/A'}
                                    </span>
                                    <span className="text-gray-400">→</span>
                                    <span className="font-mono text-sm">
                                      {match.targetFunction?.name || 'N/A'}
                                    </span>
                                  </div>
                                  <div className="flex items-center gap-2">
                                    <span className="text-sm text-gray-600">
                                      {(match.similarity * 100).toFixed(1)}% similar
                                    </span>
                                    <span className={`px-2 py-1 rounded text-xs ${
                                      match.type === 'identical' ? 'bg-green-100 text-green-800' :
                                      match.type === 'similar' ? 'bg-blue-100 text-blue-800' :
                                      match.type === 'renamed' ? 'bg-yellow-100 text-yellow-800' :
                                      match.type === 'moved' ? 'bg-purple-100 text-purple-800' :
                                      match.type === 'added' ? 'bg-emerald-100 text-emerald-800' :
                                      'bg-red-100 text-red-800'
                                    }`}>
                                      {match.type}
                                    </span>
                                  </div>
                                </div>
                              ))}
                            </div>
                          </div>

                          {/* File Changes */}
                          <div>
                            <h3 className="text-lg font-semibold mb-3">File Changes</h3>
                            <div className="space-y-2">
                              {comparisonResult.fileChanges.slice(0, 10).map((change, index) => (
                                <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                                  <div className="flex items-center gap-3">
                                    <span className="font-mono text-sm">
                                      {change.sourcePath || change.targetPath}
                                    </span>
                                  </div>
                                  <div className="flex items-center gap-2">
                                    {change.similarity && (
                                      <span className="text-sm text-gray-600">
                                        {(change.similarity * 100).toFixed(1)}% similar
                                      </span>
                                    )}
                                    <span className={`px-2 py-1 rounded text-xs ${
                                      change.type === 'unchanged' ? 'bg-gray-100 text-gray-800' :
                                      change.type === 'modified' ? 'bg-blue-100 text-blue-800' :
                                      change.type === 'added' ? 'bg-green-100 text-green-800' :
                                      change.type === 'deleted' ? 'bg-red-100 text-red-800' :
                                      'bg-purple-100 text-purple-800'
                                    }`}>
                                      {change.type}
                                    </span>
                                  </div>
                                </div>
                              ))}
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
                    <div className="text-center py-12 text-gray-500">
                      <p>🕸️ D3.js graph visualization will appear here</p>
                    </div>
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

                  {activeView === 'diff' && (
                    <InteractiveDiffViewer
                      data={comparisonResult}
                      onFunctionSelect={(functionId) => console.log('Function selected:', functionId)}
                    />
                  )}
                </CardContent>
              </Card>
            </div>
          </main>
        </div>
      </div>


    </div>
  );
}
