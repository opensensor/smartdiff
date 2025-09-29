import React, { useState, useMemo } from 'react';
import { CodeEditor, CodeChange } from './CodeEditor';
import { 
  ChevronLeft, 
  ChevronRight, 
  RotateCcw, 
  ZoomIn, 
  ZoomOut,
  Maximize2,
  Minimize2,
  Eye,
  EyeOff
} from 'lucide-react';
import { clsx } from 'clsx';

interface UnifiedDiffLine {
  type: 'context' | 'added' | 'removed' | 'modified';
  sourceLineNumber?: number;
  targetLineNumber?: number;
  content: string;
  changeId?: string;
}

interface DiffData {
  sourceFile: {
    name: string;
    content: string;
    language: string;
  };
  targetFile: {
    name: string;
    content: string;
    language: string;
  };
  changes: {
    source: CodeChange[];
    target: CodeChange[];
  };
  similarity: number;
}

interface UnifiedDiffViewProps {
  diffData: DiffData;
  className?: string;
}

export const UnifiedDiffView: React.FC<UnifiedDiffViewProps> = ({
  diffData,
  className,
}) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [fontSize, setFontSize] = useState(14);
  const [contextLines, setContextLines] = useState(3);
  const [showOnlyChanges, setShowOnlyChanges] = useState(false);
  const [selectedChange, setSelectedChange] = useState<number | null>(null);

  // Generate unified diff content
  const unifiedDiff = useMemo(() => {
    const sourceLines = diffData.sourceFile.content.split('\n');
    const targetLines = diffData.targetFile.content.split('\n');
    const unifiedLines: UnifiedDiffLine[] = [];

    // Simple diff algorithm - in a real implementation, this would be more sophisticated
    const maxLines = Math.max(sourceLines.length, targetLines.length);
    
    for (let i = 0; i < maxLines; i++) {
      const sourceLine = sourceLines[i];
      const targetLine = targetLines[i];
      
      if (sourceLine === targetLine) {
        // Context line
        if (!showOnlyChanges) {
          unifiedLines.push({
            type: 'context',
            sourceLineNumber: i + 1,
            targetLineNumber: i + 1,
            content: sourceLine || '',
          });
        }
      } else {
        // Changed line
        if (sourceLine !== undefined) {
          unifiedLines.push({
            type: 'removed',
            sourceLineNumber: i + 1,
            content: sourceLine,
            changeId: `change-${i}`,
          });
        }
        
        if (targetLine !== undefined) {
          unifiedLines.push({
            type: 'added',
            targetLineNumber: i + 1,
            content: targetLine,
            changeId: `change-${i}`,
          });
        }
      }
    }

    return unifiedLines;
  }, [diffData, showOnlyChanges]);

  // Generate unified diff text for Monaco Editor
  const unifiedDiffText = useMemo(() => {
    return unifiedLines.map(line => {
      let prefix = ' ';
      if (line.type === 'added') prefix = '+';
      if (line.type === 'removed') prefix = '-';
      if (line.type === 'modified') prefix = '~';
      
      return `${prefix} ${line.content}`;
    }).join('\n');
  }, [unifiedLines]);

  // Generate changes for Monaco Editor highlighting
  const unifiedChanges = useMemo(() => {
    const changes: CodeChange[] = [];
    
    unifiedLines.forEach((line, index) => {
      if (line.type !== 'context') {
        changes.push({
          type: line.type as 'added' | 'removed' | 'modified',
          startLine: index + 1,
          endLine: index + 1,
          content: line.content,
        });
      }
    });
    
    return changes;
  }, [unifiedLines]);

  const navigateToChange = (changeIndex: number) => {
    setSelectedChange(changeIndex);
    // Find the line number for this change and scroll to it
    // Implementation would depend on the specific change tracking
  };

  const nextChange = () => {
    const totalChanges = unifiedChanges.length;
    
    if (selectedChange === null) {
      navigateToChange(0);
    } else if (selectedChange < totalChanges - 1) {
      navigateToChange(selectedChange + 1);
    }
  };

  const previousChange = () => {
    if (selectedChange === null) return;
    
    if (selectedChange > 0) {
      navigateToChange(selectedChange - 1);
    }
  };

  const resetView = () => {
    setSelectedChange(null);
    setFontSize(14);
    setContextLines(3);
    setShowOnlyChanges(false);
  };

  const adjustFontSize = (delta: number) => {
    const newSize = Math.max(10, Math.min(24, fontSize + delta));
    setFontSize(newSize);
  };

  const toggleFullscreen = () => {
    setIsFullscreen(!isFullscreen);
  };

  const getSimilarityColor = (similarity: number) => {
    if (similarity >= 0.8) return 'text-success-600 bg-success-100';
    if (similarity >= 0.6) return 'text-warning-600 bg-warning-100';
    return 'text-danger-600 bg-danger-100';
  };

  const totalChanges = unifiedChanges.length;
  const addedLines = unifiedChanges.filter(c => c.type === 'added').length;
  const removedLines = unifiedChanges.filter(c => c.type === 'removed').length;
  const modifiedLines = unifiedChanges.filter(c => c.type === 'modified').length;

  return (
    <div className={clsx(
      'bg-white rounded-lg border border-gray-200',
      isFullscreen && 'fixed inset-0 z-50 rounded-none',
      className
    )}>
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200">
        <div className="flex items-center space-x-4">
          <div className="flex items-center space-x-2">
            <span className="text-sm font-medium text-gray-700">
              Unified Diff: {diffData.sourceFile.name} → {diffData.targetFile.name}
            </span>
          </div>
          
          <div className={clsx(
            'px-2 py-1 rounded-full text-xs font-medium',
            getSimilarityColor(diffData.similarity)
          )}>
            {(diffData.similarity * 100).toFixed(1)}% similar
          </div>
        </div>

        <div className="flex items-center space-x-2">
          {/* Navigation Controls */}
          <div className="flex items-center space-x-1 border-r pr-2">
            <button
              onClick={previousChange}
              disabled={selectedChange === null || selectedChange === 0}
              className="p-1 text-gray-400 hover:text-gray-600 disabled:opacity-50"
              title="Previous change"
            >
              <ChevronLeft className="h-4 w-4" />
            </button>
            
            <span className="text-xs text-gray-500 px-2">
              {selectedChange !== null ? selectedChange + 1 : 0} / {totalChanges}
            </span>
            
            <button
              onClick={nextChange}
              disabled={selectedChange !== null && selectedChange >= totalChanges - 1}
              className="p-1 text-gray-400 hover:text-gray-600 disabled:opacity-50"
              title="Next change"
            >
              <ChevronRight className="h-4 w-4" />
            </button>
          </div>

          {/* View Controls */}
          <div className="flex items-center space-x-1 border-r pr-2">
            <button
              onClick={() => adjustFontSize(-2)}
              className="p-1 text-gray-400 hover:text-gray-600"
              title="Decrease font size"
            >
              <ZoomOut className="h-4 w-4" />
            </button>
            
            <span className="text-xs text-gray-500 px-1">
              {fontSize}px
            </span>
            
            <button
              onClick={() => adjustFontSize(2)}
              className="p-1 text-gray-400 hover:text-gray-600"
              title="Increase font size"
            >
              <ZoomIn className="h-4 w-4" />
            </button>
          </div>

          {/* Filter Controls */}
          <div className="flex items-center space-x-1 border-r pr-2">
            <button
              onClick={() => setShowOnlyChanges(!showOnlyChanges)}
              className={clsx(
                'p-1 transition-colors',
                showOnlyChanges 
                  ? 'text-primary-600 hover:text-primary-700' 
                  : 'text-gray-400 hover:text-gray-600'
              )}
              title={showOnlyChanges ? 'Show all lines' : 'Show only changes'}
            >
              {showOnlyChanges ? (
                <Eye className="h-4 w-4" />
              ) : (
                <EyeOff className="h-4 w-4" />
              )}
            </button>
            
            <select
              value={contextLines}
              onChange={(e) => setContextLines(parseInt(e.target.value))}
              className="text-xs border border-gray-300 rounded px-1 py-0.5"
              title="Context lines"
            >
              <option value={0}>0</option>
              <option value={1}>1</option>
              <option value={3}>3</option>
              <option value={5}>5</option>
              <option value={10}>10</option>
            </select>
          </div>

          {/* Action Controls */}
          <div className="flex items-center space-x-1">
            <button
              onClick={resetView}
              className="p-1 text-gray-400 hover:text-gray-600"
              title="Reset view"
            >
              <RotateCcw className="h-4 w-4" />
            </button>
            
            <button
              onClick={toggleFullscreen}
              className="p-1 text-gray-400 hover:text-gray-600"
              title={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
            >
              {isFullscreen ? (
                <Minimize2 className="h-4 w-4" />
              ) : (
                <Maximize2 className="h-4 w-4" />
              )}
            </button>
          </div>
        </div>
      </div>

      {/* Diff Content */}
      <div className="relative">
        <CodeEditor
          value={unifiedDiffText}
          language={diffData.sourceFile.language}
          changes={unifiedChanges}
          height={isFullscreen ? 'calc(100vh - 140px)' : '600px'}
          showLineNumbers={true}
        />
      </div>

      {/* Change Summary */}
      <div className="p-4 border-t border-gray-200 bg-gray-50">
        <div className="flex items-center justify-between text-sm">
          <div className="flex items-center space-x-4">
            <span className="text-gray-600">
              Total changes: {totalChanges}
            </span>
            <div className="flex items-center space-x-3">
              <div className="flex items-center">
                <div className="w-3 h-3 bg-success-500 rounded mr-1"></div>
                <span className="text-gray-600">{addedLines} added</span>
              </div>
              <div className="flex items-center">
                <div className="w-3 h-3 bg-danger-500 rounded mr-1"></div>
                <span className="text-gray-600">{removedLines} removed</span>
              </div>
              <div className="flex items-center">
                <div className="w-3 h-3 bg-warning-500 rounded mr-1"></div>
                <span className="text-gray-600">{modifiedLines} modified</span>
              </div>
            </div>
          </div>
          
          <div className="flex items-center space-x-4 text-gray-500">
            <span>
              {showOnlyChanges ? 'Changes only' : 'All lines'}
            </span>
            <span>
              Context: {contextLines} lines
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};
