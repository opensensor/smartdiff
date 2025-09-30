'use client';

import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/Dialog';
import { ComparisonResult, FunctionMatch } from '@/services/comparisonService';
import { diffService, FileDiff } from '@/services/diffService';
import { 
  ZoomIn, 
  ZoomOut, 
  RotateCcw, 
  Maximize2, 
  Filter,
  Search,
  GitBranch,
  Code,
  TrendingUp
} from 'lucide-react';

interface GraphNode {
  id: string;
  name: string;
  type: 'function' | 'class' | 'method';
  changeType: 'added' | 'deleted' | 'modified' | 'moved' | 'unchanged';
  similarity: number;
  filePath: string;
  functionMatch?: FunctionMatch;
  x?: number;
  y?: number;
  fx?: number | null;
  fy?: number | null;
}

interface GraphLink {
  source: string | GraphNode;
  target: string | GraphNode;
  type: 'calls' | 'inherits' | 'implements' | 'contains';
  strength: number;
}

interface FunctionGraphViewerProps {
  data?: ComparisonResult;
  onNodeSelect?: (node: GraphNode) => void;
}

export function FunctionGraphViewer({ data, onNodeSelect }: FunctionGraphViewerProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [showModal, setShowModal] = useState(false);
  const [functionDiff, setFunctionDiff] = useState<FileDiff | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [filterType, setFilterType] = useState<string>('all');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [graphStats, setGraphStats] = useState({
    totalNodes: 0,
    addedNodes: 0,
    deletedNodes: 0,
    modifiedNodes: 0,
    unchangedNodes: 0
  });

  // Convert comparison data to graph nodes and links
  const { nodes, links } = React.useMemo(() => {
    if (!data?.functionMatches) return { nodes: [], links: [] };

    const graphNodes: GraphNode[] = [];
    const graphLinks: GraphLink[] = [];
    const nodeMap = new Map<string, GraphNode>();

    // Create nodes from function matches - use a single node per unique function
    const functionNodeMap = new Map<string, GraphNode>();

    data.functionMatches.forEach((match, index) => {
      // Determine the primary function (prefer target for added, source for deleted, target for others)
      let primaryFunction = match.targetFunction || match.sourceFunction;
      let changeType = match.type;

      if (match.type === 'deleted' && match.sourceFunction) {
        primaryFunction = match.sourceFunction;
      }

      if (primaryFunction) {
        const functionKey = `${primaryFunction.name}-${primaryFunction.filePath}`;

        if (!functionNodeMap.has(functionKey)) {
          const node: GraphNode = {
            id: functionKey,
            name: primaryFunction.name,
            type: 'function',
            changeType: changeType === 'deleted' ? 'deleted' :
                       changeType === 'added' ? 'added' :
                       changeType === 'moved' ? 'moved' :
                       match.changes?.bodyChanged ? 'modified' : 'unchanged',
            similarity: match.similarity,
            filePath: primaryFunction.filePath,
            functionMatch: match
          };

          functionNodeMap.set(functionKey, node);
          graphNodes.push(node);
          nodeMap.set(functionKey, node);
        }
      }
    });

    // Create relationships based on function calls (simplified - could be enhanced with AST parsing)
    data.functionMatches.forEach((match) => {
      if (match.sourceFunction && match.targetFunction) {
        const sourceKey = `${match.sourceFunction.name}-${match.sourceFunction.filePath}`;
        const targetKey = `${match.targetFunction.name}-${match.targetFunction.filePath}`;

        if (sourceKey !== targetKey && nodeMap.has(sourceKey) && nodeMap.has(targetKey)) {
          graphLinks.push({
            source: sourceKey,
            target: targetKey,
            type: 'calls',
            strength: match.similarity
          });
        }
      }
    });

    // Add some synthetic relationships for better visualization
    const fileGroups = new Map<string, GraphNode[]>();
    graphNodes.forEach(node => {
      const fileName = node.filePath.split('/').pop() || 'unknown';
      if (!fileGroups.has(fileName)) {
        fileGroups.set(fileName, []);
      }
      fileGroups.get(fileName)!.push(node);
    });

    // Create weak links between functions in the same file (limit to prevent O(n²) explosion)
    fileGroups.forEach(nodes => {
      // Only create links for small groups or limit connections
      if (nodes.length <= 10) {
        // For small groups, connect all nodes
        for (let i = 0; i < nodes.length - 1; i++) {
          for (let j = i + 1; j < nodes.length; j++) {
            graphLinks.push({
              source: nodes[i].id,
              target: nodes[j].id,
              type: 'contains',
              strength: 0.1
            });
          }
        }
      } else {
        // For large groups, only connect adjacent nodes in a chain
        for (let i = 0; i < nodes.length - 1; i++) {
          graphLinks.push({
            source: nodes[i].id,
            target: nodes[i + 1].id,
            type: 'contains',
            strength: 0.1
          });
        }
      }
    });

    // Calculate stats
    const stats = {
      totalNodes: graphNodes.length,
      addedNodes: graphNodes.filter(n => n.changeType === 'added').length,
      deletedNodes: graphNodes.filter(n => n.changeType === 'deleted').length,
      modifiedNodes: graphNodes.filter(n => n.changeType === 'modified').length,
      unchangedNodes: graphNodes.filter(n => n.changeType === 'unchanged').length
    };
    setGraphStats(stats);

    return { nodes: graphNodes, links: graphLinks };
  }, [data]);

  // Filter nodes and links based on search and filter type
  const { filteredNodes, filteredLinks } = React.useMemo(() => {
    let filtered = nodes;

    if (searchTerm) {
      filtered = filtered.filter(node =>
        node.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        node.filePath.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    if (filterType !== 'all') {
      filtered = filtered.filter(node => node.changeType === filterType);
    }

    // Filter links to only include those between filtered nodes
    const filteredNodeIds = new Set(filtered.map(n => n.id));
    const filteredLinks = links.filter(link => {
      const sourceId = typeof link.source === 'string' ? link.source : link.source.id;
      const targetId = typeof link.target === 'string' ? link.target : link.target.id;
      return filteredNodeIds.has(sourceId) && filteredNodeIds.has(targetId);
    });

    return { filteredNodes: filtered, filteredLinks };
  }, [nodes, links, searchTerm, filterType]);

  // D3 force simulation
  useEffect(() => {
    if (!svgRef.current || filteredNodes.length === 0) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    // Larger canvas for better node spreading
    const width = 1200;
    const height = 900;

    svg.attr("width", width).attr("height", height);

    // Create zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        container.attr("transform", event.transform);
      });

    svg.call(zoom);

    const container = svg.append("g");

    // Performance optimizations for large graphs
    const nodeCount = filteredNodes.length;
    const isLargeGraph = nodeCount > 50;

    // Only optimize link creation for large graphs, keep forces natural
    const linkStrength = isLargeGraph ? 0.3 : 1;
    const alphaDecay = isLargeGraph ? 0.03 : 0.0228;

    // Create force simulation - keep natural spreading behavior
    const simulation = d3.forceSimulation<GraphNode>(filteredNodes)
      .force("link", d3.forceLink<GraphNode, GraphLink>(filteredLinks)
        .id(d => d.id)
        .distance(100)
        .strength(d => d.strength * linkStrength))
      .force("charge", d3.forceManyBody()
        .strength(-300)
        .distanceMax(500))
      .force("center", d3.forceCenter(width / 2, height / 2))
      .force("collision", d3.forceCollide().radius(30).strength(0.7))
      .alphaDecay(alphaDecay)
      .velocityDecay(0.4);

    // Create links with different styles for different relationship types
    const link = container.append("g")
      .selectAll("line")
      .data(filteredLinks)
      .enter().append("line")
      .attr("stroke", d => getLinkColor(d.type))
      .attr("stroke-opacity", d => d.type === 'contains' ? 0.2 : 0.6)
      .attr("stroke-width", d => getLinkWidth(d))
      .attr("stroke-dasharray", d => d.type === 'contains' ? "2,2" : "none");

    // Create nodes with dynamic sizing
    const node = container.append("g")
      .selectAll("circle")
      .data(filteredNodes)
      .enter().append("circle")
      .attr("r", d => getNodeSize(d))
      .attr("fill", d => getNodeColor(d.changeType))
      .attr("stroke", d => d.changeType === 'modified' ? '#f59e0b' : '#fff')
      .attr("stroke-width", d => d.changeType === 'modified' ? 3 : 2)
      .style("cursor", "pointer")
      .style("opacity", 0.9)
      .call(d3.drag<SVGCircleElement, GraphNode>()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended))
      .on("click", handleNodeClick)
      .on("mouseover", function(event, d) {
        d3.select(this)
          .transition()
          .duration(200)
          .attr("r", getNodeSize(d) * 1.2)
          .style("opacity", 1);

        // Show tooltip
        showTooltip(event, d);
      })
      .on("mouseout", function(event, d) {
        d3.select(this)
          .transition()
          .duration(200)
          .attr("r", getNodeSize(d))
          .style("opacity", 0.9);

        hideTooltip();
      });

    // Add labels
    const labels = container.append("g")
      .selectAll("text")
      .data(filteredNodes)
      .enter().append("text")
      .text(d => d.name)
      .attr("font-size", "12px")
      .attr("text-anchor", "middle")
      .attr("dy", "0.35em")
      .style("pointer-events", "none")
      .style("fill", "#333");

    // Update positions on simulation tick with soft boundary constraints
    simulation.on("tick", () => {
      // Soft boundaries - gently push nodes back if they go too far
      const margin = 100;
      filteredNodes.forEach(d => {
        // Apply gentle force to keep nodes in view, but don't hard-clamp
        if (d.x! < margin) d.vx! += (margin - d.x!) * 0.01;
        if (d.x! > width - margin) d.vx! -= (d.x! - (width - margin)) * 0.01;
        if (d.y! < margin) d.vy! += (margin - d.y!) * 0.01;
        if (d.y! > height - margin) d.vy! -= (d.y! - (height - margin)) * 0.01;
      });

      link
        .attr("x1", d => (d.source as GraphNode).x!)
        .attr("y1", d => (d.source as GraphNode).y!)
        .attr("x2", d => (d.target as GraphNode).x!)
        .attr("y2", d => (d.target as GraphNode).y!);

      node
        .attr("cx", d => d.x!)
        .attr("cy", d => d.y!);

      labels
        .attr("x", d => d.x!)
        .attr("y", d => d.y!);
    });

    // Stop simulation after it settles to save CPU
    simulation.on("end", () => {
      console.log("Simulation settled");
    });

    function dragstarted(event: any, d: GraphNode) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragged(event: any, d: GraphNode) {
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragended(event: any, d: GraphNode) {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }

    function handleNodeClick(event: any, d: GraphNode) {
      setSelectedNode(d);
      loadFunctionDiff(d);
      setShowModal(true);
      onNodeSelect?.(d);
    }

    return () => {
      simulation.stop();
    };
  }, [filteredNodes, filteredLinks]);

  const getNodeColor = (changeType: string) => {
    switch (changeType) {
      case 'added': return '#22c55e';
      case 'deleted': return '#ef4444';
      case 'modified': return '#f59e0b';
      case 'moved': return '#8b5cf6';
      case 'unchanged': return '#6b7280';
      default: return '#6b7280';
    }
  };

  const getNodeSize = (node: GraphNode) => {
    // Base size
    let size = 15;

    // Increase size based on change importance
    if (node.changeType === 'added' || node.changeType === 'deleted') {
      size += 5;
    } else if (node.changeType === 'modified') {
      size += 3;
    }

    // Increase size based on similarity (lower similarity = more important change)
    if (node.similarity < 0.5) {
      size += 5;
    } else if (node.similarity < 0.8) {
      size += 2;
    }

    return Math.min(size, 25); // Cap at 25px
  };

  const showTooltip = (event: any, node: GraphNode) => {
    // Create or update tooltip
    let tooltip = d3.select("body").select(".graph-tooltip");
    if (tooltip.empty()) {
      tooltip = d3.select("body")
        .append("div")
        .attr("class", "graph-tooltip")
        .style("position", "absolute")
        .style("background", "rgba(0, 0, 0, 0.8)")
        .style("color", "white")
        .style("padding", "8px")
        .style("border-radius", "4px")
        .style("font-size", "12px")
        .style("pointer-events", "none")
        .style("z-index", "1000");
    }

    tooltip
      .style("opacity", 1)
      .style("left", (event.pageX + 10) + "px")
      .style("top", (event.pageY - 10) + "px")
      .html(`
        <strong>${node.name}</strong><br/>
        Type: ${node.changeType}<br/>
        Similarity: ${(node.similarity * 100).toFixed(1)}%<br/>
        File: ${node.filePath.split('/').pop()}
      `);
  };

  const hideTooltip = () => {
    d3.select(".graph-tooltip").style("opacity", 0);
  };

  const getLinkColor = (linkType: string) => {
    switch (linkType) {
      case 'calls': return '#3b82f6';
      case 'inherits': return '#8b5cf6';
      case 'implements': return '#10b981';
      case 'contains': return '#6b7280';
      default: return '#6b7280';
    }
  };

  const getLinkWidth = (link: GraphLink) => {
    if (link.type === 'contains') return 1;
    return Math.max(1, Math.sqrt(link.strength * 8));
  };

  const loadFunctionDiff = async (node: GraphNode) => {
    if (!node.functionMatch) return;

    try {
      const match = node.functionMatch;
      if (match.sourceFunction && match.targetFunction) {
        const diff = await diffService.getFileDiff(
          match.sourceFunction.filePath,
          match.targetFunction.filePath,
          { contextLines: 5 }
        );
        setFunctionDiff(diff);
      }
    } catch (error) {
      console.error('Failed to load function diff:', error);
    }
  };

  return (
    <div className={`${isFullscreen ? 'fixed inset-0 z-50 bg-white' : ''}`}>
      <Card className="h-full">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <GitBranch className="w-5 h-5" />
              Function Graph Analysis
            </CardTitle>
            
            <div className="flex items-center gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setIsFullscreen(!isFullscreen)}
              >
                <Maximize2 className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {/* Controls */}
          <div className="flex items-center gap-4 flex-wrap">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search functions..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="flex items-center gap-2">
              <Filter className="w-4 h-4 text-gray-500" />
              <select
                value={filterType}
                onChange={(e) => setFilterType(e.target.value)}
                className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Changes ({nodes.length})</option>
                <option value="added">Added ({nodes.filter(n => n.changeType === 'added').length})</option>
                <option value="deleted">Deleted ({nodes.filter(n => n.changeType === 'deleted').length})</option>
                <option value="modified">Modified ({nodes.filter(n => n.changeType === 'modified').length})</option>
                <option value="moved">Moved ({nodes.filter(n => n.changeType === 'moved').length})</option>
                <option value="unchanged">Unchanged ({nodes.filter(n => n.changeType === 'unchanged').length})</option>
              </select>
            </div>

            {/* Stats */}
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="bg-green-50 text-green-700">
                +{graphStats.addedNodes}
              </Badge>
              <Badge variant="outline" className="bg-red-50 text-red-700">
                -{graphStats.deletedNodes}
              </Badge>
              <Badge variant="outline" className="bg-yellow-50 text-yellow-700">
                ~{graphStats.modifiedNodes}
              </Badge>
              <Badge variant="outline" className="bg-gray-50 text-gray-700">
                ={graphStats.unchangedNodes}
              </Badge>
            </div>

            <div className="text-sm text-gray-600">
              Showing {filteredNodes.length} of {nodes.length} functions
            </div>

            {/* Quick Filter Buttons */}
            <div className="flex items-center gap-1">
              <Button
                variant={filterType === 'added' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'added' ? 'all' : 'added')}
                className="text-xs"
              >
                +Added
              </Button>
              <Button
                variant={filterType === 'deleted' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'deleted' ? 'all' : 'deleted')}
                className="text-xs"
              >
                -Deleted
              </Button>
              <Button
                variant={filterType === 'modified' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setFilterType(filterType === 'modified' ? 'all' : 'modified')}
                className="text-xs"
              >
                ~Modified
              </Button>
            </div>
          </div>
        </CardHeader>

        <CardContent className="p-0">
          <div className="relative">
            <svg ref={svgRef} className="w-full h-[600px] border"></svg>
            
            {filteredNodes.length === 0 && (
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="text-center text-gray-500">
                  <Code className="w-12 h-12 mx-auto mb-4 text-gray-300" />
                  <p>No functions found</p>
                  <p className="text-sm">Try adjusting your search or filter criteria</p>
                </div>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Function Detail Modal */}
      <Dialog open={showModal} onOpenChange={setShowModal}>
        <DialogContent className="max-w-6xl max-h-[90vh] overflow-auto">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <Code className="w-5 h-5" />
              Function Details: {selectedNode?.name}
            </DialogTitle>
          </DialogHeader>
          
          {selectedNode && (
            <div className="space-y-4">
              {/* Function metadata */}
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <h4 className="font-medium mb-2">Function Information</h4>
                  <div className="space-y-1 text-sm">
                    <p><strong>Name:</strong> {selectedNode.name}</p>
                    <p><strong>Type:</strong> {selectedNode.type}</p>
                    <p><strong>File:</strong> {selectedNode.filePath}</p>
                    <div><strong>Change Type:</strong>
                      <Badge className={`ml-2 ${getNodeColor(selectedNode.changeType)}`}>
                        {selectedNode.changeType}
                      </Badge>
                    </div>
                    <p><strong>Similarity:</strong> {(selectedNode.similarity * 100).toFixed(1)}%</p>
                  </div>
                </div>
                
                {functionDiff && (
                  <div>
                    <h4 className="font-medium mb-2">Diff Statistics</h4>
                    <div className="space-y-1 text-sm">
                      <p><strong>Lines Added:</strong> <span className="text-green-600">+{functionDiff.stats.additions}</span></p>
                      <p><strong>Lines Deleted:</strong> <span className="text-red-600">-{functionDiff.stats.deletions}</span></p>
                      <p><strong>Lines Modified:</strong> <span className="text-blue-600">~{functionDiff.stats.modifications}</span></p>
                    </div>
                  </div>
                )}
              </div>

              {/* Function diff */}
              {functionDiff && (
                <div>
                  <h4 className="font-medium mb-2">Function Diff</h4>
                  <div className="bg-gray-50 rounded-lg p-4 max-h-96 overflow-auto">
                    <div className="font-mono text-sm space-y-1">
                      {functionDiff.lines.map((line, index) => (
                        <div
                          key={index}
                          className={`flex items-start gap-3 px-2 py-1 ${
                            line.type === 'added' ? 'bg-green-50 text-green-800' :
                            line.type === 'removed' ? 'bg-red-50 text-red-800' :
                            line.type === 'modified' ? 'bg-blue-50 text-blue-800' :
                            'bg-white'
                          }`}
                        >
                          <span className="w-8 text-right text-gray-500 select-none">
                            {line.oldLineNumber || line.newLineNumber || index + 1}
                          </span>
                          <span className="w-4 text-gray-500 select-none">
                            {line.type === 'added' ? '+' : line.type === 'removed' ? '-' : ' '}
                          </span>
                          <span className="flex-1 whitespace-pre-wrap break-all">
                            {line.content}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              )}
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
