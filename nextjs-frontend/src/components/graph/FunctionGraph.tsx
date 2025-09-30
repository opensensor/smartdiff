'use client';

import React, { useEffect, useRef, useState, useCallback } from 'react';
import * as d3 from 'd3';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { 
  ZoomIn, 
  ZoomOut, 
  RotateCcw, 
  Download, 
  Settings,
  Play,
  Pause
} from 'lucide-react';
import { GraphMatchResult, FunctionMatch } from '@/types';

interface FunctionNode {
  id: string;
  name: string;
  type: 'added' | 'removed' | 'modified' | 'unchanged';
  similarity?: number;
  confidence?: number;
  x?: number;
  y?: number;
  fx?: number | null;
  fy?: number | null;
  group?: string;
  size?: number;
}

interface FunctionLink {
  source: string | FunctionNode;
  target: string | FunctionNode;
  type: 'dependency' | 'similarity' | 'move' | 'rename';
  strength: number;
  similarity?: number;
}

interface FunctionGraphProps {
  data?: GraphMatchResult;
  width?: number;
  height?: number;
  onNodeClick?: (node: FunctionNode) => void;
  onNodeHover?: (node: FunctionNode | null) => void;
}

export function FunctionGraph({ 
  data, 
  width = 800, 
  height = 600,
  onNodeClick,
  onNodeHover 
}: FunctionGraphProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const simulationRef = useRef<d3.Simulation<FunctionNode, FunctionLink> | null>(null);
  const [isPlaying, setIsPlaying] = useState(true);
  const [selectedNode, setSelectedNode] = useState<FunctionNode | null>(null);
  const [hoveredNode, setHoveredNode] = useState<FunctionNode | null>(null);

  // Transform data into nodes and links
  const { nodes, links } = React.useMemo(() => {
    if (!data) return { nodes: [], links: [] };

    const nodeMap = new Map<string, FunctionNode>();
    const linkArray: FunctionLink[] = [];

    // Create nodes from matches
    data.matches.forEach(match => {
      if (!nodeMap.has(match.source_id)) {
        nodeMap.set(match.source_id, {
          id: match.source_id,
          name: match.source_id.split('::').pop() || match.source_id,
          type: 'modified',
          similarity: match.similarity.overall_similarity,
          confidence: match.confidence,
          group: 'source',
          size: 10 + (match.confidence * 20)
        });
      }
      
      if (!nodeMap.has(match.target_id)) {
        nodeMap.set(match.target_id, {
          id: match.target_id,
          name: match.target_id.split('::').pop() || match.target_id,
          type: 'modified',
          similarity: match.similarity.overall_similarity,
          confidence: match.confidence,
          group: 'target',
          size: 10 + (match.confidence * 20)
        });
      }

      // Create similarity link
      linkArray.push({
        source: match.source_id,
        target: match.target_id,
        type: 'similarity',
        strength: match.similarity.overall_similarity,
        similarity: match.similarity.overall_similarity
      });
    });

    // Add nodes for additions and deletions
    data.additions.forEach(funcId => {
      nodeMap.set(funcId, {
        id: funcId,
        name: funcId.split('::').pop() || funcId,
        type: 'added',
        group: 'added',
        size: 15
      });
    });

    data.deletions.forEach(funcId => {
      nodeMap.set(funcId, {
        id: funcId,
        name: funcId.split('::').pop() || funcId,
        type: 'removed',
        group: 'removed',
        size: 15
      });
    });

    // Add move and rename links
    data.moves.forEach(move => {
      linkArray.push({
        source: move.function_id,
        target: move.function_id + '_moved',
        type: 'move',
        strength: move.similarity
      });
    });

    data.renames.forEach(rename => {
      linkArray.push({
        source: rename.old_name,
        target: rename.new_name,
        type: 'rename',
        strength: rename.similarity
      });
    });

    return {
      nodes: Array.from(nodeMap.values()),
      links: linkArray
    };
  }, [data]);

  // Color scheme for different node types
  const getNodeColor = useCallback((node: FunctionNode) => {
    switch (node.type) {
      case 'added': return '#22c55e';
      case 'removed': return '#ef4444';
      case 'modified': return '#f59e0b';
      case 'unchanged': return '#6b7280';
      default: return '#6b7280';
    }
  }, []);

  // Get link color based on type
  const getLinkColor = useCallback((link: FunctionLink) => {
    switch (link.type) {
      case 'similarity': return '#3b82f6';
      case 'dependency': return '#8b5cf6';
      case 'move': return '#06b6d4';
      case 'rename': return '#f97316';
      default: return '#6b7280';
    }
  }, []);

  // Initialize D3 simulation
  useEffect(() => {
    if (!svgRef.current || nodes.length === 0) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    const container = svg.append('g');

    // Create zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        container.attr('transform', event.transform);
      });

    svg.call(zoom);

    // Performance optimizations for large graphs
    const nodeCount = nodes.length;
    const isLargeGraph = nodeCount > 50;

    // Only optimize link creation for large graphs, keep forces natural
    const linkStrength = isLargeGraph ? 0.3 : 0.5;
    const alphaDecay = isLargeGraph ? 0.03 : 0.0228;

    // Create simulation - keep natural spreading behavior
    const simulation = d3.forceSimulation<FunctionNode>(nodes)
      .force('link', d3.forceLink<FunctionNode, FunctionLink>(links)
        .id(d => d.id)
        .distance(d => 100 - (d.strength * 50))
        .strength(d => d.strength * linkStrength)
      )
      .force('charge', d3.forceManyBody()
        .strength(-300)
        .distanceMax(500))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide()
        .radius(d => (d.size || 10) + 5)
        .strength(0.7))
      .alphaDecay(alphaDecay)
      .velocityDecay(0.4);

    simulationRef.current = simulation;

    // Create links
    const link = container.append('g')
      .selectAll('line')
      .data(links)
      .enter().append('line')
      .attr('stroke', getLinkColor)
      .attr('stroke-opacity', 0.6)
      .attr('stroke-width', d => Math.sqrt(d.strength * 5) + 1);

    // Create nodes
    const node = container.append('g')
      .selectAll('circle')
      .data(nodes)
      .enter().append('circle')
      .attr('r', d => d.size || 10)
      .attr('fill', getNodeColor)
      .attr('stroke', '#fff')
      .attr('stroke-width', 2)
      .style('cursor', 'pointer')
      .call(d3.drag<SVGCircleElement, FunctionNode>()
        .on('start', (event, d) => {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          d.fx = d.x;
          d.fy = d.y;
        })
        .on('drag', (event, d) => {
          d.fx = event.x;
          d.fy = event.y;
        })
        .on('end', (event, d) => {
          if (!event.active) simulation.alphaTarget(0);
          d.fx = null;
          d.fy = null;
        }));

    // Add labels
    const label = container.append('g')
      .selectAll('text')
      .data(nodes)
      .enter().append('text')
      .text(d => d.name)
      .attr('font-size', '12px')
      .attr('font-family', 'sans-serif')
      .attr('text-anchor', 'middle')
      .attr('dy', -15)
      .style('pointer-events', 'none')
      .style('fill', 'currentColor');

    // Node interactions
    node
      .on('click', (event, d) => {
        setSelectedNode(d);
        onNodeClick?.(d);
      })
      .on('mouseenter', (event, d) => {
        setHoveredNode(d);
        onNodeHover?.(d);
        
        // Highlight connected nodes and links
        node.style('opacity', n => n === d || links.some(l => 
          (l.source === d && l.target === n) || (l.target === d && l.source === n)
        ) ? 1 : 0.3);
        
        link.style('opacity', l => l.source === d || l.target === d ? 1 : 0.1);
        label.style('opacity', n => n === d ? 1 : 0.5);
      })
      .on('mouseleave', () => {
        setHoveredNode(null);
        onNodeHover?.(null);
        
        // Reset opacity
        node.style('opacity', 1);
        link.style('opacity', 0.6);
        label.style('opacity', 1);
      });

    // Update positions on simulation tick with soft boundary constraints
    simulation.on('tick', () => {
      // Soft boundaries - gently push nodes back if they go too far
      const margin = 100;
      nodes.forEach(d => {
        // Apply gentle force to keep nodes in view, but don't hard-clamp
        if (d.x! < margin) d.vx! += (margin - d.x!) * 0.01;
        if (d.x! > width - margin) d.vx! -= (d.x! - (width - margin)) * 0.01;
        if (d.y! < margin) d.vy! += (margin - d.y!) * 0.01;
        if (d.y! > height - margin) d.vy! -= (d.y! - (height - margin)) * 0.01;
      });

      link
        .attr('x1', d => (d.source as FunctionNode).x!)
        .attr('y1', d => (d.source as FunctionNode).y!)
        .attr('x2', d => (d.target as FunctionNode).x!)
        .attr('y2', d => (d.target as FunctionNode).y!);

      node
        .attr('cx', d => d.x!)
        .attr('cy', d => d.y!);

      label
        .attr('x', d => d.x!)
        .attr('y', d => d.y!);
    });

    // Stop simulation after it settles to save CPU
    simulation.on('end', () => {
      console.log('Graph simulation settled');
    });

    return () => {
      simulation.stop();
    };
  }, [nodes, links, width, height, getNodeColor, getLinkColor, onNodeClick, onNodeHover]);

  // Control functions
  const handleZoomIn = () => {
    const svg = d3.select(svgRef.current);
    svg.transition().call(
      d3.zoom<SVGSVGElement, unknown>().scaleBy as any, 1.5
    );
  };

  const handleZoomOut = () => {
    const svg = d3.select(svgRef.current);
    svg.transition().call(
      d3.zoom<SVGSVGElement, unknown>().scaleBy as any, 1 / 1.5
    );
  };

  const handleReset = () => {
    const svg = d3.select(svgRef.current);
    svg.transition().call(
      d3.zoom<SVGSVGElement, unknown>().transform as any,
      d3.zoomIdentity
    );
    
    if (simulationRef.current) {
      simulationRef.current.alpha(1).restart();
    }
  };

  const handlePlayPause = () => {
    if (simulationRef.current) {
      if (isPlaying) {
        simulationRef.current.stop();
      } else {
        simulationRef.current.restart();
      }
      setIsPlaying(!isPlaying);
    }
  };

  const handleDownload = () => {
    if (!svgRef.current) return;
    
    const svgData = new XMLSerializer().serializeToString(svgRef.current);
    const svgBlob = new Blob([svgData], { type: 'image/svg+xml;charset=utf-8' });
    const svgUrl = URL.createObjectURL(svgBlob);
    
    const downloadLink = document.createElement('a');
    downloadLink.href = svgUrl;
    downloadLink.download = 'function-graph.svg';
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
    URL.revokeObjectURL(svgUrl);
  };

  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle>Function Relationship Graph</CardTitle>
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleZoomIn}>
              <ZoomIn className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handleZoomOut}>
              <ZoomOut className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handleReset}>
              <RotateCcw className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handlePlayPause}>
              {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
            </Button>
            <Button variant="outline" size="sm" onClick={handleDownload}>
              <Download className="w-4 h-4" />
            </Button>
          </div>
        </div>
        
        {/* Legend */}
        <div className="flex items-center gap-4 text-sm">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-green-500"></div>
            <span>Added</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-red-500"></div>
            <span>Removed</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
            <span>Modified</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-gray-500"></div>
            <span>Unchanged</span>
          </div>
        </div>
      </CardHeader>

      <CardContent className="flex-1 p-0 relative">
        {nodes.length === 0 ? (
          <div className="flex items-center justify-center h-full text-muted-foreground">
            <div className="text-center">
              <Settings className="w-12 h-12 mx-auto mb-4 opacity-50" />
              <p>No function data available</p>
              <p className="text-sm">Run a comparison to see the function graph</p>
            </div>
          </div>
        ) : (
          <svg
            ref={svgRef}
            width={width}
            height={height}
            className="w-full h-full"
            style={{ background: 'transparent' }}
          />
        )}
        
        {/* Node info tooltip */}
        {hoveredNode && (
          <div className="absolute top-4 left-4 bg-background border rounded-lg p-3 shadow-lg z-10">
            <div className="font-medium">{hoveredNode.name}</div>
            <div className="text-sm text-muted-foreground">
              Type: {hoveredNode.type}
              {hoveredNode.similarity && (
                <div>Similarity: {(hoveredNode.similarity * 100).toFixed(1)}%</div>
              )}
              {hoveredNode.confidence && (
                <div>Confidence: {(hoveredNode.confidence * 100).toFixed(1)}%</div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
