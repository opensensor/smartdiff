import React, { useState, useMemo } from 'react';
import { CodeEditor } from './CodeEditor';
import { 
  Function, 
  ArrowRight, 
  BarChart3, 
  Target, 
  Shuffle,
  Filter,
  Search,
  ChevronDown,
  ChevronRight,
  Info
} from 'lucide-react';
import { clsx } from 'clsx';

interface FunctionMatch {
  id: string;
  sourceFunction: {
    name: string;
    signature: string;
    body: string;
    startLine: number;
    endLine: number;
    complexity: number;
    parameters: string[];
    returnType: string;
  };
  targetFunction?: {
    name: string;
    signature: string;
    body: string;
    startLine: number;
    endLine: number;
    complexity: number;
    parameters: string[];
    returnType: string;
  };
  similarity: {
    overall: number;
    signature: number;
    body: number;
    context: number;
  };
  changeType: 'unchanged' | 'modified' | 'renamed' | 'moved' | 'added' | 'removed';
  refactoringPattern?: {
    type: string;
    description: string;
    confidence: number;
  };
}

interface FunctionCentricData {
  sourceFile: string;
  targetFile: string;
  language: string;
  functions: FunctionMatch[];
  summary: {
    totalFunctions: number;
    matchedFunctions: number;
    averageSimilarity: number;
    refactoringPatterns: number;
  };
}

interface FunctionCentricViewProps {
  data: FunctionCentricData;
  className?: string;
}

export const FunctionCentricView: React.FC<FunctionCentricViewProps> = ({
  data,
  className,
}) => {
  const [selectedFunction, setSelectedFunction] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<string>('all');
  const [sortBy, setSortBy] = useState<'name' | 'similarity' | 'complexity'>('similarity');
  const [searchQuery, setSearchQuery] = useState('');
  const [expandedFunctions, setExpandedFunctions] = useState<Set<string>>(new Set());

  // Filter and sort functions
  const filteredFunctions = useMemo(() => {
    let filtered = data.functions.filter(func => {
      // Filter by change type
      if (filterType !== 'all' && func.changeType !== filterType) {
        return false;
      }
      
      // Filter by search query
      if (searchQuery && !func.sourceFunction.name.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false;
      }
      
      return true;
    });

    // Sort functions
    filtered.sort((a, b) => {
      switch (sortBy) {
        case 'name':
          return a.sourceFunction.name.localeCompare(b.sourceFunction.name);
        case 'similarity':
          return b.similarity.overall - a.similarity.overall;
        case 'complexity':
          return b.sourceFunction.complexity - a.sourceFunction.complexity;
        default:
          return 0;
      }
    });

    return filtered;
  }, [data.functions, filterType, sortBy, searchQuery]);

  const toggleFunction = (functionId: string) => {
    const newExpanded = new Set(expandedFunctions);
    if (newExpanded.has(functionId)) {
      newExpanded.delete(functionId);
    } else {
      newExpanded.add(functionId);
    }
    setExpandedFunctions(newExpanded);
  };

  const getSimilarityColor = (similarity: number) => {
    if (similarity >= 0.9) return 'text-success-700 bg-success-100';
    if (similarity >= 0.7) return 'text-success-600 bg-success-50';
    if (similarity >= 0.5) return 'text-warning-600 bg-warning-50';
    if (similarity >= 0.3) return 'text-danger-600 bg-danger-50';
    return 'text-gray-600 bg-gray-50';
  };

  const getChangeTypeColor = (changeType: string) => {
    switch (changeType) {
      case 'unchanged':
        return 'text-gray-600 bg-gray-100';
      case 'modified':
        return 'text-warning-600 bg-warning-100';
      case 'renamed':
        return 'text-blue-600 bg-blue-100';
      case 'moved':
        return 'text-purple-600 bg-purple-100';
      case 'added':
        return 'text-success-600 bg-success-100';
      case 'removed':
        return 'text-danger-600 bg-danger-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const getChangeTypeIcon = (changeType: string) => {
    switch (changeType) {
      case 'renamed':
        return <Shuffle className="h-4 w-4" />;
      case 'moved':
        return <ArrowRight className="h-4 w-4" />;
      default:
        return <Function className="h-4 w-4" />;
    }
  };

  const renderFunctionCard = (func: FunctionMatch) => {
    const isExpanded = expandedFunctions.has(func.id);
    const isSelected = selectedFunction === func.id;

    return (
      <div
        key={func.id}
        className={clsx(
          'border rounded-lg transition-all',
          isSelected ? 'border-primary-500 shadow-md' : 'border-gray-200 hover:border-gray-300'
        )}
      >
        {/* Function Header */}
        <div
          className="p-4 cursor-pointer"
          onClick={() => setSelectedFunction(func.id)}
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  toggleFunction(func.id);
                }}
                className="p-1 hover:bg-gray-100 rounded"
              >
                {isExpanded ? (
                  <ChevronDown className="h-4 w-4" />
                ) : (
                  <ChevronRight className="h-4 w-4" />
                )}
              </button>

              <div className="flex items-center space-x-2">
                {getChangeTypeIcon(func.changeType)}
                <span className="font-medium text-gray-900">
                  {func.sourceFunction.name}
                </span>
                {func.targetFunction && func.targetFunction.name !== func.sourceFunction.name && (
                  <>
                    <ArrowRight className="h-3 w-3 text-gray-400" />
                    <span className="font-medium text-gray-700">
                      {func.targetFunction.name}
                    </span>
                  </>
                )}
              </div>
            </div>

            <div className="flex items-center space-x-2">
              {/* Similarity Score */}
              <div className={clsx(
                'px-2 py-1 rounded-full text-xs font-medium',
                getSimilarityColor(func.similarity.overall)
              )}>
                {(func.similarity.overall * 100).toFixed(0)}%
              </div>

              {/* Change Type */}
              <div className={clsx(
                'px-2 py-1 rounded-full text-xs font-medium',
                getChangeTypeColor(func.changeType)
              )}>
                {func.changeType}
              </div>

              {/* Complexity */}
              <div className="flex items-center text-xs text-gray-500">
                <BarChart3 className="h-3 w-3 mr-1" />
                {func.sourceFunction.complexity}
              </div>
            </div>
          </div>

          {/* Function Signature */}
          <div className="mt-2 text-sm text-gray-600 font-mono">
            {func.sourceFunction.signature}
          </div>

          {/* Refactoring Pattern */}
          {func.refactoringPattern && (
            <div className="mt-2 flex items-center text-xs text-blue-600">
              <Target className="h-3 w-3 mr-1" />
              <span>
                {func.refactoringPattern.type}: {func.refactoringPattern.description}
              </span>
              <span className="ml-2 text-gray-500">
                ({(func.refactoringPattern.confidence * 100).toFixed(0)}% confidence)
              </span>
            </div>
          )}
        </div>

        {/* Expanded Content */}
        {isExpanded && (
          <div className="border-t border-gray-200">
            {/* Similarity Breakdown */}
            <div className="p-4 bg-gray-50">
              <h4 className="text-sm font-medium text-gray-900 mb-3">
                Similarity Breakdown
              </h4>
              <div className="grid grid-cols-3 gap-4 text-sm">
                <div>
                  <div className="text-gray-600">Signature</div>
                  <div className={clsx(
                    'font-medium',
                    func.similarity.signature >= 0.8 ? 'text-success-600' : 
                    func.similarity.signature >= 0.5 ? 'text-warning-600' : 'text-danger-600'
                  )}>
                    {(func.similarity.signature * 100).toFixed(0)}%
                  </div>
                </div>
                <div>
                  <div className="text-gray-600">Body</div>
                  <div className={clsx(
                    'font-medium',
                    func.similarity.body >= 0.8 ? 'text-success-600' : 
                    func.similarity.body >= 0.5 ? 'text-warning-600' : 'text-danger-600'
                  )}>
                    {(func.similarity.body * 100).toFixed(0)}%
                  </div>
                </div>
                <div>
                  <div className="text-gray-600">Context</div>
                  <div className={clsx(
                    'font-medium',
                    func.similarity.context >= 0.8 ? 'text-success-600' : 
                    func.similarity.context >= 0.5 ? 'text-warning-600' : 'text-danger-600'
                  )}>
                    {(func.similarity.context * 100).toFixed(0)}%
                  </div>
                </div>
              </div>
            </div>

            {/* Function Comparison */}
            {func.targetFunction && (
              <div className="p-4">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                  {/* Source Function */}
                  <div>
                    <h5 className="text-sm font-medium text-gray-900 mb-2">
                      Source ({data.sourceFile})
                    </h5>
                    <CodeEditor
                      value={func.sourceFunction.body}
                      language={data.language}
                      height="200px"
                      showLineNumbers={true}
                    />
                  </div>

                  {/* Target Function */}
                  <div>
                    <h5 className="text-sm font-medium text-gray-900 mb-2">
                      Target ({data.targetFile})
                    </h5>
                    <CodeEditor
                      value={func.targetFunction.body}
                      language={data.language}
                      height="200px"
                      showLineNumbers={true}
                    />
                  </div>
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    );
  };

  const filterOptions = [
    { value: 'all', label: 'All Functions' },
    { value: 'modified', label: 'Modified' },
    { value: 'renamed', label: 'Renamed' },
    { value: 'moved', label: 'Moved' },
    { value: 'added', label: 'Added' },
    { value: 'removed', label: 'Removed' },
  ];

  const sortOptions = [
    { value: 'similarity', label: 'Similarity' },
    { value: 'name', label: 'Name' },
    { value: 'complexity', label: 'Complexity' },
  ];

  return (
    <div className={clsx('bg-white rounded-lg border border-gray-200', className)}>
      {/* Header */}
      <div className="p-6 border-b border-gray-200">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-xl font-bold text-gray-900">
              Function-Centric Analysis
            </h2>
            <p className="text-gray-600 mt-1">
              {data.sourceFile} → {data.targetFile}
            </p>
          </div>

          <div className="flex items-center space-x-4 text-sm">
            <div className="text-center">
              <div className="text-2xl font-bold text-primary-600">
                {data.summary.matchedFunctions}
              </div>
              <div className="text-gray-600">Matched</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900">
                {data.summary.totalFunctions}
              </div>
              <div className="text-gray-600">Total</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-success-600">
                {(data.summary.averageSimilarity * 100).toFixed(0)}%
              </div>
              <div className="text-gray-600">Avg Similarity</div>
            </div>
          </div>
        </div>

        {/* Controls */}
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search functions..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              />
            </div>

            {/* Filter */}
            <div className="flex items-center space-x-2">
              <Filter className="h-4 w-4 text-gray-400" />
              <select
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              >
                {filterOptions.map(option => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>

            {/* Sort */}
            <div className="flex items-center space-x-2">
              <span className="text-sm text-gray-600">Sort by:</span>
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as any)}
                className="border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              >
                {sortOptions.map(option => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="text-sm text-gray-600">
            Showing {filteredFunctions.length} of {data.functions.length} functions
          </div>
        </div>
      </div>

      {/* Function List */}
      <div className="p-6">
        <div className="space-y-4 max-h-96 overflow-y-auto">
          {filteredFunctions.map(func => renderFunctionCard(func))}
        </div>

        {filteredFunctions.length === 0 && (
          <div className="text-center py-8 text-gray-500">
            <Info className="h-8 w-8 mx-auto mb-2" />
            <p>No functions match the current filters.</p>
          </div>
        )}
      </div>
    </div>
  );
};
