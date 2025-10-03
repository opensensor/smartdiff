import { NextRequest, NextResponse } from 'next/server';

const MCP_SERVER_URL = process.env.MCP_SERVER_URL || 'http://127.0.0.1:8011';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { comparisonId, functionName, includeContent = true } = body;

    if (!comparisonId || !functionName) {
      return NextResponse.json(
        { error: 'Both comparisonId and functionName are required' },
        { status: 400 }
      );
    }

    // Call MCP server's get_function_diff tool
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
          name: 'get_function_diff',
          arguments: {
            comparison_id: comparisonId,
            function_name: functionName,
            include_content: includeContent,
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

    // Parse the text content to extract structured data
    const parsedDiff = parseFunctionDiff(textContent);

    return NextResponse.json({
      success: true,
      diff: parsedDiff,
      rawText: textContent,
    });

  } catch (error) {
    console.error('Function diff error:', error);
    return NextResponse.json(
      { 
        error: 'Failed to get function diff',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      { status: 500 }
    );
  }
}

function parseFunctionDiff(text: string) {
  const lines = text.split('\n');
  const result: any = {
    functionName: '',
    changeType: '',
    changeMagnitude: 0,
    similarityScore: 0,
    sourceFile: '',
    sourceLines: '',
    sourceSignature: '',
    targetFile: '',
    targetLines: '',
    targetSignature: '',
    summary: '',
    sourceContent: '',
    targetContent: '',
    unifiedDiff: '',
  };

  let section = '';
  let contentBuffer: string[] = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Parse header information
    if (line.startsWith('Function:')) {
      result.functionName = line.substring('Function:'.length).trim();
    } else if (line.startsWith('Change Type:')) {
      result.changeType = line.substring('Change Type:'.length).trim();
    } else if (line.startsWith('Change Magnitude:')) {
      result.changeMagnitude = parseFloat(line.substring('Change Magnitude:'.length).trim());
    } else if (line.startsWith('Similarity Score:')) {
      result.similarityScore = parseFloat(line.substring('Similarity Score:'.length).trim());
    } else if (line.startsWith('Source File:')) {
      result.sourceFile = line.substring('Source File:'.length).trim();
    } else if (line.startsWith('Source Lines:')) {
      result.sourceLines = line.substring('Source Lines:'.length).trim();
    } else if (line.startsWith('Source Signature:')) {
      result.sourceSignature = line.substring('Source Signature:'.length).trim();
    } else if (line.startsWith('Target File:')) {
      result.targetFile = line.substring('Target File:'.length).trim();
    } else if (line.startsWith('Target Lines:')) {
      result.targetLines = line.substring('Target Lines:'.length).trim();
    } else if (line.startsWith('Target Signature:')) {
      result.targetSignature = line.substring('Target Signature:'.length).trim();
    } else if (line.startsWith('Summary:')) {
      result.summary = line.substring('Summary:'.length).trim();
    }

    // Detect sections
    else if (line === '--- Source Content ---') {
      section = 'source';
      contentBuffer = [];
    } else if (line === '--- Target Content ---') {
      if (section === 'source') {
        result.sourceContent = contentBuffer.join('\n');
      }
      section = 'target';
      contentBuffer = [];
    } else if (line === '=== Unified Diff ===') {
      if (section === 'target') {
        result.targetContent = contentBuffer.join('\n');
      }
      section = 'diff';
      contentBuffer = [];
    } else if (section) {
      contentBuffer.push(line);
    }
  }

  // Capture remaining content
  if (section === 'diff') {
    result.unifiedDiff = contentBuffer.join('\n');
  } else if (section === 'target') {
    result.targetContent = contentBuffer.join('\n');
  }

  return result;
}

