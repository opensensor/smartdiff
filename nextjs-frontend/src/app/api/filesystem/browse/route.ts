import { NextRequest, NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { path: requestPath, include_hidden = false, max_depth = 1 } = body;

    // Validate and sanitize the path
    let targetPath = requestPath || '/';
    
    // For Windows compatibility, handle drive letters
    if (process.platform === 'win32') {
      if (targetPath === '/') {
        // On Windows, list available drives
        const drives = [];
        for (let i = 65; i <= 90; i++) {
          const drive = String.fromCharCode(i) + ':';
          try {
            await fs.access(drive + '\\');
            drives.push({
              name: drive,
              path: drive + '\\',
              is_directory: true,
              size: 0,
              modified: new Date().toISOString()
            });
          } catch {
            // Drive not available
          }
        }
        return NextResponse.json({ entries: drives });
      }
      
      // Convert Unix-style paths to Windows paths
      if (targetPath.startsWith('/') && !targetPath.includes(':')) {
        targetPath = 'C:' + targetPath.replace(/\//g, '\\');
      }
    }

    // Resolve the absolute path
    const absolutePath = path.resolve(targetPath);
    
    // Security check: prevent directory traversal attacks
    if (!absolutePath.startsWith(process.cwd()) && !path.isAbsolute(targetPath)) {
      return NextResponse.json(
        { error: 'Access denied: Path outside allowed directory' },
        { status: 403 }
      );
    }

    // Check if path exists and is accessible
    try {
      const stats = await fs.stat(absolutePath);
      if (!stats.isDirectory()) {
        return NextResponse.json(
          { error: 'Path is not a directory' },
          { status: 400 }
        );
      }
    } catch (error) {
      return NextResponse.json(
        { error: 'Path does not exist or is not accessible' },
        { status: 404 }
      );
    }

    // Read directory contents
    const entries = [];
    try {
      const items = await fs.readdir(absolutePath, { withFileTypes: true });
      
      for (const item of items) {
        // Skip hidden files unless requested
        if (!include_hidden && item.name.startsWith('.')) {
          continue;
        }

        try {
          const itemPath = path.join(absolutePath, item.name);
          const stats = await fs.stat(itemPath);
          
          // Convert back to Unix-style paths for frontend
          let displayPath = itemPath;
          if (process.platform === 'win32') {
            displayPath = itemPath.replace(/\\/g, '/');
            if (displayPath.match(/^[A-Z]:/)) {
              displayPath = '/' + displayPath;
            }
          }

          entries.push({
            name: item.name,
            path: displayPath,
            is_directory: item.isDirectory(),
            size: item.isFile() ? stats.size : 0,
            modified: stats.mtime.toISOString()
          });
        } catch (error) {
          // Skip items that can't be accessed (permission issues, etc.)
          console.warn(`Skipping ${item.name}: ${error}`);
        }
      }

      // Sort entries: directories first, then files, both alphabetically
      entries.sort((a, b) => {
        if (a.is_directory && !b.is_directory) return -1;
        if (!a.is_directory && b.is_directory) return 1;
        return a.name.localeCompare(b.name);
      });

      return NextResponse.json({ entries });

    } catch (error) {
      console.error('Error reading directory:', error);
      return NextResponse.json(
        { error: 'Failed to read directory contents' },
        { status: 500 }
      );
    }

  } catch (error) {
    console.error('API error:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

// Handle GET requests as well for convenience
export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const path = searchParams.get('path') || '/';
  const include_hidden = searchParams.get('include_hidden') === 'true';
  
  // Convert GET to POST format
  const mockRequest = {
    json: async () => ({ path, include_hidden })
  } as NextRequest;
  
  return POST(mockRequest);
}
