import { NextRequest, NextResponse } from 'next/server';

const MCP_SERVER_URL = process.env.MCP_SERVER_URL || 'http://127.0.0.1:8011';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { comparisonId, limit = 1000, minMagnitude, changeTypes } = body;

    if (!comparisonId) {
      return NextResponse.json(
        { error: 'comparisonId is required' },
        { status: 400 }
      );
    }

    // Call MCP server's list_changed_functions tool
    const mcpResponse = await fetch(`${MCP_SERVER_URL}/message`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: Date.now(),
        method: 'tools/call',
        params: {
          name: 'list_changed_functions',
          arguments: {
            comparison_id: comparisonId,
            limit,
            min_magnitude: minMagnitude,
            change_types: changeTypes,
          },
        },
      }),
    });

    if (!mcpResponse.ok) {
      throw new Error(`MCP server returned ${mcpResponse.status}`);
    }

    const mcpResult = await mcpResponse.json();

    if (mcpResult.error) {
      return NextResponse.json(
        { error: mcpResult.error.message || 'MCP server error' },
        { status: 500 }
      );
    }

    // Extract the text content from MCP response
    const textContent = mcpResult.result?.content?.[0]?.text || '';

    // Parse the text content to extract function changes
    const parsedFunctions = parseFunctionChanges(textContent);

    return NextResponse.json({
      success: true,
      functions: parsedFunctions,
      rawText: textContent,
    });
  } catch (error: any) {
    console.error('MCP list-changed-functions error:', error);
    return NextResponse.json(
      { error: error.message || 'Internal server error' },
      { status: 500 }
    );
  }
}

interface FunctionChange {
  functionName: string;
  changeType: string;
  changeMagnitude: number;
  similarityScore: number;
  sourceFile?: string;
  targetFile?: string;
  sourceStartLine?: number;
  sourceEndLine?: number;
  targetStartLine?: number;
  targetEndLine?: number;
  diffSummary?: string;
}

function parseFunctionChanges(text: string): FunctionChange[] {
  const functions: FunctionChange[] = [];
  
  // Split by function entries (numbered list)
  const functionBlocks = text.split(/\n\d+\.\s+/);
  
  for (const block of functionBlocks) {
    if (!block.trim()) continue;
    
    // Parse function header: "function_name - change_type (magnitude: X.XX, similarity: X.XX)"
    const headerMatch = block.match(/^(.+?)\s+-\s+(\w+)\s+\(magnitude:\s+([\d.]+),\s+similarity:\s+([\d.]+)\)/);
    if (!headerMatch) continue;
    
    const functionName = headerMatch[1].trim();
    const changeType = headerMatch[2].toLowerCase();
    const changeMagnitude = parseFloat(headerMatch[3]);
    const similarityScore = parseFloat(headerMatch[4]);
    
    // Parse source file info
    const sourceMatch = block.match(/Source:\s+(.+?)\s+\(lines\s+(\d+)-(\d+)\)/);
    const sourceFile = sourceMatch ? sourceMatch[1].trim() : undefined;
    const sourceStartLine = sourceMatch ? parseInt(sourceMatch[2], 10) : undefined;
    const sourceEndLine = sourceMatch ? parseInt(sourceMatch[3], 10) : undefined;
    
    // Parse target file info
    const targetMatch = block.match(/Target:\s+(.+?)\s+\(lines\s+(\d+)-(\d+)\)/);
    const targetFile = targetMatch ? targetMatch[1].trim() : undefined;
    const targetStartLine = targetMatch ? parseInt(targetMatch[2], 10) : undefined;
    const targetEndLine = targetMatch ? parseInt(targetMatch[3], 10) : undefined;
    
    // Parse summary
    const summaryMatch = block.match(/Summary:\s+(.+?)(?:\n|$)/);
    const diffSummary = summaryMatch ? summaryMatch[1].trim() : undefined;
    
    functions.push({
      functionName,
      changeType,
      changeMagnitude,
      similarityScore,
      sourceFile,
      targetFile,
      sourceStartLine,
      sourceEndLine,
      targetStartLine,
      targetEndLine,
      diffSummary,
    });
  }
  
  return functions;
}

