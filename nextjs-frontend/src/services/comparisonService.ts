export interface FileInfo {
  path: string;
  relativePath: string;
  size: number;
  modified: string;
  hash: string;
  language?: string;
  functions?: FunctionInfo[];
}

export interface FunctionInfo {
  name: string;
  signature: string;
  startLine: number;
  endLine: number;
  content: string;
  hash: string;
  complexity?: number;
  parameters?: string[];
  returnType?: string;
  filePath?: string;
}

export interface ComparisonResult {
  comparisonId?: string; // MCP comparison ID for getting function diffs
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

export interface FileChange {
  type: 'added' | 'deleted' | 'modified' | 'unchanged' | 'moved';
  sourcePath?: string;
  targetPath?: string;
  similarity?: number;
  sizeChange?: number;
  linesAdded?: number;
  linesDeleted?: number;
}

export interface FunctionMatch {
  type: 'identical' | 'similar' | 'renamed' | 'moved' | 'added' | 'deleted' | 'modified';
  matchType: 'identical' | 'similar' | 'renamed' | 'moved' | 'added' | 'deleted' | 'modified';
  sourceFunction?: FunctionInfo & { filePath: string };
  targetFunction?: FunctionInfo & { filePath: string };
  similarity: number;
  changes?: {
    signatureChanged: boolean;
    bodyChanged: boolean;
    moved: boolean;
    renamed: boolean;
  };
  changeMagnitude?: number; // 0.0 = no change, 1.0 = complete change
}

export interface ComparisonOptions {
  includeHiddenFiles?: boolean;
  fileExtensions?: string[];
  maxFileSize?: number;
  excludePatterns?: string[];
  functionSimilarityThreshold?: number;
  enableDeepAnalysis?: boolean;
}

export class ComparisonService {
  // Use Rust backend for comparison analysis (with advanced AST-based matching)
  private rustBackendUrl = process.env.NEXT_PUBLIC_RUST_API_URL || 'http://localhost:8080';
  // Use Next.js backend for file operations
  private nextjsBackendUrl = `${process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000'}/api`;

  async analyzeDirectories(
    sourcePath: string,
    targetPath: string,
    options: ComparisonOptions = {}
  ): Promise<ComparisonResult> {
    // Call Rust backend for comparison analysis (now uses advanced AST-based matching)
    const response = await fetch(`${this.rustBackendUrl}/api/comparison/analyze`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        source_path: sourcePath,  // Rust backend expects snake_case
        target_path: targetPath,
        options: {
          include_hidden: options.includeHiddenFiles || false,
          file_extensions: options.fileExtensions || [
            // Common programming languages
            "js", "jsx", "ts", "tsx", "vue", "svelte",
            "py", "pyx", "pyi",
            "java", "kt", "scala",
            "c", "cpp", "cc", "cxx", "h", "hpp", "hxx",
            "cs", "vb", "fs",
            "rs", "go", "rb", "php",
            "swift", "m", "mm",
            "dart", "lua", "r", "jl",
            // Web technologies
            "html", "htm", "xml", "css", "scss", "sass", "less",
            // Configuration and data
            "json", "yaml", "yml", "toml", "ini", "cfg",
            // Shell and scripts
            "sh", "bash", "zsh", "fish", "ps1", "bat", "cmd",
            // Documentation
            "md", "rst", "txt"
          ],
          max_depth: 10,
          similarity_threshold: options.functionSimilarityThreshold || 0.7,
        },
      }),
    });

    if (!response.ok) {
      console.error('Rust backend request failed:', response.status, response.statusText);
      const error = await response.json().catch(() => ({ error: 'Unknown error' }));
      console.error('Error details:', error);
      throw new Error(error.error || `HTTP ${response.status}: ${response.statusText}`);
    }

    const rustResponse = await response.json();
    console.log('Rust backend response (AST-based):', rustResponse);

    // Transform Rust response (snake_case) to frontend interface (camelCase)
    return {
      summary: {
        totalFiles: rustResponse.summary.total_files,
        addedFiles: rustResponse.summary.added_files,
        deletedFiles: rustResponse.summary.deleted_files,
        modifiedFiles: rustResponse.summary.modified_files,
        unchangedFiles: rustResponse.summary.unchanged_files,
        totalFunctions: rustResponse.summary.total_functions,
        addedFunctions: rustResponse.summary.added_functions,
        deletedFunctions: rustResponse.summary.deleted_functions,
        modifiedFunctions: rustResponse.summary.modified_functions,
        movedFunctions: rustResponse.summary.moved_functions,
      },
      fileChanges: rustResponse.file_changes.map((change: any) => ({
        type: change.change_type,
        sourcePath: change.source_path,
        targetPath: change.target_path,
        similarity: change.similarity,
      })),
      functionMatches: rustResponse.function_matches.map((match: any) => {
        const similarity = match.similarity?.overall || 0;
        const changeMagnitude = 1 - similarity; // Higher magnitude = more changed

        return {
          type: match.match_type,
          matchType: match.match_type,
          sourceFunction: match.source_function ? {
            name: match.source_function.name,
            signature: match.source_function.signature,
            startLine: match.source_function.start_line,
            endLine: match.source_function.end_line,
            content: match.source_function.content || '',
            hash: match.source_function.hash || '',
            complexity: match.source_function.complexity,
            parameters: match.source_function.parameters,
            returnType: match.source_function.return_type,
            filePath: match.source_function.file_path || '',
          } : undefined,
          targetFunction: match.target_function ? {
            name: match.target_function.name,
            signature: match.target_function.signature,
            startLine: match.target_function.start_line,
            endLine: match.target_function.end_line,
            content: match.target_function.content || '',
            hash: match.target_function.hash || '',
            complexity: match.target_function.complexity,
            parameters: match.target_function.parameters,
            returnType: match.target_function.return_type,
            filePath: match.target_function.file_path || '',
          } : undefined,
          similarity,
          changeMagnitude,
          changes: {
            signatureChanged: match.source_function?.signature !== match.target_function?.signature,
            bodyChanged: similarity < 1.0,
            moved: match.source_function?.file_path !== match.target_function?.file_path,
            renamed: match.source_function?.name !== match.target_function?.name,
          },
        };
      }),
      analysisTime: rustResponse.execution_time_ms,
    };
  }

  async getFileContent(filePath: string): Promise<string> {
    // Use Next.js backend for file operations
    const response = await fetch(`${this.nextjsBackendUrl}/file-content`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ filePath }),
    });

    if (!response.ok) {
      throw new Error(`Failed to fetch file content: ${response.statusText}`);
    }

    const result = await response.json();
    return result.content;
  }

  async generateDiff(
    sourceFilePath: string,
    targetFilePath: string,
    options: { context?: number; ignoreWhitespace?: boolean } = {}
  ): Promise<string> {
    // Use Next.js backend for diff generation
    const response = await fetch(`${this.nextjsBackendUrl}/diff`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        sourceFilePath,
        targetFilePath,
        options
      }),
    });

    if (!response.ok) {
      throw new Error(`Failed to generate diff: ${response.statusText}`);
    }

    const result = await response.json();
    return result.diff;
  }

  // Utility methods for processing comparison results
  static getFileChangesByType(fileChanges: FileChange[]): Record<string, FileChange[]> {
    return fileChanges.reduce((acc, change) => {
      if (!acc[change.type]) {
        acc[change.type] = [];
      }
      acc[change.type].push(change);
      return acc;
    }, {} as Record<string, FileChange[]>);
  }

  static getFunctionMatchesByType(functionMatches: FunctionMatch[]): Record<string, FunctionMatch[]> {
    return functionMatches.reduce((acc, match) => {
      if (!acc[match.type]) {
        acc[match.type] = [];
      }
      acc[match.type].push(match);
      return acc;
    }, {} as Record<string, FunctionMatch[]>);
  }

  static calculateOverallSimilarity(result: ComparisonResult): number {
    const { summary } = result;
    const totalFiles = summary.totalFiles;
    
    if (totalFiles === 0) return 1.0;
    
    const unchangedWeight = summary.unchangedFiles * 1.0;
    const modifiedWeight = summary.modifiedFiles * 0.5; // Assume 50% similarity for modified files
    const addedDeletedWeight = (summary.addedFiles + summary.deletedFiles) * 0.0;
    
    return (unchangedWeight + modifiedWeight + addedDeletedWeight) / totalFiles;
  }

  static getTopChangedFiles(fileChanges: FileChange[], limit: number = 10): FileChange[] {
    return fileChanges
      .filter(change => change.type === 'modified')
      .sort((a, b) => (a.similarity || 0) - (b.similarity || 0))
      .slice(0, limit);
  }

  static getTopChangedFunctions(functionMatches: FunctionMatch[], limit: number = 10): FunctionMatch[] {
    return functionMatches
      .filter(match => match.type === 'similar' || match.type === 'renamed' || match.type === 'moved')
      .sort((a, b) => a.similarity - b.similarity)
      .slice(0, limit);
  }

  static groupFunctionsByFile(functionMatches: FunctionMatch[]): Record<string, FunctionMatch[]> {
    const grouped: Record<string, FunctionMatch[]> = {};
    
    for (const match of functionMatches) {
      const filePath = match.sourceFunction?.filePath || match.targetFunction?.filePath || 'unknown';
      
      if (!grouped[filePath]) {
        grouped[filePath] = [];
      }
      grouped[filePath].push(match);
    }
    
    return grouped;
  }

  static getLanguageDistribution(fileChanges: FileChange[]): Record<string, number> {
    const distribution: Record<string, number> = {};
    
    for (const change of fileChanges) {
      const filePath = change.sourcePath || change.targetPath || '';
      const extension = filePath.split('.').pop()?.toLowerCase() || 'unknown';
      
      distribution[extension] = (distribution[extension] || 0) + 1;
    }
    
    return distribution;
  }

  static formatAnalysisTime(milliseconds: number): string {
    if (milliseconds < 1000) {
      return `${milliseconds}ms`;
    } else if (milliseconds < 60000) {
      return `${(milliseconds / 1000).toFixed(1)}s`;
    } else {
      const minutes = Math.floor(milliseconds / 60000);
      const seconds = Math.floor((milliseconds % 60000) / 1000);
      return `${minutes}m ${seconds}s`;
    }
  }

  static getComplexityDistribution(functionMatches: FunctionMatch[]): {
    low: number;
    medium: number;
    high: number;
  } {
    const distribution = { low: 0, medium: 0, high: 0 };
    
    for (const match of functionMatches) {
      const complexity = match.sourceFunction?.complexity || match.targetFunction?.complexity || 0;
      
      if (complexity <= 5) {
        distribution.low++;
      } else if (complexity <= 15) {
        distribution.medium++;
      } else {
        distribution.high++;
      }
    }
    
    return distribution;
  }

  static generateInsights(result: ComparisonResult): string[] {
    const insights: string[] = [];
    const { summary, fileChanges, functionMatches } = result;
    
    // File-level insights
    if (summary.addedFiles > summary.deletedFiles * 2) {
      insights.push(`Significant expansion: ${summary.addedFiles} new files added`);
    } else if (summary.deletedFiles > summary.addedFiles * 2) {
      insights.push(`Major cleanup: ${summary.deletedFiles} files removed`);
    }
    
    // Function-level insights
    const renamedFunctions = functionMatches.filter(m => m.type === 'renamed').length;
    if (renamedFunctions > 0) {
      insights.push(`${renamedFunctions} functions were renamed`);
    }
    
    const movedFunctions = functionMatches.filter(m => m.type === 'moved').length;
    if (movedFunctions > 0) {
      insights.push(`${movedFunctions} functions were moved between files`);
    }
    
    // Complexity insights
    const complexityDist = this.getComplexityDistribution(functionMatches);
    if (complexityDist.high > complexityDist.low) {
      insights.push('High complexity functions dominate the codebase');
    }
    
    // Overall similarity insight
    const similarity = this.calculateOverallSimilarity(result);
    if (similarity > 0.8) {
      insights.push('High similarity - mostly minor changes');
    } else if (similarity < 0.3) {
      insights.push('Major restructuring detected');
    }
    
    return insights;
  }
}

// Export a singleton instance
export const comparisonService = new ComparisonService();
