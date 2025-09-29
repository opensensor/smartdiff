import { NextRequest, NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { filePath } = body;

    if (!filePath) {
      return NextResponse.json(
        { error: 'File path is required' },
        { status: 400 }
      );
    }

    // Security check: ensure the path is absolute and doesn't contain traversal attempts
    const absolutePath = path.resolve(filePath);
    if (!absolutePath.startsWith(process.cwd()) && !path.isAbsolute(filePath)) {
      return NextResponse.json(
        { error: 'Access denied: Invalid file path' },
        { status: 403 }
      );
    }

    try {
      const stats = await fs.stat(absolutePath);
      
      if (!stats.isFile()) {
        return NextResponse.json(
          { error: 'Path is not a file' },
          { status: 400 }
        );
      }

      // Check file size (limit to 10MB for safety)
      if (stats.size > 10 * 1024 * 1024) {
        return NextResponse.json(
          { error: 'File too large (max 10MB)' },
          { status: 413 }
        );
      }

      const content = await fs.readFile(absolutePath, 'utf-8');
      
      return NextResponse.json({
        content,
        size: stats.size,
        modified: stats.mtime.toISOString(),
        encoding: 'utf-8'
      });

    } catch (error: any) {
      if (error.code === 'ENOENT') {
        return NextResponse.json(
          { error: 'File not found' },
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
    console.error('File content error:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
