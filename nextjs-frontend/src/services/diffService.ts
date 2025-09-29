export interface DiffLine {
  lineNumber: number;
  content: string;
  type: 'added' | 'removed' | 'unchanged' | 'modified';
  oldLineNumber?: number;
  newLineNumber?: number;
}

export interface FileDiff {
  sourcePath: string;
  targetPath: string;
  sourceContent: string;
  targetContent: string;
  lines: DiffLine[];
  stats: {
    additions: number;
    deletions: number;
    modifications: number;
  };
}

export interface DiffOptions {
  contextLines?: number;
  ignoreWhitespace?: boolean;
  caseSensitive?: boolean;
}

class DiffService {
  async getFileDiff(
    sourcePath: string, 
    targetPath: string, 
    options: DiffOptions = {}
  ): Promise<FileDiff> {
    const response = await fetch('/api/comparison/diff', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        sourceFilePath: sourcePath,
        targetFilePath: targetPath,
        options,
      }),
    });

    if (!response.ok) {
      throw new Error(`Failed to get diff: ${response.statusText}`);
    }

    return response.json();
  }

  async getFileContent(filePath: string): Promise<string> {
    const response = await fetch('/api/comparison/file-content', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ filePath }),
    });

    if (!response.ok) {
      throw new Error(`Failed to get file content: ${response.statusText}`);
    }

    const data = await response.json();
    return data.content;
  }

  // Generate diff lines from two strings using a simple LCS-based algorithm
  generateDiffLines(sourceContent: string, targetContent: string): DiffLine[] {
    const sourceLines = sourceContent.split('\n');
    const targetLines = targetContent.split('\n');
    
    const diffLines: DiffLine[] = [];
    let sourceIndex = 0;
    let targetIndex = 0;
    let lineNumber = 1;

    // Simple diff algorithm - can be enhanced with proper LCS
    while (sourceIndex < sourceLines.length || targetIndex < targetLines.length) {
      const sourceLine = sourceLines[sourceIndex];
      const targetLine = targetLines[targetIndex];

      if (sourceIndex >= sourceLines.length) {
        // Only target lines left (additions)
        diffLines.push({
          lineNumber: lineNumber++,
          content: targetLine,
          type: 'added',
          newLineNumber: targetIndex + 1,
        });
        targetIndex++;
      } else if (targetIndex >= targetLines.length) {
        // Only source lines left (deletions)
        diffLines.push({
          lineNumber: lineNumber++,
          content: sourceLine,
          type: 'removed',
          oldLineNumber: sourceIndex + 1,
        });
        sourceIndex++;
      } else if (sourceLine === targetLine) {
        // Lines are identical
        diffLines.push({
          lineNumber: lineNumber++,
          content: sourceLine,
          type: 'unchanged',
          oldLineNumber: sourceIndex + 1,
          newLineNumber: targetIndex + 1,
        });
        sourceIndex++;
        targetIndex++;
      } else {
        // Lines are different - check if it's a modification or add/remove
        const nextSourceMatch = targetLines.slice(targetIndex + 1).findIndex(line => line === sourceLine);
        const nextTargetMatch = sourceLines.slice(sourceIndex + 1).findIndex(line => line === targetLine);

        if (nextSourceMatch === -1 && nextTargetMatch === -1) {
          // Likely a modification
          diffLines.push({
            lineNumber: lineNumber++,
            content: sourceLine,
            type: 'removed',
            oldLineNumber: sourceIndex + 1,
          });
          diffLines.push({
            lineNumber: lineNumber++,
            content: targetLine,
            type: 'added',
            newLineNumber: targetIndex + 1,
          });
          sourceIndex++;
          targetIndex++;
        } else if (nextSourceMatch !== -1 && (nextTargetMatch === -1 || nextSourceMatch < nextTargetMatch)) {
          // Source line appears later in target, so target lines are additions
          diffLines.push({
            lineNumber: lineNumber++,
            content: targetLine,
            type: 'added',
            newLineNumber: targetIndex + 1,
          });
          targetIndex++;
        } else {
          // Target line appears later in source, so source line is deletion
          diffLines.push({
            lineNumber: lineNumber++,
            content: sourceLine,
            type: 'removed',
            oldLineNumber: sourceIndex + 1,
          });
          sourceIndex++;
        }
      }
    }

    return diffLines;
  }

  calculateDiffStats(lines: DiffLine[]) {
    return lines.reduce(
      (stats, line) => {
        switch (line.type) {
          case 'added':
            stats.additions++;
            break;
          case 'removed':
            stats.deletions++;
            break;
          case 'modified':
            stats.modifications++;
            break;
        }
        return stats;
      },
      { additions: 0, deletions: 0, modifications: 0 }
    );
  }
}

export const diffService = new DiffService();
