import { NextRequest, NextResponse } from 'next/server';
import os from 'os';
import path from 'path';

export async function GET(request: NextRequest) {
  try {
    // Get the user's home directory
    const homeDir = os.homedir();
    
    // Normalize the path for cross-platform compatibility
    const normalizedPath = path.resolve(homeDir);
    
    return NextResponse.json({
      success: true,
      homeDirectory: normalizedPath
    });
  } catch (error) {
    console.error('Error getting home directory:', error);
    
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to get home directory',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      { status: 500 }
    );
  }
}
