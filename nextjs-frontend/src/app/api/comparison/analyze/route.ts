import { NextRequest, NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';
import { createHash } from 'crypto';

interface FileInfo {
  path: string;
  relativePath: string;
  size: number;
  modified: string;
  hash: string;
  language?: string;
  functions?: FunctionInfo[];
}

interface FunctionInfo {
  name: string;
  signature: string;
  startLine: number;
  endLine: number;
  content: string;
  hash: string;
  complexity?: number;
  parameters?: string[];
  returnType?: string;
}

interface ComparisonResult {
  summary: {
    totalFiles: number;
    addedFiles: number;
    deletedFiles: number;
    modifiedFiles: number;
    unchangedFiles: number;
    totalFunctions: number;
    addedFunctions: number;
    deletedFunctions: number;
    modifiedFunctions: number;
    movedFunctions: number;
  };
  fileChanges: FileChange[];
  functionMatches: FunctionMatch[];
  analysisTime: number;
}

interface FileChange {
  type: 'added' | 'deleted' | 'modified' | 'unchanged' | 'moved';
  sourcePath?: string;
  targetPath?: string;
  similarity?: number;
  sizeChange?: number;
  linesAdded?: number;
  linesDeleted?: number;
}

interface FunctionMatch {
  type: 'identical' | 'similar' | 'renamed' | 'moved' | 'added' | 'deleted';
  sourceFunction?: FunctionInfo & { filePath: string };
  targetFunction?: FunctionInfo & { filePath: string };
  similarity: number;
  changes?: {
    signatureChanged: boolean;
    bodyChanged: boolean;
    moved: boolean;
    renamed: boolean;
  };
}

export async function POST(request: NextRequest) {
  const startTime = Date.now();
  
  try {
    const body = await request.json();
    const { sourcePath, targetPath, options = {} } = body;

    if (!sourcePath || !targetPath) {
      return NextResponse.json(
        { error: 'Source and target paths are required' },
        { status: 400 }
      );
    }

    // Validate paths exist and are directories
    try {
      const [sourceStats, targetStats] = await Promise.all([
        fs.stat(sourcePath),
        fs.stat(targetPath)
      ]);

      if (!sourceStats.isDirectory() || !targetStats.isDirectory()) {
        return NextResponse.json(
          { error: 'Both paths must be directories' },
          { status: 400 }
        );
      }
    } catch (error) {
      return NextResponse.json(
        { error: 'One or both paths do not exist or are not accessible' },
        { status: 404 }
      );
    }

    // Perform the comparison
    const result = await performDirectoryComparison(sourcePath, targetPath, options);
    
    const analysisTime = Date.now() - startTime;
    result.analysisTime = analysisTime;

    return NextResponse.json(result);

  } catch (error) {
    console.error('Comparison error:', error);
    return NextResponse.json(
      { error: 'Internal server error during comparison' },
      { status: 500 }
    );
  }
}

async function performDirectoryComparison(
  sourcePath: string, 
  targetPath: string, 
  options: any
): Promise<ComparisonResult> {
  // Scan both directories
  const [sourceFiles, targetFiles] = await Promise.all([
    scanDirectory(sourcePath, sourcePath),
    scanDirectory(targetPath, targetPath)
  ]);

  // Create maps for efficient lookup
  const sourceFileMap = new Map(sourceFiles.map(f => [f.relativePath, f]));
  const targetFileMap = new Map(targetFiles.map(f => [f.relativePath, f]));

  // Analyze file changes
  const fileChanges = analyzeFileChanges(sourceFileMap, targetFileMap);
  
  // Extract and match functions
  const functionMatches = await analyzeFunctionChanges(sourceFiles, targetFiles);

  // Generate summary
  const summary = generateSummary(fileChanges, functionMatches);

  return {
    summary,
    fileChanges,
    functionMatches,
    analysisTime: 0 // Will be set by caller
  };
}

async function scanDirectory(dirPath: string, basePath: string): Promise<FileInfo[]> {
  const files: FileInfo[] = [];
  
  async function scanRecursive(currentPath: string) {
    try {
      const items = await fs.readdir(currentPath, { withFileTypes: true });
      
      for (const item of items) {
        const fullPath = path.join(currentPath, item.name);
        
        // Skip hidden files and common ignore patterns
        if (item.name.startsWith('.') || 
            item.name === 'node_modules' || 
            item.name === '__pycache__' ||
            item.name === 'target' ||
            item.name === 'build') {
          continue;
        }

        if (item.isDirectory()) {
          await scanRecursive(fullPath);
        } else if (item.isFile()) {
          const stats = await fs.stat(fullPath);
          const content = await fs.readFile(fullPath, 'utf-8').catch(() => '');
          const hash = createHash('md5').update(content).digest('hex');
          const relativePath = path.relative(basePath, fullPath);
          
          const fileInfo: FileInfo = {
            path: fullPath,
            relativePath,
            size: stats.size,
            modified: stats.mtime.toISOString(),
            hash,
            language: detectLanguage(fullPath),
            functions: await extractFunctions(content, detectLanguage(fullPath))
          };
          
          files.push(fileInfo);
        }
      }
    } catch (error) {
      console.warn(`Error scanning ${currentPath}:`, error);
    }
  }
  
  await scanRecursive(dirPath);
  return files;
}

function detectLanguage(filePath: string): string {
  const ext = path.extname(filePath).toLowerCase();
  const languageMap: Record<string, string> = {
    '.js': 'javascript',
    '.jsx': 'javascript',
    '.ts': 'typescript',
    '.tsx': 'typescript',
    '.py': 'python',
    '.java': 'java',
    '.cpp': 'cpp',
    '.c': 'c',
    '.h': 'c',
    '.cs': 'csharp',
    '.php': 'php',
    '.rb': 'ruby',
    '.go': 'go',
    '.rs': 'rust',
    '.swift': 'swift',
    '.kt': 'kotlin',
    '.scala': 'scala'
  };
  
  return languageMap[ext] || 'text';
}

async function extractFunctions(content: string, language: string): Promise<FunctionInfo[]> {
  // This is a simplified function extraction - in a real implementation,
  // you'd use proper AST parsers for each language
  const functions: FunctionInfo[] = [];
  
  if (!content || language === 'text') {
    return functions;
  }

  const lines = content.split('\n');
  
  // Improved regex patterns for different languages
  const patterns: Record<string, RegExp> = {
    javascript: /^(?:export\s+)?(?:async\s+)?function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\([^)]*\)|^(?:export\s+)?const\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*(?:async\s+)?\([^)]*\)\s*=>/,
    typescript: /^(?:export\s+)?(?:async\s+)?function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\([^)]*\)|^(?:export\s+)?const\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*(?:async\s+)?\([^)]*\)\s*=>/,
    python: /^def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\):/,
    java: /^(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\([^)]*\)\s*\{/,
    cpp: /^(?:\w+\s+)+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)\s*\{/,
    c: /^(?:\w+\s+)+([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)\s*\{/
  };

  const pattern = patterns[language];
  if (!pattern) return functions;

  // Language keywords to exclude
  const keywords = new Set([
    'if', 'else', 'for', 'while', 'do', 'switch', 'case', 'default', 'break', 'continue',
    'return', 'try', 'catch', 'finally', 'throw', 'new', 'delete', 'typeof', 'instanceof',
    'var', 'let', 'const', 'function', 'class', 'extends', 'implements', 'interface',
    'public', 'private', 'protected', 'static', 'abstract', 'final', 'override',
    'import', 'export', 'from', 'as', 'default', 'async', 'await', 'yield',
    'true', 'false', 'null', 'undefined', 'void', 'this', 'super'
  ]);

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    const match = line.match(pattern);

    if (match) {
      const functionName = match[1] || match[2];
      if (functionName && !keywords.has(functionName.toLowerCase())) {
        // Find the end of the function (simplified)
        let endLine = i;
        let braceCount = 0;
        let inFunction = false;
        
        for (let j = i; j < lines.length; j++) {
          const currentLine = lines[j];
          
          if (currentLine.includes('{')) {
            braceCount += (currentLine.match(/\{/g) || []).length;
            inFunction = true;
          }
          if (currentLine.includes('}')) {
            braceCount -= (currentLine.match(/\}/g) || []).length;
          }
          
          if (inFunction && braceCount === 0) {
            endLine = j;
            break;
          }
        }

        const functionContent = lines.slice(i, endLine + 1).join('\n');
        const functionHash = createHash('md5').update(functionContent).digest('hex');

        functions.push({
          name: functionName,
          signature: line,
          startLine: i + 1,
          endLine: endLine + 1,
          content: functionContent,
          hash: functionHash,
          complexity: calculateComplexity(functionContent),
          parameters: extractParameters(line),
          returnType: extractReturnType(line, language)
        });
      }
    }
  }

  return functions;
}

function calculateComplexity(content: string): number {
  // Simplified cyclomatic complexity calculation
  const complexityKeywords = [
    'if', 'else', 'while', 'for', 'switch', 'case', 'catch', 'try'
  ];

  const complexityOperators = ['&&', '||', '?'];

  let complexity = 1; // Base complexity

  for (const keyword of complexityKeywords) {
    // Escape special regex characters and use word boundaries for keywords
    const escapedKeyword = keyword.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const matches = content.match(new RegExp(`\\b${escapedKeyword}\\b`, 'g'));
    if (matches) {
      complexity += matches.length;
    }
  }

  for (const operator of complexityOperators) {
    // Escape special regex characters for operators
    const escapedOperator = operator.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const matches = content.match(new RegExp(escapedOperator, 'g'));
    if (matches) {
      complexity += matches.length;
    }
  }

  return complexity;
}

function extractParameters(signature: string): string[] {
  const match = signature.match(/\(([^)]*)\)/);
  if (!match || !match[1]) return [];
  
  return match[1]
    .split(',')
    .map(param => param.trim())
    .filter(param => param.length > 0);
}

function extractReturnType(signature: string, language: string): string {
  // Simplified return type extraction
  if (language === 'typescript' || language === 'java') {
    const match = signature.match(/:\s*(\w+)|(\w+)\s+\w+\s*\(/);
    return match ? (match[1] || match[2]) : 'void';
  }
  return 'unknown';
}

function analyzeFileChanges(
  sourceFiles: Map<string, FileInfo>, 
  targetFiles: Map<string, FileInfo>
): FileChange[] {
  const changes: FileChange[] = [];
  const processedTargets = new Set<string>();

  // Check for deleted and modified files
  for (const [relativePath, sourceFile] of sourceFiles) {
    const targetFile = targetFiles.get(relativePath);
    
    if (!targetFile) {
      changes.push({
        type: 'deleted',
        sourcePath: sourceFile.path // Use absolute path
      });
    } else {
      processedTargets.add(relativePath);

      if (sourceFile.hash === targetFile.hash) {
        changes.push({
          type: 'unchanged',
          sourcePath: sourceFile.path, // Use absolute path
          targetPath: targetFile.path, // Use absolute path
          similarity: 1.0
        });
      } else {
        const similarity = calculateFileSimilarity(sourceFile, targetFile);
        changes.push({
          type: 'modified',
          sourcePath: sourceFile.path, // Use absolute path
          targetPath: targetFile.path, // Use absolute path
          similarity,
          sizeChange: targetFile.size - sourceFile.size
        });
      }
    }
  }

  // Check for added files
  for (const [relativePath, targetFile] of targetFiles) {
    if (!processedTargets.has(relativePath)) {
      changes.push({
        type: 'added',
        targetPath: targetFile.path // Use absolute path
      });
    }
  }

  return changes;
}

function calculateFileSimilarity(sourceFile: FileInfo, targetFile: FileInfo): number {
  // Simple similarity based on function overlap
  if (!sourceFile.functions || !targetFile.functions) {
    return sourceFile.hash === targetFile.hash ? 1.0 : 0.0;
  }

  const sourceFunctionHashes = new Set(sourceFile.functions.map(f => f.hash));
  const targetFunctionHashes = new Set(targetFile.functions.map(f => f.hash));
  
  const intersection = new Set([...sourceFunctionHashes].filter(h => targetFunctionHashes.has(h)));
  const union = new Set([...sourceFunctionHashes, ...targetFunctionHashes]);
  
  return union.size > 0 ? intersection.size / union.size : 0.0;
}

async function analyzeFunctionChanges(
  sourceFiles: FileInfo[], 
  targetFiles: FileInfo[]
): Promise<FunctionMatch[]> {
  const matches: FunctionMatch[] = [];
  
  // Collect all functions with their file paths
  const sourceFunctions = sourceFiles.flatMap(file => 
    (file.functions || []).map(func => ({ ...func, filePath: file.relativePath }))
  );
  
  const targetFunctions = targetFiles.flatMap(file => 
    (file.functions || []).map(func => ({ ...func, filePath: file.relativePath }))
  );

  const matchedTargets = new Set<string>();

  // Find matches for source functions
  for (const sourceFunc of sourceFunctions) {
    let bestMatch: any = null;
    let bestSimilarity = 0;

    for (const targetFunc of targetFunctions) {
      if (matchedTargets.has(`${targetFunc.filePath}:${targetFunc.name}`)) continue;

      const similarity = calculateFunctionSimilarity(sourceFunc, targetFunc);
      
      if (similarity > bestSimilarity && similarity > 0.3) { // Threshold for considering a match
        bestMatch = targetFunc;
        bestSimilarity = similarity;
      }
    }

    if (bestMatch) {
      matchedTargets.add(`${bestMatch.filePath}:${bestMatch.name}`);
      
      const matchType = bestSimilarity === 1.0 ? 'identical' : 
                       sourceFunc.name !== bestMatch.name ? 'renamed' :
                       sourceFunc.filePath !== bestMatch.filePath ? 'moved' : 'similar';

      matches.push({
        type: matchType,
        sourceFunction: sourceFunc,
        targetFunction: bestMatch,
        similarity: bestSimilarity,
        changes: {
          signatureChanged: sourceFunc.signature !== bestMatch.signature,
          bodyChanged: sourceFunc.hash !== bestMatch.hash,
          moved: sourceFunc.filePath !== bestMatch.filePath,
          renamed: sourceFunc.name !== bestMatch.name
        }
      });
    } else {
      matches.push({
        type: 'deleted',
        sourceFunction: sourceFunc,
        similarity: 0
      });
    }
  }

  // Find added functions
  for (const targetFunc of targetFunctions) {
    if (!matchedTargets.has(`${targetFunc.filePath}:${targetFunc.name}`)) {
      matches.push({
        type: 'added',
        targetFunction: targetFunc,
        similarity: 0
      });
    }
  }

  return matches;
}

function calculateFunctionSimilarity(func1: any, func2: any): number {
  // Exact hash match
  if (func1.hash === func2.hash) return 1.0;
  
  // Name similarity
  const nameSimilarity = func1.name === func2.name ? 1.0 : 0.0;
  
  // Signature similarity
  const signatureSimilarity = func1.signature === func2.signature ? 1.0 : 0.5;
  
  // Content similarity (simplified)
  const contentSimilarity = calculateContentSimilarity(func1.content, func2.content);
  
  // Weighted average
  return (nameSimilarity * 0.3 + signatureSimilarity * 0.3 + contentSimilarity * 0.4);
}

function calculateContentSimilarity(content1: string, content2: string): number {
  // Simple line-based similarity
  const lines1 = content1.split('\n').map(l => l.trim()).filter(l => l.length > 0);
  const lines2 = content2.split('\n').map(l => l.trim()).filter(l => l.length > 0);
  
  const set1 = new Set(lines1);
  const set2 = new Set(lines2);
  
  const intersection = new Set([...set1].filter(line => set2.has(line)));
  const union = new Set([...set1, ...set2]);
  
  return union.size > 0 ? intersection.size / union.size : 0.0;
}

function generateSummary(fileChanges: FileChange[], functionMatches: FunctionMatch[]) {
  const summary = {
    totalFiles: fileChanges.length,
    addedFiles: fileChanges.filter(c => c.type === 'added').length,
    deletedFiles: fileChanges.filter(c => c.type === 'deleted').length,
    modifiedFiles: fileChanges.filter(c => c.type === 'modified').length,
    unchangedFiles: fileChanges.filter(c => c.type === 'unchanged').length,
    totalFunctions: functionMatches.length,
    addedFunctions: functionMatches.filter(m => m.type === 'added').length,
    deletedFunctions: functionMatches.filter(m => m.type === 'deleted').length,
    modifiedFunctions: functionMatches.filter(m => m.type === 'similar').length,
    movedFunctions: functionMatches.filter(m => m.type === 'moved' || m.type === 'renamed').length
  };

  return summary;
}
