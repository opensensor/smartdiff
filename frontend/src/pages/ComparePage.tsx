import React, { useState } from 'react';
import { 
  Upload, 
  FileText, 
  Play, 
  Settings, 
  Download,
  AlertCircle,
  CheckCircle,
  Loader2
} from 'lucide-react';
import { clsx } from 'clsx';

interface ComparisonSettings {
  threshold: number;
  detectRefactoring: boolean;
  trackMoves: boolean;
  showSimilarity: boolean;
  includeAst: boolean;
  language: string;
}

export const ComparePage: React.FC = () => {
  const [sourceFile, setSourceFile] = useState<File | null>(null);
  const [targetFile, setTargetFile] = useState<File | null>(null);
  const [isComparing, setIsComparing] = useState(false);
  const [comparisonResult, setComparisonResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<ComparisonSettings>({
    threshold: 0.7,
    detectRefactoring: true,
    trackMoves: true,
    showSimilarity: true,
    includeAst: false,
    language: 'auto',
  });

  const handleFileUpload = (file: File, type: 'source' | 'target') => {
    if (type === 'source') {
      setSourceFile(file);
    } else {
      setTargetFile(file);
    }
    setError(null);
  };

  const handleCompare = async () => {
    if (!sourceFile || !targetFile) {
      setError('Please select both source and target files');
      return;
    }

    setIsComparing(true);
    setError(null);

    try {
      // Simulate API call - replace with actual API integration
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Mock comparison result
      setComparisonResult({
        similarity: 0.873,
        changes: [
          {
            type: 'RENAME',
            description: 'Method "calculateTotal" renamed to "computeSum"',
            confidence: 0.95,
            location: { line: 42, column: 5 }
          },
          {
            type: 'ADD',
            description: 'New method "validateInput" added',
            confidence: 1.0,
            location: { line: 15, column: 3 }
          },
          {
            type: 'MODIFY',
            description: 'Method "processData" implementation changed',
            confidence: 0.82,
            location: { line: 28, column: 5 }
          }
        ],
        refactoringPatterns: [
          {
            type: 'ExtractMethod',
            description: 'Extracted method "validateInput" from "processData"',
            confidence: 0.89,
            complexity: 'Simple'
          }
        ],
        stats: {
          functionsMatched: 24,
          totalFunctions: 27,
          linesChanged: 156,
          totalLines: 1247
        }
      });
    } catch (err) {
      setError('Failed to compare files. Please try again.');
    } finally {
      setIsComparing(false);
    }
  };

  const FileUploadArea: React.FC<{
    file: File | null;
    onFileSelect: (file: File) => void;
    label: string;
    type: 'source' | 'target';
  }> = ({ file, onFileSelect, label, type }) => {
    const handleDrop = (e: React.DragEvent) => {
      e.preventDefault();
      const droppedFile = e.dataTransfer.files[0];
      if (droppedFile) {
        onFileSelect(droppedFile);
      }
    };

    const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
      const selectedFile = e.target.files?.[0];
      if (selectedFile) {
        onFileSelect(selectedFile);
      }
    };

    return (
      <div
        className={clsx(
          'border-2 border-dashed rounded-lg p-8 text-center transition-colors',
          file 
            ? 'border-success-300 bg-success-50' 
            : 'border-gray-300 hover:border-primary-400 hover:bg-primary-50'
        )}
        onDrop={handleDrop}
        onDragOver={(e) => e.preventDefault()}
      >
        <input
          type="file"
          id={`file-${type}`}
          className="hidden"
          accept=".java,.py,.js,.jsx,.cpp,.c,.h,.hpp"
          onChange={handleFileInput}
        />
        
        {file ? (
          <div className="space-y-2">
            <CheckCircle className="h-12 w-12 text-success-500 mx-auto" />
            <p className="text-sm font-medium text-success-700">{file.name}</p>
            <p className="text-xs text-success-600">
              {(file.size / 1024).toFixed(1)} KB
            </p>
            <label
              htmlFor={`file-${type}`}
              className="btn-sm btn-outline cursor-pointer"
            >
              Change File
            </label>
          </div>
        ) : (
          <div className="space-y-2">
            <Upload className="h-12 w-12 text-gray-400 mx-auto" />
            <p className="text-sm font-medium text-gray-700">{label}</p>
            <p className="text-xs text-gray-500">
              Drag and drop or{' '}
              <label
                htmlFor={`file-${type}`}
                className="text-primary-600 hover:text-primary-700 cursor-pointer underline"
              >
                browse files
              </label>
            </p>
            <p className="text-xs text-gray-400">
              Supports: .java, .py, .js, .jsx, .cpp, .c, .h, .hpp
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
          Compare Code Files
        </h1>
        <p className="text-gray-600">
          Upload two code files to perform structural comparison and detect refactoring patterns.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* File Upload Section */}
        <div className="lg:col-span-2 space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-lg font-medium text-gray-900 mb-3">
                Source File
              </h3>
              <FileUploadArea
                file={sourceFile}
                onFileSelect={(file) => handleFileUpload(file, 'source')}
                label="Upload source file"
                type="source"
              />
            </div>
            <div>
              <h3 className="text-lg font-medium text-gray-900 mb-3">
                Target File
              </h3>
              <FileUploadArea
                file={targetFile}
                onFileSelect={(file) => handleFileUpload(file, 'target')}
                label="Upload target file"
                type="target"
              />
            </div>
          </div>

          {/* Error Display */}
          {error && (
            <div className="flex items-center p-4 bg-danger-50 border border-danger-200 rounded-lg">
              <AlertCircle className="h-5 w-5 text-danger-500 mr-3" />
              <p className="text-danger-700">{error}</p>
            </div>
          )}

          {/* Compare Button */}
          <div className="flex justify-center">
            <button
              onClick={handleCompare}
              disabled={!sourceFile || !targetFile || isComparing}
              className={clsx(
                'btn-lg inline-flex items-center',
                (!sourceFile || !targetFile || isComparing)
                  ? 'btn-secondary cursor-not-allowed'
                  : 'btn-primary'
              )}
            >
              {isComparing ? (
                <>
                  <Loader2 className="h-5 w-5 mr-2 animate-spin" />
                  Comparing...
                </>
              ) : (
                <>
                  <Play className="h-5 w-5 mr-2" />
                  Compare Files
                </>
              )}
            </button>
          </div>
        </div>

        {/* Settings Panel */}
        <div className="card p-6">
          <div className="flex items-center mb-4">
            <Settings className="h-5 w-5 text-gray-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Comparison Settings
            </h3>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Similarity Threshold: {settings.threshold}
              </label>
              <input
                type="range"
                min="0.1"
                max="1.0"
                step="0.1"
                value={settings.threshold}
                onChange={(e) => setSettings({
                  ...settings,
                  threshold: parseFloat(e.target.value)
                })}
                className="w-full"
              />
            </div>

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
                <option value="cpp">C++</option>
                <option value="c">C</option>
              </select>
            </div>

            <div className="space-y-3">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.detectRefactoring}
                  onChange={(e) => setSettings({
                    ...settings,
                    detectRefactoring: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Detect refactoring patterns
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.trackMoves}
                  onChange={(e) => setSettings({
                    ...settings,
                    trackMoves: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Track cross-file moves
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.showSimilarity}
                  onChange={(e) => setSettings({
                    ...settings,
                    showSimilarity: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Show similarity scores
                </span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={settings.includeAst}
                  onChange={(e) => setSettings({
                    ...settings,
                    includeAst: e.target.checked
                  })}
                  className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                />
                <span className="ml-2 text-sm text-gray-700">
                  Include AST in output
                </span>
              </label>
            </div>
          </div>
        </div>
      </div>

      {/* Results Section */}
      {comparisonResult && (
        <div className="mt-12">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-bold text-gray-900">
              Comparison Results
            </h2>
            <button className="btn-outline btn-sm inline-flex items-center">
              <Download className="h-4 w-4 mr-2" />
              Export Results
            </button>
          </div>

          {/* Summary Cards */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
            <div className="card p-4 text-center">
              <div className="text-2xl font-bold text-success-600 mb-1">
                {(comparisonResult.similarity * 100).toFixed(1)}%
              </div>
              <div className="text-sm text-gray-600">Similarity</div>
            </div>
            <div className="card p-4 text-center">
              <div className="text-2xl font-bold text-primary-600 mb-1">
                {comparisonResult.changes.length}
              </div>
              <div className="text-sm text-gray-600">Changes</div>
            </div>
            <div className="card p-4 text-center">
              <div className="text-2xl font-bold text-warning-600 mb-1">
                {comparisonResult.refactoringPatterns.length}
              </div>
              <div className="text-sm text-gray-600">Refactorings</div>
            </div>
            <div className="card p-4 text-center">
              <div className="text-2xl font-bold text-gray-600 mb-1">
                {comparisonResult.stats.functionsMatched}/{comparisonResult.stats.totalFunctions}
              </div>
              <div className="text-sm text-gray-600">Functions Matched</div>
            </div>
          </div>

          {/* Detailed Results */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Changes */}
            <div className="card p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                Detected Changes
              </h3>
              <div className="space-y-3">
                {comparisonResult.changes.map((change: any, index: number) => (
                  <div key={index} className="border-l-4 border-primary-500 pl-4 py-2">
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-sm font-medium text-primary-600">
                        {change.type}
                      </span>
                      <span className="text-xs text-gray-500">
                        {(change.confidence * 100).toFixed(0)}% confidence
                      </span>
                    </div>
                    <p className="text-sm text-gray-700">{change.description}</p>
                    <p className="text-xs text-gray-500">
                      Line {change.location.line}, Column {change.location.column}
                    </p>
                  </div>
                ))}
              </div>
            </div>

            {/* Refactoring Patterns */}
            <div className="card p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                Refactoring Patterns
              </h3>
              <div className="space-y-3">
                {comparisonResult.refactoringPatterns.map((pattern: any, index: number) => (
                  <div key={index} className="border-l-4 border-success-500 pl-4 py-2">
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-sm font-medium text-success-600">
                        {pattern.type}
                      </span>
                      <span className="text-xs text-gray-500">
                        {pattern.complexity}
                      </span>
                    </div>
                    <p className="text-sm text-gray-700">{pattern.description}</p>
                    <p className="text-xs text-gray-500">
                      {(pattern.confidence * 100).toFixed(0)}% confidence
                    </p>
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
