'use client';

import React, { useState, useCallback, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { 
  ChevronRight, 
  ChevronDown, 
  Folder, 
  FolderOpen, 
  File, 
  Search,
  Filter,
  X,
  Check,
  Minus
} from 'lucide-react';
import { api } from '@/api/client';
import { FileSystemEntry, BrowseDirectoryRequest } from '@/types';
import { Input } from '@/components/ui/Input';
import { Button } from '@/components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';

interface FileSystemExplorerProps {
  onSelectionChange?: (selectedPaths: string[]) => void;
  multiSelect?: boolean;
  allowDirectories?: boolean;
  allowFiles?: boolean;
  fileExtensions?: string[];
  initialPath?: string;
}

interface TreeNodeState {
  expanded: boolean;
  selected: boolean;
  loading: boolean;
}

export function FileSystemExplorer({
  onSelectionChange,
  multiSelect = true,
  allowDirectories = true,
  allowFiles = true,
  fileExtensions,
  initialPath = '/'
}: FileSystemExplorerProps) {
  const [rootPath, setRootPath] = useState(initialPath);
  const [searchQuery, setSearchQuery] = useState('');
  const [showHidden, setShowHidden] = useState(false);
  const [nodeStates, setNodeStates] = useState<Record<string, TreeNodeState>>({});
  const [selectedPaths, setSelectedPaths] = useState<Set<string>>(new Set());

  // Fetch directory contents
  const { data: directoryData, isLoading, error } = useQuery({
    queryKey: ['filesystem', rootPath, showHidden, fileExtensions],
    queryFn: () => api.browseDirectory({
      path: rootPath,
      recursive: false,
      include_hidden: showHidden,
      max_depth: 1
    } as BrowseDirectoryRequest),
    enabled: !!rootPath
  });

  // Filter entries based on search and file type preferences
  const filteredEntries = useMemo(() => {
    if (!directoryData?.entries) return [];
    
    let entries = directoryData.entries.filter(entry => {
      // Filter by file type
      if (entry.is_directory && !allowDirectories) return false;
      if (!entry.is_directory && !allowFiles) return false;
      
      // Filter by file extensions
      if (!entry.is_directory && fileExtensions && fileExtensions.length > 0) {
        const ext = entry.name.split('.').pop()?.toLowerCase();
        if (!ext || !fileExtensions.includes(ext)) return false;
      }
      
      // Filter by search query
      if (searchQuery) {
        return entry.name.toLowerCase().includes(searchQuery.toLowerCase());
      }
      
      return true;
    });

    // Sort: directories first, then files, both alphabetically
    return entries.sort((a, b) => {
      if (a.is_directory && !b.is_directory) return -1;
      if (!a.is_directory && b.is_directory) return 1;
      return a.name.localeCompare(b.name);
    });
  }, [directoryData?.entries, searchQuery, allowDirectories, allowFiles, fileExtensions]);

  // Toggle node expansion
  const toggleExpanded = useCallback((path: string) => {
    setNodeStates(prev => ({
      ...prev,
      [path]: {
        ...prev[path],
        expanded: !prev[path]?.expanded
      }
    }));
  }, []);

  // Handle selection
  const handleSelection = useCallback((path: string, isDirectory: boolean) => {
    if (!multiSelect) {
      setSelectedPaths(new Set([path]));
      onSelectionChange?.([path]);
      return;
    }

    setSelectedPaths(prev => {
      const newSelection = new Set(prev);
      if (newSelection.has(path)) {
        newSelection.delete(path);
      } else {
        newSelection.add(path);
      }
      onSelectionChange?.(Array.from(newSelection));
      return newSelection;
    });
  }, [multiSelect, onSelectionChange]);

  // Navigate to directory
  const navigateToDirectory = useCallback((path: string) => {
    setRootPath(path);
    setSelectedPaths(new Set());
    setNodeStates({});
  }, []);

  // Get parent directory
  const getParentDirectory = useCallback(() => {
    const parts = rootPath.split('/').filter(Boolean);
    if (parts.length === 0) return '/';
    return '/' + parts.slice(0, -1).join('/');
  }, [rootPath]);

  // Clear selection
  const clearSelection = useCallback(() => {
    setSelectedPaths(new Set());
    onSelectionChange?.([]);
  }, [onSelectionChange]);

  // Get file icon
  const getFileIcon = useCallback((entry: FileSystemEntry) => {
    if (entry.is_directory) {
      const isExpanded = nodeStates[entry.path]?.expanded;
      return isExpanded ? <FolderOpen className="w-4 h-4" /> : <Folder className="w-4 h-4" />;
    }
    return <File className="w-4 h-4" />;
  }, [nodeStates]);

  // Get selection state for display
  const getSelectionState = useCallback(() => {
    const total = filteredEntries.length;
    const selected = selectedPaths.size;
    
    if (selected === 0) return 'none';
    if (selected === total) return 'all';
    return 'partial';
  }, [filteredEntries.length, selectedPaths.size]);

  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center justify-between">
          <span>File System Explorer</span>
          {selectedPaths.size > 0 && (
            <Button variant="outline" size="sm" onClick={clearSelection}>
              <X className="w-4 h-4 mr-1" />
              Clear ({selectedPaths.size})
            </Button>
          )}
        </CardTitle>
        
        {/* Navigation */}
        <div className="flex items-center gap-2 text-sm">
          <Button 
            variant="ghost" 
            size="sm" 
            onClick={() => navigateToDirectory(getParentDirectory())}
            disabled={rootPath === '/'}
          >
            ↑ Parent
          </Button>
          <span className="text-muted-foreground truncate flex-1">{rootPath}</span>
        </div>

        {/* Search and Filters */}
        <div className="space-y-2">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <Input
              placeholder="Search files and directories..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
          
          <div className="flex items-center gap-2">
            <Button
              variant={showHidden ? "default" : "outline"}
              size="sm"
              onClick={() => setShowHidden(!showHidden)}
            >
              <Filter className="w-4 h-4 mr-1" />
              Hidden Files
            </Button>
            
            {multiSelect && filteredEntries.length > 0 && (
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  const selectionState = getSelectionState();
                  if (selectionState === 'all') {
                    clearSelection();
                  } else {
                    const allPaths = filteredEntries.map(entry => entry.path);
                    setSelectedPaths(new Set(allPaths));
                    onSelectionChange?.(allPaths);
                  }
                }}
              >
                {getSelectionState() === 'all' ? (
                  <Minus className="w-4 h-4 mr-1" />
                ) : (
                  <Check className="w-4 h-4 mr-1" />
                )}
                Select All
              </Button>
            )}
          </div>
        </div>
      </CardHeader>

      <CardContent className="flex-1 overflow-auto p-0">
        {isLoading && (
          <div className="flex items-center justify-center h-32">
            <div className="loading-spinner w-6 h-6" />
          </div>
        )}

        {error && (
          <div className="p-4 text-center text-destructive">
            <p>Error loading directory: {error.message}</p>
            <Button 
              variant="outline" 
              size="sm" 
              onClick={() => window.location.reload()}
              className="mt-2"
            >
              Retry
            </Button>
          </div>
        )}

        {!isLoading && !error && filteredEntries.length === 0 && (
          <div className="p-4 text-center text-muted-foreground">
            <Folder className="w-12 h-12 mx-auto mb-2 opacity-50" />
            <p>No files or directories found</p>
            {searchQuery && (
              <p className="text-sm">Try adjusting your search query</p>
            )}
          </div>
        )}

        {!isLoading && !error && filteredEntries.length > 0 && (
          <div className="divide-y">
            {filteredEntries.map((entry) => (
              <FileSystemItem
                key={entry.path}
                entry={entry}
                isSelected={selectedPaths.has(entry.path)}
                onSelect={() => handleSelection(entry.path, entry.is_directory)}
                onNavigate={() => entry.is_directory && navigateToDirectory(entry.path)}
                icon={getFileIcon(entry)}
                multiSelect={multiSelect}
              />
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}

interface FileSystemItemProps {
  entry: FileSystemEntry;
  isSelected: boolean;
  onSelect: () => void;
  onNavigate: () => void;
  icon: React.ReactNode;
  multiSelect: boolean;
}

function FileSystemItem({ 
  entry, 
  isSelected, 
  onSelect, 
  onNavigate, 
  icon, 
  multiSelect 
}: FileSystemItemProps) {
  const formatSize = (bytes?: number) => {
    if (!bytes) return '';
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '';
    return new Date(dateString).toLocaleDateString();
  };

  return (
    <div 
      className={`flex items-center gap-3 p-3 hover:bg-muted/50 cursor-pointer transition-colors ${
        isSelected ? 'bg-primary/10 border-l-2 border-l-primary' : ''
      }`}
      onClick={onSelect}
      onDoubleClick={onNavigate}
    >
      {multiSelect && (
        <div className={`w-4 h-4 border rounded ${
          isSelected ? 'bg-primary border-primary' : 'border-muted-foreground'
        } flex items-center justify-center`}>
          {isSelected && <Check className="w-3 h-3 text-primary-foreground" />}
        </div>
      )}
      
      <div className="flex items-center gap-2 flex-1 min-w-0">
        {icon}
        <span className="truncate font-medium">{entry.name}</span>
        {entry.language && (
          <span className="text-xs bg-muted px-2 py-1 rounded">
            {entry.language}
          </span>
        )}
      </div>
      
      <div className="flex items-center gap-4 text-sm text-muted-foreground">
        {!entry.is_directory && entry.size && (
          <span>{formatSize(entry.size)}</span>
        )}
        {entry.modified && (
          <span>{formatDate(entry.modified)}</span>
        )}
        {entry.is_directory && (
          <ChevronRight className="w-4 h-4" />
        )}
      </div>
    </div>
  );
}
