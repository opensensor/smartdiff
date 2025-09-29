'use client';

import React, { useState, useEffect } from 'react';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/Dialog';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { FolderOpen, Check, X } from 'lucide-react';
import { FileSystemExplorer } from './FileSystemExplorer';

interface DirectoryPickerProps {
  value?: string;
  onChange?: (path: string) => void;
  placeholder?: string;
  disabled?: boolean;
  allowFiles?: boolean;
  fileExtensions?: string[];
  multiSelect?: boolean;
  children?: React.ReactNode;
}

export function DirectoryPicker({
  value,
  onChange,
  placeholder = "Select directory...",
  disabled = false,
  allowFiles = false,
  fileExtensions,
  multiSelect = false,
  children
}: DirectoryPickerProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedPaths, setSelectedPaths] = useState<string[]>([]);
  const [manualPath, setManualPath] = useState(value || '');
  const [initialPath, setInitialPath] = useState<string>('/');

  // Get user's home directory on mount
  useEffect(() => {
    const getHomeDirectory = async () => {
      try {
        const response = await fetch('/api/filesystem/home');
        const data = await response.json();
        if (data.success) {
          setInitialPath(data.homeDirectory);
        }
      } catch (error) {
        console.error('Failed to get home directory:', error);
        // Fallback to root if home directory fetch fails
        setInitialPath('/');
      }
    };

    getHomeDirectory();
  }, []);

  const handleSelectionChange = (paths: string[]) => {
    setSelectedPaths(paths);
  };

  const handleConfirm = () => {
    if (multiSelect) {
      onChange?.(selectedPaths.join(';')); // Join multiple paths with semicolon
    } else if (selectedPaths.length > 0) {
      onChange?.(selectedPaths[0]);
    }
    setIsOpen(false);
  };

  const handleManualPathChange = (path: string) => {
    setManualPath(path);
    if (!multiSelect) {
      onChange?.(path);
    }
  };

  const displayValue = multiSelect && value?.includes(';') 
    ? `${value.split(';').length} paths selected`
    : value || '';

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <div className="flex gap-2">
        <Input
          value={displayValue}
          onChange={(e) => handleManualPathChange(e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
          className="flex-1"
        />
        <DialogTrigger asChild>
          {children || (
            <Button variant="outline" size="icon" disabled={disabled}>
              <FolderOpen className="w-4 h-4" />
            </Button>
          )}
        </DialogTrigger>
      </div>

      <DialogContent className="max-w-4xl h-[80vh] flex flex-col">
        <DialogHeader>
          <DialogTitle>
            Select {allowFiles ? 'Files or Directories' : 'Directory'}
            {multiSelect && ' (Multiple Selection)'}
          </DialogTitle>
        </DialogHeader>

        <div className="flex-1 overflow-hidden">
          <FileSystemExplorer
            onSelectionChange={handleSelectionChange}
            multiSelect={multiSelect}
            allowDirectories={true}
            allowFiles={allowFiles}
            fileExtensions={fileExtensions}
            initialPath={value?.split(';')[0] || initialPath}
          />
        </div>

        <div className="flex items-center justify-between pt-4 border-t">
          <div className="text-sm text-muted-foreground">
            {selectedPaths.length > 0 ? (
              multiSelect ? (
                `${selectedPaths.length} item${selectedPaths.length === 1 ? '' : 's'} selected`
              ) : (
                `Selected: ${selectedPaths[0]}`
              )
            ) : (
              'No items selected'
            )}
          </div>

          <div className="flex gap-2">
            <Button variant="outline" onClick={() => setIsOpen(false)}>
              <X className="w-4 h-4 mr-2" />
              Cancel
            </Button>
            <Button 
              onClick={handleConfirm}
              disabled={selectedPaths.length === 0}
            >
              <Check className="w-4 h-4 mr-2" />
              Select
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
