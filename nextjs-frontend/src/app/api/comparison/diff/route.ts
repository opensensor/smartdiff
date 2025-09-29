import { NextRequest, NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';

interface DiffOptions {
  context?: number;
  ignoreWhitespace?: boolean;
  ignoreCase?: boolean;
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { sourceFilePath, targetFilePath, options = {} } = body;

    if (!sourceFilePath || !targetFilePath) {
      return NextResponse.json(
        { error: 'Both source and target file paths are required' },
        { status: 400 }
      );
    }

    // Security checks
    const sourceAbsolutePath = path.resolve(sourceFilePath);
    const targetAbsolutePath = path.resolve(targetFilePath);

    try {
      const [sourceStats, targetStats] = await Promise.all([
        fs.stat(sourceAbsolutePath),
        fs.stat(targetAbsolutePath)
      ]);

      if (!sourceStats.isFile() || !targetStats.isFile()) {
        return NextResponse.json(
          { error: 'Both paths must be files' },
          { status: 400 }
        );
      }

      // Check file sizes (limit to 5MB each for diff generation)
      if (sourceStats.size > 5 * 1024 * 1024 || targetStats.size > 5 * 1024 * 1024) {
        return NextResponse.json(
          { error: 'Files too large for diff generation (max 5MB each)' },
          { status: 413 }
        );
      }

      const [sourceContent, targetContent] = await Promise.all([
        fs.readFile(sourceAbsolutePath, 'utf-8'),
        fs.readFile(targetAbsolutePath, 'utf-8')
      ]);

      const diff = generateUnifiedDiff(
        sourceContent,
        targetContent,
        sourceFilePath,
        targetFilePath,
        options
      );

      const stats = calculateDiffStats(sourceContent, targetContent);

      return NextResponse.json({
        diff,
        stats,
        sourceSize: sourceStats.size,
        targetSize: targetStats.size,
        sourceModified: sourceStats.mtime.toISOString(),
        targetModified: targetStats.mtime.toISOString()
      });

    } catch (error: any) {
      if (error.code === 'ENOENT') {
        return NextResponse.json(
          { error: 'One or both files not found' },
          { status: 404 }
        );
      } else if (error.code === 'EACCES') {
        return NextResponse.json(
          { error: 'Permission denied' },
          { status: 403 }
        );
      } else {
        throw error;
      }
    }

  } catch (error) {
    console.error('Diff generation error:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

function generateUnifiedDiff(
  sourceContent: string,
  targetContent: string,
  sourceFilePath: string,
  targetFilePath: string,
  options: DiffOptions
): string {
  const sourceLines = sourceContent.split('\n');
  const targetLines = targetContent.split('\n');
  
  // Apply preprocessing based on options
  const processedSourceLines = preprocessLines(sourceLines, options);
  const processedTargetLines = preprocessLines(targetLines, options);
  
  const diff = computeLCS(processedSourceLines, processedTargetLines);
  return formatUnifiedDiff(diff, sourceFilePath, targetFilePath, options.context || 3);
}

function preprocessLines(lines: string[], options: DiffOptions): string[] {
  let processed = lines;
  
  if (options.ignoreWhitespace) {
    processed = processed.map(line => line.trim());
  }
  
  if (options.ignoreCase) {
    processed = processed.map(line => line.toLowerCase());
  }
  
  return processed;
}

interface DiffLine {
  type: 'context' | 'added' | 'deleted';
  content: string;
  sourceLineNumber?: number;
  targetLineNumber?: number;
}

function computeLCS(sourceLines: string[], targetLines: string[]): DiffLine[] {
  const m = sourceLines.length;
  const n = targetLines.length;
  
  // Create LCS table
  const lcs: number[][] = Array(m + 1).fill(null).map(() => Array(n + 1).fill(0));
  
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (sourceLines[i - 1] === targetLines[j - 1]) {
        lcs[i][j] = lcs[i - 1][j - 1] + 1;
      } else {
        lcs[i][j] = Math.max(lcs[i - 1][j], lcs[i][j - 1]);
      }
    }
  }
  
  // Backtrack to find the diff
  const diff: DiffLine[] = [];
  let i = m, j = n;
  
  while (i > 0 || j > 0) {
    if (i > 0 && j > 0 && sourceLines[i - 1] === targetLines[j - 1]) {
      diff.unshift({
        type: 'context',
        content: sourceLines[i - 1],
        sourceLineNumber: i,
        targetLineNumber: j
      });
      i--;
      j--;
    } else if (j > 0 && (i === 0 || lcs[i][j - 1] >= lcs[i - 1][j])) {
      diff.unshift({
        type: 'added',
        content: targetLines[j - 1],
        targetLineNumber: j
      });
      j--;
    } else if (i > 0) {
      diff.unshift({
        type: 'deleted',
        content: sourceLines[i - 1],
        sourceLineNumber: i
      });
      i--;
    }
  }
  
  return diff;
}

function formatUnifiedDiff(
  diff: DiffLine[],
  sourceFilePath: string,
  targetFilePath: string,
  context: number
): string {
  const lines: string[] = [];
  
  // Header
  lines.push(`--- ${sourceFilePath}`);
  lines.push(`+++ ${targetFilePath}`);
  
  // Group changes into hunks
  const hunks = groupIntoHunks(diff, context);
  
  for (const hunk of hunks) {
    // Hunk header
    const sourceStart = hunk.sourceStart;
    const sourceCount = hunk.sourceCount;
    const targetStart = hunk.targetStart;
    const targetCount = hunk.targetCount;
    
    lines.push(`@@ -${sourceStart},${sourceCount} +${targetStart},${targetCount} @@`);
    
    // Hunk content
    for (const line of hunk.lines) {
      switch (line.type) {
        case 'context':
          lines.push(` ${line.content}`);
          break;
        case 'deleted':
          lines.push(`-${line.content}`);
          break;
        case 'added':
          lines.push(`+${line.content}`);
          break;
      }
    }
  }
  
  return lines.join('\n');
}

interface Hunk {
  sourceStart: number;
  sourceCount: number;
  targetStart: number;
  targetCount: number;
  lines: DiffLine[];
}

function groupIntoHunks(diff: DiffLine[], context: number): Hunk[] {
  const hunks: Hunk[] = [];
  let currentHunk: DiffLine[] = [];
  let contextCount = 0;
  
  for (let i = 0; i < diff.length; i++) {
    const line = diff[i];
    
    if (line.type === 'context') {
      if (currentHunk.length === 0) {
        // Start of potential hunk
        contextCount++;
        if (contextCount <= context) {
          currentHunk.push(line);
        }
      } else {
        // In a hunk
        currentHunk.push(line);
        contextCount++;
        
        // Check if we should end the hunk
        if (contextCount > context * 2) {
          // Look ahead to see if there are more changes
          let hasMoreChanges = false;
          for (let j = i + 1; j < Math.min(i + context + 1, diff.length); j++) {
            if (diff[j].type !== 'context') {
              hasMoreChanges = true;
              break;
            }
          }
          
          if (!hasMoreChanges) {
            // End the hunk
            const hunk = createHunk(currentHunk.slice(0, -context));
            if (hunk.lines.length > 0) {
              hunks.push(hunk);
            }
            currentHunk = [];
            contextCount = 0;
          }
        }
      }
    } else {
      // Added or deleted line
      if (currentHunk.length === 0) {
        // Add preceding context
        const startContext = Math.max(0, i - context);
        for (let j = startContext; j < i; j++) {
          if (diff[j].type === 'context') {
            currentHunk.push(diff[j]);
          }
        }
      }
      
      currentHunk.push(line);
      contextCount = 0;
    }
  }
  
  // Add final hunk if any
  if (currentHunk.length > 0) {
    const hunk = createHunk(currentHunk);
    if (hunk.lines.length > 0) {
      hunks.push(hunk);
    }
  }
  
  return hunks;
}

function createHunk(lines: DiffLine[]): Hunk {
  if (lines.length === 0) {
    return {
      sourceStart: 0,
      sourceCount: 0,
      targetStart: 0,
      targetCount: 0,
      lines: []
    };
  }
  
  const firstLine = lines[0];
  const lastLine = lines[lines.length - 1];
  
  const sourceStart = firstLine.sourceLineNumber || 1;
  const targetStart = firstLine.targetLineNumber || 1;
  
  let sourceCount = 0;
  let targetCount = 0;
  
  for (const line of lines) {
    if (line.type === 'context' || line.type === 'deleted') {
      sourceCount++;
    }
    if (line.type === 'context' || line.type === 'added') {
      targetCount++;
    }
  }
  
  return {
    sourceStart,
    sourceCount,
    targetStart,
    targetCount,
    lines
  };
}

function calculateDiffStats(sourceContent: string, targetContent: string) {
  const sourceLines = sourceContent.split('\n');
  const targetLines = targetContent.split('\n');
  
  const diff = computeLCS(sourceLines, targetLines);
  
  const stats = {
    linesAdded: diff.filter(line => line.type === 'added').length,
    linesDeleted: diff.filter(line => line.type === 'deleted').length,
    linesChanged: 0,
    totalSourceLines: sourceLines.length,
    totalTargetLines: targetLines.length
  };
  
  stats.linesChanged = stats.linesAdded + stats.linesDeleted;
  
  return stats;
}
