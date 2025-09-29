import React, { useState } from 'react';
import { 
  Upload, 
  Search, 
  FileText, 
  BarChart3, 
  Network,
  Code,
  AlertCircle,
  CheckCircle,
  Loader2,
  Download
} from 'lucide-react';
import { clsx } from 'clsx';

interface AnalysisSettings {
  includeComplexity: boolean;
  includeDependencies: boolean;
  includeSignatures: boolean;
  recursive: boolean;
  language: string;
}

export const AnalyzePage: React.FC = () => {
  const [selectedFiles, setSelectedFiles] = useState<FileList | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [analysisResult, setAnalysisResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<AnalysisSettings>({
    includeComplexity: true,
    includeDependencies: true,
    includeSignatures: false,
    recursive: true,
    language: 'auto',
  });

  const handleFileUpload = (files: FileList) => {
    setSelectedFiles(files);
    setError(null);
  };

  const handleAnalyze = async () => {
    if (!selectedFiles || selectedFiles.length === 0) {
      setError('Please select files to analyze');
      return;
    }

    setIsAnalyzing(true);
    setError(null);

    try {
      // Simulate API call - replace with actual API integration
      await new Promise(resolve => setTimeout(resolve, 3000));
      
      // Mock analysis result
      setAnalysisResult({
        summary: {
          totalFiles: selectedFiles.length,
          totalLines: 2847,
          totalFunctions: 156,
          totalClasses: 23,
          averageComplexity: 4.2,
          languages: ['JavaScript', 'TypeScript']
        },
        files: Array.from(selectedFiles).map((file, index) => ({
          name: file.name,
          language: 'JavaScript',
          lines: Math.floor(Math.random() * 500) + 50,
          functions: Math.floor(Math.random() * 20) + 5,
          classes: Math.floor(Math.random() * 5),
          complexity: {
            cyclomatic: Math.floor(Math.random() * 10) + 1,
            cognitive: Math.floor(Math.random() * 15) + 2,
          },
          dependencies: Math.floor(Math.random() * 8) + 2,
          issues: Math.floor(Math.random() * 3),
        })),
        complexity: {
          distribution: {
            low: 45,
            medium: 32,
            high: 18,
            veryHigh: 5
          },
          topComplexFunctions: [
            { name: 'processUserData', complexity: 12, file: 'user.js' },
            { name: 'validateForm', complexity: 10, file: 'form.js' },
            { name: 'calculateMetrics', complexity: 9, file: 'metrics.js' },
          ]
        },
        dependencies: {
          internal: 24,
          external: 12,
          circular: 2,
          topDependencies: [
            { name: 'lodash', count: 8 },
            { name: 'react', count: 6 },
            { name: 'axios', count: 4 },
          ]
        }
      });
    } catch (err) {
      setError('Failed to analyze files. Please try again.');
    } finally {
      setIsAnalyzing(false);
    }
  };

  const FileUploadArea: React.FC = () => {
    const handleDrop = (e: React.DragEvent) => {
      e.preventDefault();
      const files = e.dataTransfer.files;
      if (files.length > 0) {
        handleFileUpload(files);
      }
    };

    const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (files && files.length > 0) {
        handleFileUpload(files);
      }
    };

    return (
      <div
        className={clsx(
          'border-2 border-dashed rounded-lg p-8 text-center transition-colors',
          selectedFiles && selectedFiles.length > 0
            ? 'border-success-300 bg-success-50' 
            : 'border-gray-300 hover:border-primary-400 hover:bg-primary-50'
        )}
        onDrop={handleDrop}
        onDragOver={(e) => e.preventDefault()}
      >
        <input
          type="file"
          id="files"
          className="hidden"
          multiple
          accept=".java,.py,.js,.jsx,.ts,.tsx,.cpp,.c,.h,.hpp"
          onChange={handleFileInput}
        />
        
        {selectedFiles && selectedFiles.length > 0 ? (
          <div className="space-y-2">
            <CheckCircle className="h-12 w-12 text-success-500 mx-auto" />
            <p className="text-sm font-medium text-success-700">
              {selectedFiles.length} file{selectedFiles.length > 1 ? 's' : ''} selected
            </p>
            <div className="text-xs text-success-600 space-y-1">
              {Array.from(selectedFiles).slice(0, 3).map((file, index) => (
                <div key={index}>{file.name}</div>
              ))}
              {selectedFiles.length > 3 && (
                <div>... and {selectedFiles.length - 3} more</div>
              )}
            </div>
            <label
              htmlFor="files"
              className="btn-sm btn-outline cursor-pointer"
            >
              Change Files
            </label>
          </div>
        ) : (
          <div className="space-y-2">
            <Upload className="h-12 w-12 text-gray-400 mx-auto" />
            <p className="text-sm font-medium text-gray-700">
              Upload files or directory
            </p>
            <p className="text-xs text-gray-500">
              Drag and drop or{' '}
              <label
                htmlFor="files"
                className="text-primary-600 hover:text-primary-700 cursor-pointer underline"
              >
                browse files
              </label>
            </p>
            <p className="text-xs text-gray-400">
              Supports: .java, .py, .js, .jsx, .ts, .tsx, .cpp, .c, .h, .hpp
            </p>
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Code Analysis
        </h1>
        <p className="text-gray-600">
          Analyze code files to extract metrics, complexity, dependencies, and function signatures.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* File Upload Section */}
        <div className="lg:col-span-2 space-y-6">
          <div>
            <h3 className="text-lg font-medium text-gray-900 mb-3">
              Select Files
            </h3>
            <FileUploadArea />
          </div>

          {/* Error Display */}
          {error && (
            <div className="flex items-center p-4 bg-danger-50 border border-danger-200 rounded-lg">
              <AlertCircle className="h-5 w-5 text-danger-500 mr-3" />
              <p className="text-danger-700">{error}</p>
            </div>
          )}

          {/* Analyze Button */}
          <div className="flex justify-center">
            <button
              onClick={handleAnalyze}
              disabled={!selectedFiles || selectedFiles.length === 0 || isAnalyzing}
              className={clsx(
                'btn-lg inline-flex items-center',
                (!selectedFiles || selectedFiles.length === 0 || isAnalyzing)
                  ? 'btn-secondary cursor-not-allowed'
                  : 'btn-primary'
              )}
            >
              {isAnalyzing ? (
                <>
                  <Loader2 className="h-5 w-5 mr-2 animate-spin" />
                  Analyzing...
                </>
              ) : (
                <>
                  <Search className="h-5 w-5 mr-2" />
                  Analyze Files
                </>
              )}
            </button>
          </div>
        </div>

        {/* Settings Panel */}
        <div className="card p-6">
          <div className="flex items-center mb-4">
            <BarChart3 className="h-5 w-5 text-gray-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Analysis Settings
            </h3>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Language
              </label>
              <select
                value={settings.language}
                onChange={(e) => setSettings({
                  ...settings,
                  language: e.target.value
                })}
                className="input w-full"
              >
                <option value="auto">Auto-detect</option>
                <option value="java">Java</option>
                <option value="python">Python</option>
                <option value="javascript">JavaScript</option>
                <option value="typescript">TypeScript</option>
                <option value="cpp">C++</option>
                <option value="c">C</option>
              </select>
            </div>

            <div className="space-y-3">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.includeComplexity}
                  onChange={(e) => setSettings({
                    ...settings,
                    includeComplexity: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Include complexity metrics
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.includeDependencies}
                  onChange={(e) => setSettings({
                    ...settings,
                    includeDependencies: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Analyze dependencies
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.includeSignatures}
                  onChange={(e) => setSettings({
                    ...settings,
                    includeSignatures: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Extract function signatures
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.recursive}
                  onChange={(e) => setSettings({
                    ...settings,
                    recursive: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Recursive analysis
                </span>
              </label>
            </div>
          </div>
        </div>
      </div>

      {/* Results Section */}
      {analysisResult && (
        <div className="mt-12">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-bold text-gray-900">
              Analysis Results
            </h2>
            <button className="btn-outline btn-sm inline-flex items-center">
              <Download className="h-4 w-4 mr-2" />
              Export Report
            </button>
          </div>

          {/* Summary Cards */}
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4 mb-8">
            <div className="card p-4 text-center">
              <FileText className="h-6 w-6 text-primary-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.totalFiles}
              </div>
              <div className="text-xs text-gray-600">Files</div>
            </div>
            <div className="card p-4 text-center">
              <Code className="h-6 w-6 text-blue-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.totalLines.toLocaleString()}
              </div>
              <div className="text-xs text-gray-600">Lines</div>
            </div>
            <div className="card p-4 text-center">
              <Network className="h-6 w-6 text-green-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.totalFunctions}
              </div>
              <div className="text-xs text-gray-600">Functions</div>
            </div>
            <div className="card p-4 text-center">
              <BarChart3 className="h-6 w-6 text-purple-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.totalClasses}
              </div>
              <div className="text-xs text-gray-600">Classes</div>
            </div>
            <div className="card p-4 text-center">
              <AlertCircle className="h-6 w-6 text-warning-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.averageComplexity}
              </div>
              <div className="text-xs text-gray-600">Avg Complexity</div>
            </div>
            <div className="card p-4 text-center">
              <CheckCircle className="h-6 w-6 text-success-600 mx-auto mb-2" />
              <div className="text-lg font-bold text-gray-900">
                {analysisResult.summary.languages.length}
              </div>
              <div className="text-xs text-gray-600">Languages</div>
            </div>
          </div>

          {/* Detailed Results */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* File Details */}
            <div className="card p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                File Analysis
              </h3>
              <div className="space-y-3 max-h-96 overflow-y-auto">
                {analysisResult.files.map((file: any, index: number) => (
                  <div key={index} className="border border-gray-200 rounded-lg p-3">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-sm font-medium text-gray-900">
                        {file.name}
                      </span>
                      <span className="text-xs text-gray-500">
                        {file.language}
                      </span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 text-xs text-gray-600">
                      <div>{file.lines} lines</div>
                      <div>{file.functions} functions</div>
                      <div>Complexity: {file.complexity.cyclomatic}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Complexity Analysis */}
            <div className="card p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                Complexity Distribution
              </h3>
              <div className="space-y-4">
                <div className="space-y-2">
                  {Object.entries(analysisResult.complexity.distribution).map(([level, count]) => (
                    <div key={level} className="flex items-center justify-between">
                      <span className="text-sm text-gray-700 capitalize">{level}</span>
                      <div className="flex items-center space-x-2">
                        <div className="w-24 bg-gray-200 rounded-full h-2">
                          <div 
                            className={clsx(
                              'h-2 rounded-full',
                              level === 'low' && 'bg-success-500',
                              level === 'medium' && 'bg-warning-500',
                              level === 'high' && 'bg-danger-500',
                              level === 'veryHigh' && 'bg-red-700'
                            )}
                            style={{ width: `${(count as number / 100) * 100}%` }}
                          ></div>
                        </div>
                        <span className="text-sm font-medium text-gray-900 w-8">
                          {count}%
                        </span>
                      </div>
                    </div>
                  ))}
                </div>

                <div className="border-t pt-4">
                  <h4 className="text-sm font-medium text-gray-900 mb-2">
                    Most Complex Functions
                  </h4>
                  <div className="space-y-2">
                    {analysisResult.complexity.topComplexFunctions.map((func: any, index: number) => (
                      <div key={index} className="flex items-center justify-between text-sm">
                        <span className="text-gray-700">{func.name}</span>
                        <div className="flex items-center space-x-2">
                          <span className="text-xs text-gray-500">{func.file}</span>
                          <span className="font-medium text-danger-600">{func.complexity}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
