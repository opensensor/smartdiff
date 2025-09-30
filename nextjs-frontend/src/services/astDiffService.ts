export interface ASTDiffLine {
  type: 'unchanged' | 'added' | 'deleted' | 'modified';
  sourceLineNumber?: number;
  targetLineNumber?: number;
  sourceContent?: string;
  targetContent?: string;
  astNodeType?: string;
  similarity?: number;
  isStructuralChange?: boolean;
  semanticChanges?: string[];
}

export interface ASTDiffResult {
  lines: ASTDiffLine[];
  summary: {
    totalLines: number;
    addedLines: number;
    deletedLines: number;
    modifiedLines: number;
    unchangedLines: number;
    structuralChanges: number;
    semanticChanges: number;
  };
  astOperations: ASTOperation[];
}

export interface ASTOperation {
  type: 'insert' | 'delete' | 'update' | 'move';
  nodeType: string;
  position: number;
  description: string;
  impact: 'low' | 'medium' | 'high';
}

export interface FunctionDiffRequest {
  sourceFunction: {
    name: string;
    content: string;
    filePath: string;
  };
  targetFunction: {
    name: string;
    content: string;
    filePath: string;
  };
  language?: string;
}

export class ASTDiffService {
  private baseUrl = `${process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000'}/api`;

  async generateFunctionDiff(request: FunctionDiffRequest): Promise<ASTDiffResult> {
    console.log('AST Diff Request:', request);

    const requestBody = {
      source_content: request.sourceFunction.content,
      target_content: request.targetFunction.content,
      source_file_path: request.sourceFunction.filePath,
      target_file_path: request.targetFunction.filePath,
      language: request.language || 'auto',
      options: {
        enable_semantic_analysis: true,
        enable_structural_analysis: true,
        generate_line_mapping: true,
      }
    };

    console.log('AST Diff Request Body:', requestBody);

    const response = await fetch(`${this.baseUrl}/ast/diff`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(requestBody),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error('AST diff failed:', response.status, errorText);
      throw new Error(`AST diff failed: ${response.statusText}`);
    }

    const rustResponse = await response.json();
    console.log('AST Diff Response:', rustResponse);

    // Transform Rust response to frontend format
    const result = this.transformASTDiffResponse(rustResponse);
    console.log('Transformed AST Diff Result:', result);

    return result;
  }

  private transformASTDiffResponse(rustResponse: any): ASTDiffResult {
    const lines: ASTDiffLine[] = [];
    
    // Process line mappings from Rust backend
    if (rustResponse.line_mappings) {
      rustResponse.line_mappings.forEach((mapping: any) => {
        lines.push({
          type: mapping.change_type,
          sourceLineNumber: mapping.source_line,
          targetLineNumber: mapping.target_line,
          sourceContent: mapping.source_content,
          targetContent: mapping.target_content,
          astNodeType: mapping.ast_node_type,
          similarity: mapping.similarity,
          isStructuralChange: mapping.is_structural_change,
          semanticChanges: mapping.semantic_changes || [],
        });
      });
    }

    const summary = {
      totalLines: lines.length,
      addedLines: lines.filter(l => l.type === 'added').length,
      deletedLines: lines.filter(l => l.type === 'deleted').length,
      modifiedLines: lines.filter(l => l.type === 'modified').length,
      unchangedLines: lines.filter(l => l.type === 'unchanged').length,
      structuralChanges: lines.filter(l => l.isStructuralChange).length,
      semanticChanges: lines.reduce((sum, l) => sum + (l.semanticChanges?.length || 0), 0),
    };

    const astOperations: ASTOperation[] = (rustResponse.ast_operations || []).map((op: any) => ({
      type: op.operation_type,
      nodeType: op.node_type,
      position: op.position,
      description: op.description,
      impact: op.impact_level,
    }));

    return {
      lines,
      summary,
      astOperations,
    };
  }

  // Fallback: Generate simple line-based diff if AST diff is not available
  generateSimpleDiff(sourceContent: string, targetContent: string): ASTDiffResult {
    const sourceLines = sourceContent.split('\n');
    const targetLines = targetContent.split('\n');
    const lines: ASTDiffLine[] = [];

    // Simple LCS-based diff algorithm
    const maxLines = Math.max(sourceLines.length, targetLines.length);
    
    for (let i = 0; i < maxLines; i++) {
      const sourceLine = i < sourceLines.length ? sourceLines[i] : undefined;
      const targetLine = i < targetLines.length ? targetLines[i] : undefined;

      if (sourceLine === undefined) {
        // Added line
        lines.push({
          type: 'added',
          targetLineNumber: i + 1,
          targetContent: targetLine,
        });
      } else if (targetLine === undefined) {
        // Deleted line
        lines.push({
          type: 'deleted',
          sourceLineNumber: i + 1,
          sourceContent: sourceLine,
        });
      } else if (sourceLine === targetLine) {
        // Unchanged line
        lines.push({
          type: 'unchanged',
          sourceLineNumber: i + 1,
          targetLineNumber: i + 1,
          sourceContent: sourceLine,
          targetContent: targetLine,
        });
      } else {
        // Modified line
        lines.push({
          type: 'modified',
          sourceLineNumber: i + 1,
          targetLineNumber: i + 1,
          sourceContent: sourceLine,
          targetContent: targetLine,
          similarity: this.calculateLineSimilarity(sourceLine, targetLine),
        });
      }
    }

    const summary = {
      totalLines: lines.length,
      addedLines: lines.filter(l => l.type === 'added').length,
      deletedLines: lines.filter(l => l.type === 'deleted').length,
      modifiedLines: lines.filter(l => l.type === 'modified').length,
      unchangedLines: lines.filter(l => l.type === 'unchanged').length,
      structuralChanges: 0,
      semanticChanges: 0,
    };

    return {
      lines,
      summary,
      astOperations: [],
    };
  }

  private calculateLineSimilarity(line1: string, line2: string): number {
    const trimmed1 = line1.trim();
    const trimmed2 = line2.trim();
    
    if (trimmed1 === trimmed2) return 1.0;
    if (trimmed1.length === 0 || trimmed2.length === 0) return 0.0;
    
    // Simple character-based similarity
    const maxLength = Math.max(trimmed1.length, trimmed2.length);
    let matches = 0;
    
    for (let i = 0; i < Math.min(trimmed1.length, trimmed2.length); i++) {
      if (trimmed1[i] === trimmed2[i]) {
        matches++;
      }
    }
    
    return matches / maxLength;
  }
}

export const astDiffService = new ASTDiffService();
