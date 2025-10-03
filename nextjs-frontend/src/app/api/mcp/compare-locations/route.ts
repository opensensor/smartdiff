import { NextRequest, NextResponse } from 'next/server';

const MCP_SERVER_URL = process.env.MCP_SERVER_URL || 'http://127.0.0.1:8011';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { sourcePath, targetPath } = body;

    if (!sourcePath || !targetPath) {
      return NextResponse.json(
        { error: 'Both sourcePath and targetPath are required' },
        { status: 400 }
      );
    }

    // Call MCP server's compare_locations tool
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
          name: 'compare_locations',
          arguments: {
            source_path: sourcePath,
            target_path: targetPath,
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

    // Parse the text content to extract comparison ID and summary
    const parsedResult = parseComparisonResult(textContent);

    return NextResponse.json({
      success: true,
      comparisonId: parsedResult.comparisonId,
      summary: parsedResult.summary,
      rawText: textContent,
    });
  } catch (error: any) {
    console.error('MCP compare-locations error:', error);
    return NextResponse.json(
      { error: error.message || 'Internal server error' },
      { status: 500 }
    );
  }
}

function parseComparisonResult(text: string): {
  comparisonId: string;
  summary: {
    totalFunctions: number;
    addedFunctions: number;
    deletedFunctions: number;
    modifiedFunctions: number;
    renamedFunctions: number;
    movedFunctions: number;
  };
} {
  // Extract comparison ID
  const idMatch = text.match(/Comparison ID:\s*([a-f0-9-]+)/i);
  const comparisonId = idMatch ? idMatch[1] : '';

  // Extract summary counts
  const totalMatch = text.match(/Total functions analyzed:\s*(\d+)/i);
  const addedMatch = text.match(/Added:\s*(\d+)/i);
  const deletedMatch = text.match(/Deleted:\s*(\d+)/i);
  const modifiedMatch = text.match(/Modified:\s*(\d+)/i);
  const renamedMatch = text.match(/Renamed:\s*(\d+)/i);
  const movedMatch = text.match(/Moved:\s*(\d+)/i);

  return {
    comparisonId,
    summary: {
      totalFunctions: totalMatch ? parseInt(totalMatch[1], 10) : 0,
      addedFunctions: addedMatch ? parseInt(addedMatch[1], 10) : 0,
      deletedFunctions: deletedMatch ? parseInt(deletedMatch[1], 10) : 0,
      modifiedFunctions: modifiedMatch ? parseInt(modifiedMatch[1], 10) : 0,
      renamedFunctions: renamedMatch ? parseInt(renamedMatch[1], 10) : 0,
      movedFunctions: movedMatch ? parseInt(movedMatch[1], 10) : 0,
    },
  };
}

