import React, { useRef, useEffect } from 'react';
import Editor, { Monaco } from '@monaco-editor/react';
import { clsx } from 'clsx';

export interface CodeChange {
  type: 'added' | 'removed' | 'modified';
  startLine: number;
  endLine: number;
  content?: string;
}

interface CodeEditorProps {
  value: string;
  language: string;
  theme?: 'light' | 'dark';
  readOnly?: boolean;
  showLineNumbers?: boolean;
  changes?: CodeChange[];
  onSelectionChange?: (selection: any) => void;
  className?: string;
  height?: string;
}

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  language,
  theme = 'light',
  readOnly = true,
  showLineNumbers = true,
  changes = [],
  onSelectionChange,
  className,
  height = '400px',
}) => {
  const editorRef = useRef<any>(null);
  const monacoRef = useRef<Monaco | null>(null);

  const handleEditorDidMount = (editor: any, monaco: Monaco) => {
    editorRef.current = editor;
    monacoRef.current = monaco;

    // Configure editor options
    editor.updateOptions({
      readOnly,
      lineNumbers: showLineNumbers ? 'on' : 'off',
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      automaticLayout: true,
      wordWrap: 'on',
      fontSize: 14,
      fontFamily: 'JetBrains Mono, Fira Code, Monaco, Consolas, monospace',
    });

    // Apply change decorations
    if (changes.length > 0) {
      applyChangeDecorations(editor, monaco, changes);
    }

    // Handle selection changes
    if (onSelectionChange) {
      editor.onDidChangeCursorSelection((e: any) => {
        onSelectionChange(e.selection);
      });
    }
  };

  const applyChangeDecorations = (editor: any, monaco: Monaco, changes: CodeChange[]) => {
    const decorations = changes.map(change => {
      let className = '';
      let glyphMarginClassName = '';
      
      switch (change.type) {
        case 'added':
          className = 'code-line-added';
          glyphMarginClassName = 'code-glyph-added';
          break;
        case 'removed':
          className = 'code-line-removed';
          glyphMarginClassName = 'code-glyph-removed';
          break;
        case 'modified':
          className = 'code-line-modified';
          glyphMarginClassName = 'code-glyph-modified';
          break;
      }

      return {
        range: new monaco.Range(change.startLine, 1, change.endLine, 1),
        options: {
          isWholeLine: true,
          className,
          glyphMarginClassName,
          glyphMarginHoverMessage: {
            value: `**${change.type.toUpperCase()}**${change.content ? `\n\n${change.content}` : ''}`,
          },
        },
      };
    });

    editor.deltaDecorations([], decorations);
  };

  useEffect(() => {
    if (editorRef.current && monacoRef.current && changes.length > 0) {
      applyChangeDecorations(editorRef.current, monacoRef.current, changes);
    }
  }, [changes]);

  // Define custom CSS for change highlighting
  useEffect(() => {
    const style = document.createElement('style');
    style.textContent = `
      .code-line-added {
        background-color: rgba(34, 197, 94, 0.1) !important;
        border-left: 3px solid #22c55e !important;
      }
      .code-line-removed {
        background-color: rgba(239, 68, 68, 0.1) !important;
        border-left: 3px solid #ef4444 !important;
      }
      .code-line-modified {
        background-color: rgba(251, 191, 36, 0.1) !important;
        border-left: 3px solid #fbbf24 !important;
      }
      .code-glyph-added::before {
        content: '+';
        color: #22c55e;
        font-weight: bold;
      }
      .code-glyph-removed::before {
        content: '-';
        color: #ef4444;
        font-weight: bold;
      }
      .code-glyph-modified::before {
        content: '~';
        color: #fbbf24;
        font-weight: bold;
      }
    `;
    document.head.appendChild(style);

    return () => {
      document.head.removeChild(style);
    };
  }, []);

  return (
    <div className={clsx('border border-gray-300 rounded-lg overflow-hidden', className)}>
      <Editor
        height={height}
        language={language}
        value={value}
        theme={theme === 'dark' ? 'vs-dark' : 'vs'}
        onMount={handleEditorDidMount}
        options={{
          readOnly,
          lineNumbers: showLineNumbers ? 'on' : 'off',
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
          automaticLayout: true,
          wordWrap: 'on',
          fontSize: 14,
          fontFamily: 'JetBrains Mono, Fira Code, Monaco, Consolas, monospace',
          glyphMargin: true,
          folding: true,
          lineDecorationsWidth: 10,
          lineNumbersMinChars: 3,
          renderLineHighlight: 'line',
          selectOnLineNumbers: true,
          roundedSelection: false,
          cursorStyle: 'line',
          automaticLayout: true,
        }}
      />
    </div>
  );
};
