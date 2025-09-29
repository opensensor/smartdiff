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
}

export interface ComparisonResult {
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

export interface ComparisonOptions {
  includeHiddenFiles?: boolean;
  fileExtensions?: string[];
  maxFileSize?: number;
  excludePatterns?: string[];
  functionSimilarityThreshold?: number;
  enableDeepAnalysis?: boolean;
}

export class ComparisonService {
  private baseUrl = '/api/comparison';

  async analyzeDirectories(
    sourcePath: string,
    targetPath: string,
    options: ComparisonOptions = {}
  ): Promise<ComparisonResult> {
    const response = await fetch(`${this.baseUrl}/analyze`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        sourcePath,
        targetPath,
        options
      }),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Unknown error' }));
      throw new Error(error.error || `HTTP ${response.status}: ${response.statusText}`);
    }

    return response.json();
  }

  async getFileContent(filePath: string): Promise<string> {
    const response = await fetch(`${this.baseUrl}/file-content`, {
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
    const response = await fetch(`${this.baseUrl}/diff`, {
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
