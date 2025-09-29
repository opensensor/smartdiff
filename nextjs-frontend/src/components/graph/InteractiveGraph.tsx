'use client';

import React, { useCallback, useMemo, useState } from 'react';
import {
  ReactFlow,
  Node,
  Edge,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
  Controls,
  MiniMap,
  Background,
  BackgroundVariant,
  Panel,
  NodeTypes,
  EdgeTypes,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { GraphMatchResult } from '@/types';
import { 
  GitBranch, 
  GitMerge, 
  Plus, 
  Minus, 
  Edit,
  ArrowRight
} from 'lucide-react';

// Custom node component for functions
function FunctionNode({ data }: { data: any }) {
  const getNodeIcon = () => {
    switch (data.type) {
      case 'added': return <Plus className="w-4 h-4" />;
      case 'removed': return <Minus className="w-4 h-4" />;
      case 'modified': return <Edit className="w-4 h-4" />;
      case 'moved': return <ArrowRight className="w-4 h-4" />;
      default: return <GitBranch className="w-4 h-4" />;
    }
  };

  const getNodeColor = () => {
    switch (data.type) {
      case 'added': return 'bg-green-100 border-green-500 text-green-800';
      case 'removed': return 'bg-red-100 border-red-500 text-red-800';
      case 'modified': return 'bg-yellow-100 border-yellow-500 text-yellow-800';
      case 'moved': return 'bg-blue-100 border-blue-500 text-blue-800';
      default: return 'bg-gray-100 border-gray-500 text-gray-800';
    }
  };

  return (
    <div className={`px-4 py-2 shadow-md rounded-md border-2 ${getNodeColor()} min-w-[150px]`}>
      <div className="flex items-center gap-2">
        {getNodeIcon()}
        <div className="font-bold text-sm">{data.label}</div>
      </div>
      {data.similarity && (
        <div className="text-xs mt-1">
          Similarity: {(data.similarity * 100).toFixed(1)}%
        </div>
      )}
      {data.confidence && (
        <div className="text-xs">
          Confidence: {(data.confidence * 100).toFixed(1)}%
        </div>
      )}
      {data.file && (
        <div className="text-xs text-gray-600 truncate">
          {data.file}
        </div>
      )}
    </div>
  );
}

// Custom edge component for relationships
function RelationshipEdge({ data }: { data: any }) {
  const getEdgeColor = () => {
    switch (data.type) {
      case 'similarity': return '#3b82f6';
      case 'dependency': return '#8b5cf6';
      case 'move': return '#06b6d4';
      case 'rename': return '#f97316';
      default: return '#6b7280';
    }
  };

  return (
    <div className="react-flow__edge-path" style={{ stroke: getEdgeColor() }}>
      {data.label && (
        <div className="react-flow__edge-label bg-white px-2 py-1 rounded text-xs border">
          {data.label}
        </div>
      )}
    </div>
  );
}

const nodeTypes: NodeTypes = {
  function: FunctionNode,
};

const edgeTypes: EdgeTypes = {
  relationship: RelationshipEdge,
};

interface InteractiveGraphProps {
  data?: GraphMatchResult;
  onNodeClick?: (nodeId: string) => void;
  onEdgeClick?: (edgeId: string) => void;
}

export function InteractiveGraph({ data, onNodeClick, onEdgeClick }: InteractiveGraphProps) {
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [selectedEdge, setSelectedEdge] = useState<string | null>(null);

  // Transform data into React Flow format
  const { initialNodes, initialEdges } = useMemo(() => {
    if (!data) return { initialNodes: [], initialEdges: [] };

    const nodes: Node[] = [];
    const edges: Edge[] = [];
    const nodePositions = new Map<string, { x: number; y: number }>();

    // Calculate positions in a grid layout
    let x = 0;
    let y = 0;
    const spacing = 200;
    const maxColumns = 4;

    // Create nodes for function matches
    data.matches.forEach((match, index) => {
      const sourceId = `source-${match.source_id}`;
      const targetId = `target-${match.target_id}`;

      // Source node
      if (!nodePositions.has(sourceId)) {
        nodes.push({
          id: sourceId,
          type: 'function',
          position: { x: x * spacing, y: y * spacing },
          data: {
            label: match.source_id.split('::').pop() || match.source_id,
            type: 'modified',
            similarity: match.similarity.overall_similarity,
            confidence: match.confidence,
            file: match.source_id.split('::')[0],
            originalId: match.source_id,
          },
        });
        nodePositions.set(sourceId, { x: x * spacing, y: y * spacing });
        
        x++;
        if (x >= maxColumns) {
          x = 0;
          y++;
        }
      }

      // Target node
      if (!nodePositions.has(targetId)) {
        nodes.push({
          id: targetId,
          type: 'function',
          position: { x: x * spacing, y: y * spacing },
          data: {
            label: match.target_id.split('::').pop() || match.target_id,
            type: 'modified',
            similarity: match.similarity.overall_similarity,
            confidence: match.confidence,
            file: match.target_id.split('::')[0],
            originalId: match.target_id,
          },
        });
        nodePositions.set(targetId, { x: x * spacing, y: y * spacing });
        
        x++;
        if (x >= maxColumns) {
          x = 0;
          y++;
        }
      }

      // Create similarity edge
      edges.push({
        id: `similarity-${index}`,
        source: sourceId,
        target: targetId,
        type: 'relationship',
        animated: match.similarity.overall_similarity > 0.8,
        style: { 
          strokeWidth: Math.max(1, match.similarity.overall_similarity * 5),
          stroke: '#3b82f6'
        },
        data: {
          type: 'similarity',
          label: `${(match.similarity.overall_similarity * 100).toFixed(0)}%`,
          similarity: match.similarity.overall_similarity,
        },
      });
    });

    // Add nodes for additions
    data.additions.forEach((funcId, index) => {
      const nodeId = `added-${funcId}`;
      nodes.push({
        id: nodeId,
        type: 'function',
        position: { x: x * spacing, y: y * spacing },
        data: {
          label: funcId.split('::').pop() || funcId,
          type: 'added',
          file: funcId.split('::')[0],
          originalId: funcId,
        },
      });
      
      x++;
      if (x >= maxColumns) {
        x = 0;
        y++;
      }
    });

    // Add nodes for deletions
    data.deletions.forEach((funcId, index) => {
      const nodeId = `removed-${funcId}`;
      nodes.push({
        id: nodeId,
        type: 'function',
        position: { x: x * spacing, y: y * spacing },
        data: {
          label: funcId.split('::').pop() || funcId,
          type: 'removed',
          file: funcId.split('::')[0],
          originalId: funcId,
        },
      });
      
      x++;
      if (x >= maxColumns) {
        x = 0;
        y++;
      }
    });

    // Add edges for moves
    data.moves.forEach((move, index) => {
      const sourceId = `source-${move.function_id}`;
      const targetId = `moved-${move.function_id}`;
      
      edges.push({
        id: `move-${index}`,
        source: sourceId,
        target: targetId,
        type: 'relationship',
        animated: true,
        style: { stroke: '#06b6d4', strokeDasharray: '5,5' },
        data: {
          type: 'move',
          label: 'Moved',
          similarity: move.similarity,
        },
      });
    });

    // Add edges for renames
    data.renames.forEach((rename, index) => {
      const sourceId = `source-${rename.old_name}`;
      const targetId = `target-${rename.new_name}`;
      
      edges.push({
        id: `rename-${index}`,
        source: sourceId,
        target: targetId,
        type: 'relationship',
        animated: true,
        style: { stroke: '#f97316', strokeDasharray: '3,3' },
        data: {
          type: 'rename',
          label: 'Renamed',
          similarity: rename.similarity,
        },
      });
    });

    return { initialNodes: nodes, initialEdges: edges };
  }, [data]);

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const onNodeClickHandler = useCallback((event: React.MouseEvent, node: Node) => {
    setSelectedNode(node.id);
    onNodeClick?.(node.data.originalId);
  }, [onNodeClick]);

  const onEdgeClickHandler = useCallback((event: React.MouseEvent, edge: Edge) => {
    setSelectedEdge(edge.id);
    onEdgeClick?.(edge.id);
  }, [onEdgeClick]);

  // Get statistics
  const stats = useMemo(() => {
    if (!data) return null;
    
    return {
      total: nodes.length,
      added: data.additions.length,
      removed: data.deletions.length,
      modified: data.matches.length,
      moved: data.moves.length,
      renamed: data.renames.length,
      similarity: data.overall_similarity,
    };
  }, [data, nodes.length]);

  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <GitMerge className="w-5 h-5" />
            Interactive Function Graph
          </CardTitle>
          
          {stats && (
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="text-green-600">
                +{stats.added}
              </Badge>
              <Badge variant="outline" className="text-red-600">
                -{stats.removed}
              </Badge>
              <Badge variant="outline" className="text-yellow-600">
                ~{stats.modified}
              </Badge>
              <Badge variant="outline" className="text-blue-600">
                {(stats.similarity * 100).toFixed(1)}%
              </Badge>
            </div>
          )}
        </div>
      </CardHeader>

      <CardContent className="flex-1 p-0 relative">
        {nodes.length === 0 ? (
          <div className="flex items-center justify-center h-full text-muted-foreground">
            <div className="text-center">
              <GitBranch className="w-12 h-12 mx-auto mb-4 opacity-50" />
              <p>No function data available</p>
              <p className="text-sm">Run a comparison to see the interactive graph</p>
            </div>
          </div>
        ) : (
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            onNodeClick={onNodeClickHandler}
            onEdgeClick={onEdgeClickHandler}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            fitView
            attributionPosition="bottom-left"
          >
            <Controls />
            <MiniMap 
              nodeColor={(node) => {
                switch (node.data?.type) {
                  case 'added': return '#22c55e';
                  case 'removed': return '#ef4444';
                  case 'modified': return '#f59e0b';
                  default: return '#6b7280';
                }
              }}
            />
            <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
            
            <Panel position="top-right">
              <div className="bg-background border rounded-lg p-3 shadow-lg">
                <div className="text-sm font-medium mb-2">Legend</div>
                <div className="space-y-1 text-xs">
                  <div className="flex items-center gap-2">
                    <Plus className="w-3 h-3 text-green-600" />
                    <span>Added</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Minus className="w-3 h-3 text-red-600" />
                    <span>Removed</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Edit className="w-3 h-3 text-yellow-600" />
                    <span>Modified</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <ArrowRight className="w-3 h-3 text-blue-600" />
                    <span>Moved</span>
                  </div>
                </div>
              </div>
            </Panel>
          </ReactFlow>
        )}
      </CardContent>
    </Card>
  );
}
