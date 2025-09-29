'use client';

import { useState } from 'react';
import { clsx } from 'clsx';
import { 
  FolderOpen, 
  GitCompare, 
  Network, 
  BarChart3, 
  Settings, 
  ChevronLeft,
  ChevronRight,
  Home
} from 'lucide-react';

interface SidebarProps {
  collapsed: boolean;
  onToggle: () => void;
}

interface NavItem {
  id: string;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  href?: string;
  active?: boolean;
}

const navItems: NavItem[] = [
  { id: 'home', label: 'Home', icon: Home, href: '/', active: true },
  { id: 'files', label: 'File Explorer', icon: FolderOpen },
  { id: 'diff', label: 'Diff Viewer', icon: GitCompare },
  { id: 'graph', label: 'Dependency Graph', icon: Network },
  { id: 'analysis', label: 'Analysis', icon: BarChart3 },
  { id: 'settings', label: 'Settings', icon: Settings },
];

export function Sidebar({ collapsed, onToggle }: SidebarProps) {
  const [activeItem, setActiveItem] = useState('home');

  return (
    <div 
      className={clsx(
        'fixed left-0 top-0 h-full bg-card border-r border-border transition-all duration-300 z-50',
        collapsed ? 'w-16' : 'w-64'
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-border">
        {!collapsed && (
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
              <GitCompare className="w-4 h-4 text-primary-foreground" />
            </div>
            <span className="font-semibold text-foreground">Smart Diff</span>
          </div>
        )}
        
        <button
          onClick={onToggle}
          className="p-1 rounded-md hover:bg-muted transition-colors"
        >
          {collapsed ? (
            <ChevronRight className="w-4 h-4" />
          ) : (
            <ChevronLeft className="w-4 h-4" />
          )}
        </button>
      </div>

      {/* Navigation */}
      <nav className="p-2">
        <ul className="space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeItem === item.id;
            
            return (
              <li key={item.id}>
                <button
                  onClick={() => setActiveItem(item.id)}
                  className={clsx(
                    'w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors',
                    isActive 
                      ? 'bg-primary text-primary-foreground' 
                      : 'text-muted-foreground hover:text-foreground hover:bg-muted'
                  )}
                  title={collapsed ? item.label : undefined}
                >
                  <Icon className="w-4 h-4 flex-shrink-0" />
                  {!collapsed && (
                    <span className="truncate">{item.label}</span>
                  )}
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Footer */}
      {!collapsed && (
        <div className="absolute bottom-4 left-4 right-4">
          <div className="text-xs text-muted-foreground">
            <p>Smart Diff v0.1.0</p>
            <p>Graph-based code comparison</p>
          </div>
        </div>
      )}
    </div>
  );
}
