import React, { useState, useRef, useEffect } from 'react';
import { CodeEditor, CodeChange } from './CodeEditor';
import { 
  ChevronLeft, 
  ChevronRight, 
  RotateCcw, 
  ZoomIn, 
  ZoomOut,
  Maximize2,
  Minimize2
} from 'lucide-react';
import { clsx } from 'clsx';

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

interface SideBySideDiffViewProps {
  diffData: DiffData;
  className?: string;
}

export const SideBySideDiffView: React.FC<SideBySideDiffViewProps> = ({
  diffData,
  className,
}) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [syncScroll, setSyncScroll] = useState(true);
  const [fontSize, setFontSize] = useState(14);
  const [selectedChange, setSelectedChange] = useState<number | null>(null);
  
  const sourceEditorRef = useRef<any>(null);
  const targetEditorRef = useRef<any>(null);

  const handleScrollSync = (editor: any, otherEditor: any) => {
    if (!syncScroll) return;

    const scrollTop = editor.getScrollTop();
    const scrollLeft = editor.getScrollLeft();
    
    otherEditor.setScrollTop(scrollTop);
    otherEditor.setScrollLeft(scrollLeft);
  };

  const navigateToChange = (changeIndex: number) => {
    setSelectedChange(changeIndex);
    
    const sourceChange = diffData.changes.source[changeIndex];
    const targetChange = diffData.changes.target[changeIndex];
    
    if (sourceChange && sourceEditorRef.current) {
      sourceEditorRef.current.revealLineInCenter(sourceChange.startLine);
    }
    
    if (targetChange && targetEditorRef.current) {
      targetEditorRef.current.revealLineInCenter(targetChange.startLine);
    }
  };

  const nextChange = () => {
    const totalChanges = Math.max(
      diffData.changes.source.length,
      diffData.changes.target.length
    );
    
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
    
    if (sourceEditorRef.current) {
      sourceEditorRef.current.setScrollTop(0);
      sourceEditorRef.current.setScrollLeft(0);
    }
    
    if (targetEditorRef.current) {
      targetEditorRef.current.setScrollTop(0);
      targetEditorRef.current.setScrollLeft(0);
    }
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

  const totalChanges = Math.max(
    diffData.changes.source.length,
    diffData.changes.target.length
  );

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
              {diffData.sourceFile.name}
            </span>
            <span className="text-gray-400">↔</span>
            <span className="text-sm font-medium text-gray-700">
              {diffData.targetFile.name}
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

          {/* Action Controls */}
          <div className="flex items-center space-x-1">
            <button
              onClick={resetView}
              className="p-1 text-gray-400 hover:text-gray-600"
              title="Reset view"
            >
              <RotateCcw className="h-4 w-4" />
            </button>
            
            <label className="flex items-center text-xs text-gray-600">
              <input
                type="checkbox"
                checked={syncScroll}
                onChange={(e) => setSyncScroll(e.target.checked)}
                className="mr-1 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
              />
              Sync
            </label>
            
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
      <div className="grid grid-cols-2 divide-x divide-gray-200">
        {/* Source File */}
        <div className="flex flex-col">
          <div className="px-4 py-2 bg-gray-50 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium text-gray-700">
                Source: {diffData.sourceFile.name}
              </span>
              <span className="text-xs text-gray-500">
                {diffData.changes.source.length} changes
              </span>
            </div>
          </div>
          
          <div className="flex-1">
            <CodeEditor
              value={diffData.sourceFile.content}
              language={diffData.sourceFile.language}
              changes={diffData.changes.source}
              height={isFullscreen ? 'calc(100vh - 140px)' : '600px'}
              onSelectionChange={(selection) => {
                if (syncScroll && targetEditorRef.current) {
                  // Sync selection if needed
                }
              }}
            />
          </div>
        </div>

        {/* Target File */}
        <div className="flex flex-col">
          <div className="px-4 py-2 bg-gray-50 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium text-gray-700">
                Target: {diffData.targetFile.name}
              </span>
              <span className="text-xs text-gray-500">
                {diffData.changes.target.length} changes
              </span>
            </div>
          </div>
          
          <div className="flex-1">
            <CodeEditor
              value={diffData.targetFile.content}
              language={diffData.targetFile.language}
              changes={diffData.changes.target}
              height={isFullscreen ? 'calc(100vh - 140px)' : '600px'}
              onSelectionChange={(selection) => {
                if (syncScroll && sourceEditorRef.current) {
                  // Sync selection if needed
                }
              }}
            />
          </div>
        </div>
      </div>

      {/* Change Summary */}
      {totalChanges > 0 && (
        <div className="p-4 border-t border-gray-200 bg-gray-50">
          <div className="flex items-center justify-between text-sm">
            <div className="flex items-center space-x-4">
              <span className="text-gray-600">
                Changes: {totalChanges}
              </span>
              <div className="flex items-center space-x-2">
                <div className="flex items-center">
                  <div className="w-3 h-3 bg-success-500 rounded mr-1"></div>
                  <span className="text-gray-600">
                    {diffData.changes.source.filter(c => c.type === 'added').length + 
                     diffData.changes.target.filter(c => c.type === 'added').length} added
                  </span>
                </div>
                <div className="flex items-center">
                  <div className="w-3 h-3 bg-danger-500 rounded mr-1"></div>
                  <span className="text-gray-600">
                    {diffData.changes.source.filter(c => c.type === 'removed').length + 
                     diffData.changes.target.filter(c => c.type === 'removed').length} removed
                  </span>
                </div>
                <div className="flex items-center">
                  <div className="w-3 h-3 bg-warning-500 rounded mr-1"></div>
                  <span className="text-gray-600">
                    {diffData.changes.source.filter(c => c.type === 'modified').length + 
                     diffData.changes.target.filter(c => c.type === 'modified').length} modified
                  </span>
                </div>
              </div>
            </div>
            
            <div className="text-gray-500">
              Use ← → to navigate changes
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
