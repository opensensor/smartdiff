import React, { useState } from 'react';
import { 
  ChevronDown, 
  ChevronRight, 
  Function, 
  Package, 
  FileText,
  Plus,
  Minus,
  Edit,
  ArrowRight,
  Search,
  Filter
} from 'lucide-react';
import { clsx } from 'clsx';

interface StructureNode {
  id: string;
  name: string;
  type: 'file' | 'class' | 'function' | 'variable' | 'import';
  changeType?: 'added' | 'removed' | 'modified' | 'moved' | 'renamed';
  similarity?: number;
  children?: StructureNode[];
  metadata?: {
    lineNumber?: number;
    parameters?: string[];
    returnType?: string;
    complexity?: number;
    size?: number;
  };
  matchedWith?: string; // ID of matched node in other file
}

interface StructureData {
  sourceFile: {
    name: string;
    structure: StructureNode;
  };
  targetFile: {
    name: string;
    structure: StructureNode;
  };
  matches: Array<{
    sourceId: string;
    targetId: string;
    similarity: number;
    changeType: string;
  }>;
}

interface StructureViewProps {
  structureData: StructureData;
  className?: string;
}

export const StructureView: React.FC<StructureViewProps> = ({
  structureData,
  className,
}) => {
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState('');

  const toggleNode = (nodeId: string) => {
    const newExpanded = new Set(expandedNodes);
    if (newExpanded.has(nodeId)) {
      newExpanded.delete(nodeId);
    } else {
      newExpanded.add(nodeId);
    }
    setExpandedNodes(newExpanded);
  };

  const selectNode = (nodeId: string) => {
    setSelectedNode(nodeId);
    
    // Find matched node and highlight it
    const match = structureData.matches.find(
      m => m.sourceId === nodeId || m.targetId === nodeId
    );
    
    if (match) {
      const matchedId = match.sourceId === nodeId ? match.targetId : match.sourceId;
      // Scroll to matched node or highlight it
    }
  };

  const getNodeIcon = (node: StructureNode) => {
    switch (node.type) {
      case 'file':
        return <FileText className="h-4 w-4" />;
      case 'class':
        return <Package className="h-4 w-4" />;
      case 'function':
        return <Function className="h-4 w-4" />;
      default:
        return <div className="w-4 h-4" />;
    }
  };

  const getChangeIcon = (changeType?: string) => {
    switch (changeType) {
      case 'added':
        return <Plus className="h-3 w-3 text-success-600" />;
      case 'removed':
        return <Minus className="h-3 w-3 text-danger-600" />;
      case 'modified':
        return <Edit className="h-3 w-3 text-warning-600" />;
      case 'moved':
        return <ArrowRight className="h-3 w-3 text-blue-600" />;
      case 'renamed':
        return <Edit className="h-3 w-3 text-purple-600" />;
      default:
        return null;
    }
  };

  const getChangeColor = (changeType?: string) => {
    switch (changeType) {
      case 'added':
        return 'bg-success-50 border-success-200';
      case 'removed':
        return 'bg-danger-50 border-danger-200';
      case 'modified':
        return 'bg-warning-50 border-warning-200';
      case 'moved':
        return 'bg-blue-50 border-blue-200';
      case 'renamed':
        return 'bg-purple-50 border-purple-200';
      default:
        return 'bg-white border-gray-200';
    }
  };

  const getSimilarityColor = (similarity?: number) => {
    if (!similarity) return '';
    if (similarity >= 0.8) return 'text-success-600';
    if (similarity >= 0.6) return 'text-warning-600';
    return 'text-danger-600';
  };

  const filterNodes = (node: StructureNode): boolean => {
    if (filterType !== 'all' && node.changeType !== filterType) {
      return false;
    }
    
    if (searchQuery && !node.name.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    
    return true;
  };

  const renderNode = (node: StructureNode, depth: number = 0, side: 'source' | 'target') => {
    const hasChildren = node.children && node.children.length > 0;
    const isExpanded = expandedNodes.has(node.id);
    const isSelected = selectedNode === node.id;
    const shouldShow = filterNodes(node);
    
    if (!shouldShow && (!hasChildren || !node.children?.some(child => filterNodes(child)))) {
      return null;
    }

    const match = structureData.matches.find(
      m => (side === 'source' ? m.sourceId : m.targetId) === node.id
    );

    return (
      <div key={node.id} className="select-none">
        <div
          className={clsx(
            'flex items-center py-1 px-2 rounded cursor-pointer transition-colors',
            isSelected && 'bg-primary-100 border border-primary-300',
            !isSelected && 'hover:bg-gray-50',
            getChangeColor(node.changeType)
          )}
          style={{ paddingLeft: `${depth * 20 + 8}px` }}
          onClick={() => selectNode(node.id)}
        >
          {/* Expand/Collapse Button */}
          <button
            onClick={(e) => {
              e.stopPropagation();
              if (hasChildren) toggleNode(node.id);
            }}
            className="mr-1 p-0.5 hover:bg-gray-200 rounded"
          >
            {hasChildren ? (
              isExpanded ? (
                <ChevronDown className="h-3 w-3" />
              ) : (
                <ChevronRight className="h-3 w-3" />
              )
            ) : (
              <div className="w-3 h-3" />
            )}
          </button>

          {/* Node Icon */}
          <div className="mr-2 text-gray-600">
            {getNodeIcon(node)}
          </div>

          {/* Node Name */}
          <span className={clsx(
            'flex-1 text-sm',
            node.changeType && 'font-medium'
          )}>
            {node.name}
          </span>

          {/* Metadata */}
          <div className="flex items-center space-x-2 text-xs text-gray-500">
            {node.metadata?.complexity && (
              <span>C:{node.metadata.complexity}</span>
            )}
            {node.metadata?.lineNumber && (
              <span>L:{node.metadata.lineNumber}</span>
            )}
            {match && (
              <span className={getSimilarityColor(match.similarity)}>
                {(match.similarity * 100).toFixed(0)}%
              </span>
            )}
          </div>

          {/* Change Icon */}
          <div className="ml-2">
            {getChangeIcon(node.changeType)}
          </div>
        </div>

        {/* Children */}
        {hasChildren && isExpanded && (
          <div>
            {node.children!.map(child => 
              renderNode(child, depth + 1, side)
            )}
          </div>
        )}
      </div>
    );
  };

  const changeTypes = [
    { value: 'all', label: 'All Changes' },
    { value: 'added', label: 'Added' },
    { value: 'removed', label: 'Removed' },
    { value: 'modified', label: 'Modified' },
    { value: 'moved', label: 'Moved' },
    { value: 'renamed', label: 'Renamed' },
  ];

  return (
    <div className={clsx('bg-white rounded-lg border border-gray-200', className)}>
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200">
        <div className="flex items-center space-x-4">
          <h3 className="text-lg font-medium text-gray-900">
            Structure Comparison
          </h3>
          <div className="text-sm text-gray-600">
            {structureData.matches.length} matches found
          </div>
        </div>

        <div className="flex items-center space-x-2">
          {/* Search */}
          <div className="relative">
            <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search nodes..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-8 pr-3 py-1 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            />
          </div>

          {/* Filter */}
          <div className="relative">
            <Filter className="absolute left-2 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <select
              value={filterType}
              onChange={(e) => setFilterType(e.target.value)}
              className="pl-8 pr-8 py-1 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent appearance-none bg-white"
            >
              {changeTypes.map(type => (
                <option key={type.value} value={type.value}>
                  {type.label}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Structure Content */}
      <div className="grid grid-cols-2 divide-x divide-gray-200">
        {/* Source Structure */}
        <div className="p-4">
          <div className="flex items-center justify-between mb-3">
            <h4 className="text-sm font-medium text-gray-700">
              Source: {structureData.sourceFile.name}
            </h4>
            <button
              onClick={() => setExpandedNodes(new Set())}
              className="text-xs text-gray-500 hover:text-gray-700"
            >
              Collapse All
            </button>
          </div>
          
          <div className="space-y-1 max-h-96 overflow-y-auto">
            {renderNode(structureData.sourceFile.structure, 0, 'source')}
          </div>
        </div>

        {/* Target Structure */}
        <div className="p-4">
          <div className="flex items-center justify-between mb-3">
            <h4 className="text-sm font-medium text-gray-700">
              Target: {structureData.targetFile.name}
            </h4>
            <button
              onClick={() => {
                // Expand all nodes that have matches
                const newExpanded = new Set<string>();
                structureData.matches.forEach(match => {
                  newExpanded.add(match.targetId);
                });
                setExpandedNodes(newExpanded);
              }}
              className="text-xs text-gray-500 hover:text-gray-700"
            >
              Expand Matches
            </button>
          </div>
          
          <div className="space-y-1 max-h-96 overflow-y-auto">
            {renderNode(structureData.targetFile.structure, 0, 'target')}
          </div>
        </div>
      </div>

      {/* Legend */}
      <div className="p-4 border-t border-gray-200 bg-gray-50">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4 text-xs">
            <div className="flex items-center">
              <Plus className="h-3 w-3 text-success-600 mr-1" />
              <span>Added</span>
            </div>
            <div className="flex items-center">
              <Minus className="h-3 w-3 text-danger-600 mr-1" />
              <span>Removed</span>
            </div>
            <div className="flex items-center">
              <Edit className="h-3 w-3 text-warning-600 mr-1" />
              <span>Modified</span>
            </div>
            <div className="flex items-center">
              <ArrowRight className="h-3 w-3 text-blue-600 mr-1" />
              <span>Moved</span>
            </div>
          </div>
          
          <div className="text-xs text-gray-500">
            Click nodes to see matches • C: Complexity • L: Line Number
          </div>
        </div>
      </div>
    </div>
  );
};
