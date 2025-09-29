'use client';

import React, { memo, useMemo, useState, useCallback } from 'react';
import { useVirtualScroll, useDebounce } from '@/hooks/useVirtualScroll';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Button } from '@/components/ui/Button';
import { 
  Search, 
  File, 
  Folder, 
  ChevronRight, 
  ChevronDown,
  Filter,
  SortAsc,
  SortDesc
} from 'lucide-react';

interface FileItem {
  id: string;
  name: string;
  type: 'file' | 'directory';
  path: string;
  size?: number;
  modified?: Date;
  children?: FileItem[];
  isExpanded?: boolean;
}

interface OptimizedFileListProps {
  files: FileItem[];
  onFileSelect?: (file: FileItem) => void;
  onDirectoryToggle?: (directory: FileItem) => void;
  height?: number;
  searchable?: boolean;
  sortable?: boolean;
}

// Memoized file item component for performance
const FileItemComponent = memo(({ 
  item, 
  depth = 0, 
  onSelect, 
  onToggle 
}: {
  item: FileItem;
  depth?: number;
  onSelect?: (item: FileItem) => void;
  onToggle?: (item: FileItem) => void;
}) => {
  const handleClick = useCallback(() => {
    if (item.type === 'directory') {
      onToggle?.(item);
    } else {
      onSelect?.(item);
    }
  }, [item, onSelect, onToggle]);

  return (
    <div
      className="flex items-center gap-2 px-3 py-2 hover:bg-gray-50 cursor-pointer transition-colors"
      style={{ paddingLeft: `${depth * 20 + 12}px` }}
      onClick={handleClick}
    >
      {item.type === 'directory' && (
        <div className="w-4 h-4 flex items-center justify-center">
          {item.isExpanded ? (
            <ChevronDown className="w-3 h-3 text-gray-500" />
          ) : (
            <ChevronRight className="w-3 h-3 text-gray-500" />
          )}
        </div>
      )}
      
      <div className="w-4 h-4 flex items-center justify-center">
        {item.type === 'directory' ? (
          <Folder className="w-4 h-4 text-blue-500" />
        ) : (
          <File className="w-4 h-4 text-gray-500" />
        )}
      </div>
      
      <span className="flex-1 text-sm truncate">{item.name}</span>
      
      {item.size && (
        <span className="text-xs text-gray-500">
          {formatFileSize(item.size)}
        </span>
      )}
    </div>
  );
});

FileItemComponent.displayName = 'FileItemComponent';

// Utility function to format file sizes
function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB'];
  let size = bytes;
  let unitIndex = 0;
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  
  return `${size.toFixed(1)} ${units[unitIndex]}`;
}

// Flatten file tree for virtual scrolling
function flattenFileTree(files: FileItem[], depth = 0): Array<FileItem & { depth: number }> {
  const result: Array<FileItem & { depth: number }> = [];
  
  for (const file of files) {
    result.push({ ...file, depth });
    
    if (file.type === 'directory' && file.isExpanded && file.children) {
      result.push(...flattenFileTree(file.children, depth + 1));
    }
  }
  
  return result;
}

export function OptimizedFileList({
  files,
  onFileSelect,
  onDirectoryToggle,
  height = 400,
  searchable = true,
  sortable = true
}: OptimizedFileListProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'name' | 'size' | 'modified'>('name');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  
  const debouncedSearchTerm = useDebounce(searchTerm, 300);

  // Filter and sort files
  const processedFiles = useMemo(() => {
    let filtered = files;

    // Apply search filter
    if (debouncedSearchTerm) {
      const filterFiles = (items: FileItem[]): FileItem[] => {
        return items.filter(item => {
          const matchesSearch = item.name.toLowerCase().includes(debouncedSearchTerm.toLowerCase());
          const hasMatchingChildren = item.children ? filterFiles(item.children).length > 0 : false;
          
          if (matchesSearch || hasMatchingChildren) {
            return {
              ...item,
              children: item.children ? filterFiles(item.children) : undefined
            };
          }
          return false;
        }).map(item => ({
          ...item,
          children: item.children ? filterFiles(item.children) : undefined
        }));
      };
      
      filtered = filterFiles(files);
    }

    // Apply sorting
    const sortFiles = (items: FileItem[]): FileItem[] => {
      return [...items].sort((a, b) => {
        // Directories first
        if (a.type !== b.type) {
          return a.type === 'directory' ? -1 : 1;
        }

        let comparison = 0;
        switch (sortBy) {
          case 'name':
            comparison = a.name.localeCompare(b.name);
            break;
          case 'size':
            comparison = (a.size || 0) - (b.size || 0);
            break;
          case 'modified':
            comparison = (a.modified?.getTime() || 0) - (b.modified?.getTime() || 0);
            break;
        }

        return sortOrder === 'desc' ? -comparison : comparison;
      }).map(item => ({
        ...item,
        children: item.children ? sortFiles(item.children) : undefined
      }));
    };

    return sortFiles(filtered);
  }, [files, debouncedSearchTerm, sortBy, sortOrder]);

  // Flatten for virtual scrolling
  const flattenedFiles = useMemo(() => {
    return flattenFileTree(processedFiles);
  }, [processedFiles]);

  // Virtual scrolling
  const {
    startIndex,
    endIndex,
    totalHeight,
    offsetY,
    scrollElementProps
  } = useVirtualScroll(flattenedFiles, {
    itemHeight: 36,
    containerHeight: height,
    overscan: 5
  });

  const visibleItems = flattenedFiles.slice(startIndex, endIndex + 1);

  const handleSort = (newSortBy: typeof sortBy) => {
    if (sortBy === newSortBy) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(newSortBy);
      setSortOrder('asc');
    }
  };

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center gap-2">
          <Folder className="w-5 h-5" />
          File Explorer
        </CardTitle>
        
        {/* Search and Controls */}
        {(searchable || sortable) && (
          <div className="flex items-center gap-2">
            {searchable && (
              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <Input
                  placeholder="Search files..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            )}
            
            {sortable && (
              <div className="flex items-center gap-1">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleSort('name')}
                  className={sortBy === 'name' ? 'bg-blue-50' : ''}
                >
                  Name
                  {sortBy === 'name' && (
                    sortOrder === 'asc' ? <SortAsc className="w-3 h-3 ml-1" /> : <SortDesc className="w-3 h-3 ml-1" />
                  )}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleSort('size')}
                  className={sortBy === 'size' ? 'bg-blue-50' : ''}
                >
                  Size
                  {sortBy === 'size' && (
                    sortOrder === 'asc' ? <SortAsc className="w-3 h-3 ml-1" /> : <SortDesc className="w-3 h-3 ml-1" />
                  )}
                </Button>
              </div>
            )}
          </div>
        )}
      </CardHeader>

      <CardContent className="p-0">
        <div {...scrollElementProps}>
          <div style={{ height: totalHeight, position: 'relative' }}>
            <div style={{ transform: `translateY(${offsetY}px)` }}>
              {visibleItems.map((item, index) => (
                <FileItemComponent
                  key={`${item.id}-${startIndex + index}`}
                  item={item}
                  depth={item.depth}
                  onSelect={onFileSelect}
                  onToggle={onDirectoryToggle}
                />
              ))}
            </div>
          </div>
        </div>
        
        {flattenedFiles.length === 0 && (
          <div className="text-center py-8 text-gray-500">
            <Folder className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No files found</p>
            {debouncedSearchTerm && (
              <p className="text-sm">Try adjusting your search terms</p>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
