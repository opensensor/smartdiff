import React, { useState } from 'react';
import { SideBySideDiffView } from '../components/SideBySideDiffView';
import { UnifiedDiffView } from '../components/UnifiedDiffView';
import { StructureView } from '../components/StructureView';
import { FunctionCentricView } from '../components/FunctionCentricView';
import {
  Columns,
  FileText,
  TreePine,
  Eye,
  Function
} from 'lucide-react';
import { clsx } from 'clsx';

// Mock data for demonstration
const mockDiffData = {
  sourceFile: {
    name: 'Calculator.java',
    content: `public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    public boolean isEven(int number) {
        return number % 2 == 0;
    }
    
    public double divide(double a, double b) {
        if (b == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        return a / b;
    }
}`,
    language: 'java',
  },
  targetFile: {
    name: 'Calculator.java',
    content: `public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
    
    public int multiply(int a, int b) {
        return a * b;
    }
    
    // Renamed method
    public boolean isNumberEven(int number) {
        return checkEvenness(number);
    }
    
    // Extracted method
    private boolean checkEvenness(int number) {
        return number % 2 == 0;
    }
    
    public double divide(double a, double b) {
        if (b == 0) {
            throw new ArithmeticException("Cannot divide by zero");
        }
        return a / b;
    }
    
    // New method
    public int subtract(int a, int b) {
        return a - b;
    }
}`,
    language: 'java',
  },
  changes: {
    source: [
      {
        type: 'removed' as const,
        startLine: 9,
        endLine: 11,
        content: 'Original isEven method removed',
      },
      {
        type: 'modified' as const,
        startLine: 13,
        endLine: 17,
        content: 'Exception type changed',
      },
    ],
    target: [
      {
        type: 'modified' as const,
        startLine: 9,
        endLine: 11,
        content: 'Method renamed to isNumberEven',
      },
      {
        type: 'added' as const,
        startLine: 13,
        endLine: 16,
        content: 'New checkEvenness method extracted',
      },
      {
        type: 'modified' as const,
        startLine: 18,
        endLine: 22,
        content: 'Exception type changed to ArithmeticException',
      },
      {
        type: 'added' as const,
        startLine: 25,
        endLine: 27,
        content: 'New subtract method added',
      },
    ],
  },
  similarity: 0.873,
};

const mockStructureData = {
  sourceFile: {
    name: 'Calculator.java',
    structure: {
      id: 'file-1',
      name: 'Calculator.java',
      type: 'file' as const,
      children: [
        {
          id: 'class-1',
          name: 'Calculator',
          type: 'class' as const,
          children: [
            {
              id: 'method-1',
              name: 'add',
              type: 'function' as const,
              metadata: {
                lineNumber: 2,
                parameters: ['int a', 'int b'],
                returnType: 'int',
                complexity: 1,
              },
            },
            {
              id: 'method-2',
              name: 'multiply',
              type: 'function' as const,
              metadata: {
                lineNumber: 6,
                parameters: ['int a', 'int b'],
                returnType: 'int',
                complexity: 1,
              },
            },
            {
              id: 'method-3',
              name: 'isEven',
              type: 'function' as const,
              changeType: 'removed' as const,
              metadata: {
                lineNumber: 10,
                parameters: ['int number'],
                returnType: 'boolean',
                complexity: 1,
              },
            },
            {
              id: 'method-4',
              name: 'divide',
              type: 'function' as const,
              changeType: 'modified' as const,
              metadata: {
                lineNumber: 14,
                parameters: ['double a', 'double b'],
                returnType: 'double',
                complexity: 2,
              },
            },
          ],
        },
      ],
    },
  },
  targetFile: {
    name: 'Calculator.java',
    structure: {
      id: 'file-2',
      name: 'Calculator.java',
      type: 'file' as const,
      children: [
        {
          id: 'class-2',
          name: 'Calculator',
          type: 'class' as const,
          children: [
            {
              id: 'method-5',
              name: 'add',
              type: 'function' as const,
              metadata: {
                lineNumber: 2,
                parameters: ['int a', 'int b'],
                returnType: 'int',
                complexity: 1,
              },
            },
            {
              id: 'method-6',
              name: 'multiply',
              type: 'function' as const,
              metadata: {
                lineNumber: 6,
                parameters: ['int a', 'int b'],
                returnType: 'int',
                complexity: 1,
              },
            },
            {
              id: 'method-7',
              name: 'isNumberEven',
              type: 'function' as const,
              changeType: 'renamed' as const,
              metadata: {
                lineNumber: 10,
                parameters: ['int number'],
                returnType: 'boolean',
                complexity: 1,
              },
            },
            {
              id: 'method-8',
              name: 'checkEvenness',
              type: 'function' as const,
              changeType: 'added' as const,
              metadata: {
                lineNumber: 15,
                parameters: ['int number'],
                returnType: 'boolean',
                complexity: 1,
              },
            },
            {
              id: 'method-9',
              name: 'divide',
              type: 'function' as const,
              changeType: 'modified' as const,
              metadata: {
                lineNumber: 19,
                parameters: ['double a', 'double b'],
                returnType: 'double',
                complexity: 2,
              },
            },
            {
              id: 'method-10',
              name: 'subtract',
              type: 'function' as const,
              changeType: 'added' as const,
              metadata: {
                lineNumber: 26,
                parameters: ['int a', 'int b'],
                returnType: 'int',
                complexity: 1,
              },
            },
          ],
        },
      ],
    },
  },
  matches: [
    { sourceId: 'method-1', targetId: 'method-5', similarity: 1.0, changeType: 'unchanged' },
    { sourceId: 'method-2', targetId: 'method-6', similarity: 1.0, changeType: 'unchanged' },
    { sourceId: 'method-3', targetId: 'method-7', similarity: 0.75, changeType: 'renamed' },
    { sourceId: 'method-4', targetId: 'method-9', similarity: 0.85, changeType: 'modified' },
  ],
};

// Mock function-centric data
const mockFunctionData = {
  sourceFile: 'Calculator.java',
  targetFile: 'Calculator.java',
  language: 'java',
  functions: [
    {
      id: 'func-1',
      sourceFunction: {
        name: 'add',
        signature: 'public int add(int a, int b)',
        body: 'public int add(int a, int b) {\n    return a + b;\n}',
        startLine: 2,
        endLine: 4,
        complexity: 1,
        parameters: ['int a', 'int b'],
        returnType: 'int',
      },
      targetFunction: {
        name: 'add',
        signature: 'public int add(int a, int b)',
        body: 'public int add(int a, int b) {\n    return a + b;\n}',
        startLine: 2,
        endLine: 4,
        complexity: 1,
        parameters: ['int a', 'int b'],
        returnType: 'int',
      },
      similarity: {
        overall: 1.0,
        signature: 1.0,
        body: 1.0,
        context: 1.0,
      },
      changeType: 'unchanged' as const,
    },
    {
      id: 'func-2',
      sourceFunction: {
        name: 'isEven',
        signature: 'public boolean isEven(int number)',
        body: 'public boolean isEven(int number) {\n    return number % 2 == 0;\n}',
        startLine: 10,
        endLine: 12,
        complexity: 1,
        parameters: ['int number'],
        returnType: 'boolean',
      },
      targetFunction: {
        name: 'isNumberEven',
        signature: 'public boolean isNumberEven(int number)',
        body: 'public boolean isNumberEven(int number) {\n    return checkEvenness(number);\n}',
        startLine: 10,
        endLine: 12,
        complexity: 1,
        parameters: ['int number'],
        returnType: 'boolean',
      },
      similarity: {
        overall: 0.75,
        signature: 0.85,
        body: 0.60,
        context: 0.80,
      },
      changeType: 'renamed' as const,
      refactoringPattern: {
        type: 'Extract Method',
        description: 'Logic extracted to checkEvenness method',
        confidence: 0.92,
      },
    },
    {
      id: 'func-3',
      sourceFunction: {
        name: 'divide',
        signature: 'public double divide(double a, double b)',
        body: 'public double divide(double a, double b) {\n    if (b == 0) {\n        throw new IllegalArgumentException("Division by zero");\n    }\n    return a / b;\n}',
        startLine: 14,
        endLine: 19,
        complexity: 2,
        parameters: ['double a', 'double b'],
        returnType: 'double',
      },
      targetFunction: {
        name: 'divide',
        signature: 'public double divide(double a, double b)',
        body: 'public double divide(double a, double b) {\n    if (b == 0) {\n        throw new ArithmeticException("Cannot divide by zero");\n    }\n    return a / b;\n}',
        startLine: 19,
        endLine: 24,
        complexity: 2,
        parameters: ['double a', 'double b'],
        returnType: 'double',
      },
      similarity: {
        overall: 0.85,
        signature: 1.0,
        body: 0.80,
        context: 0.75,
      },
      changeType: 'modified' as const,
    },
  ],
  summary: {
    totalFunctions: 4,
    matchedFunctions: 3,
    averageSimilarity: 0.87,
    refactoringPatterns: 1,
  },
};

export const DemoPage: React.FC = () => {
  const [activeView, setActiveView] = useState<'side-by-side' | 'unified' | 'structure' | 'function-centric'>('side-by-side');

  const views = [
    {
      id: 'side-by-side' as const,
      name: 'Side by Side',
      icon: Columns,
      description: 'Compare files side by side with synchronized scrolling',
    },
    {
      id: 'unified' as const,
      name: 'Unified Diff',
      icon: FileText,
      description: 'Traditional unified diff view with context lines',
    },
    {
      id: 'structure' as const,
      name: 'Structure View',
      icon: TreePine,
      description: 'Compare code structure and function-level changes',
    },
    {
      id: 'function-centric' as const,
      name: 'Function Analysis',
      icon: Function,
      description: 'Detailed function-level analysis with similarity scores and refactoring patterns',
    },
  ];

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Code Visualization Demo
        </h1>
        <p className="text-gray-600">
          Explore different ways to visualize code differences with our advanced comparison tools.
        </p>
      </div>

      {/* View Selector */}
      <div className="mb-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {views.map((view) => {
            const Icon = view.icon;
            return (
              <button
                key={view.id}
                onClick={() => setActiveView(view.id)}
                className={clsx(
                  'p-4 rounded-lg border-2 text-left transition-all',
                  activeView === view.id
                    ? 'border-primary-500 bg-primary-50'
                    : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                )}
              >
                <div className="flex items-center mb-2">
                  <Icon className={clsx(
                    'h-5 w-5 mr-2',
                    activeView === view.id ? 'text-primary-600' : 'text-gray-600'
                  )} />
                  <span className={clsx(
                    'font-medium',
                    activeView === view.id ? 'text-primary-900' : 'text-gray-900'
                  )}>
                    {view.name}
                  </span>
                </div>
                <p className={clsx(
                  'text-sm',
                  activeView === view.id ? 'text-primary-700' : 'text-gray-600'
                )}>
                  {view.description}
                </p>
              </button>
            );
          })}
        </div>
      </div>

      {/* Active View */}
      <div className="mb-8">
        {activeView === 'side-by-side' && (
          <SideBySideDiffView diffData={mockDiffData} />
        )}

        {activeView === 'unified' && (
          <UnifiedDiffView diffData={mockDiffData} />
        )}

        {activeView === 'structure' && (
          <StructureView structureData={mockStructureData} />
        )}

        {activeView === 'function-centric' && (
          <FunctionCentricView data={mockFunctionData} />
        )}
      </div>

      {/* Features Overview */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="card p-6">
          <div className="flex items-center mb-3">
            <Eye className="h-5 w-5 text-primary-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Smart Highlighting
            </h3>
          </div>
          <p className="text-gray-600 text-sm">
            Intelligent syntax highlighting with change-specific colors for added, 
            removed, and modified code sections.
          </p>
        </div>

        <div className="card p-6">
          <div className="flex items-center mb-3">
            <Columns className="h-5 w-5 text-primary-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Synchronized Views
            </h3>
          </div>
          <p className="text-gray-600 text-sm">
            Side-by-side comparison with synchronized scrolling and navigation 
            to easily track changes across files.
          </p>
        </div>

        <div className="card p-6">
          <div className="flex items-center mb-3">
            <TreePine className="h-5 w-5 text-primary-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Structure Analysis
            </h3>
          </div>
          <p className="text-gray-600 text-sm">
            Hierarchical view of code structure showing function-level changes,
            complexity metrics, and similarity scores.
          </p>
        </div>

        <div className="card p-6">
          <div className="flex items-center mb-3">
            <Function className="h-5 w-5 text-primary-600 mr-2" />
            <h3 className="text-lg font-medium text-gray-900">
              Function Analysis
            </h3>
          </div>
          <p className="text-gray-600 text-sm">
            Detailed function-level comparison with similarity breakdowns,
            refactoring pattern detection, and change classification.
          </p>
        </div>
      </div>
    </div>
  );
};
