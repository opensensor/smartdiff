'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { 
  Code, 
  GitCompare, 
  TrendingUp, 
  TrendingDown,
  ArrowRight,
  FileText,
  Hash,
  Clock,
  MapPin,
  Layers,
  Zap,
  Target,
  BarChart3
} from 'lucide-react';
import { FunctionMatch, ComprehensiveSimilarityScore } from '@/types';

interface FunctionDetailViewProps {
  match: FunctionMatch;
  onClose?: () => void;
}

export function FunctionDetailView({ match, onClose }: FunctionDetailViewProps) {
  const [activeTab, setActiveTab] = useState<'overview' | 'similarity' | 'changes'>('overview');

  const similarity = match.similarity;

  // Get similarity level color and label
  const getSimilarityLevel = (score: number) => {
    if (score >= 0.9) return { label: 'Excellent', color: 'text-green-600', bg: 'bg-green-100' };
    if (score >= 0.7) return { label: 'Good', color: 'text-blue-600', bg: 'bg-blue-100' };
    if (score >= 0.5) return { label: 'Fair', color: 'text-yellow-600', bg: 'bg-yellow-100' };
    return { label: 'Poor', color: 'text-red-600', bg: 'bg-red-100' };
  };

  const overallLevel = getSimilarityLevel(similarity.overall_similarity);

  return (
    <div className="space-y-6">
      {/* Header */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <GitCompare className="w-5 h-5" />
              Function Comparison Details
            </CardTitle>
            {onClose && (
              <Button variant="outline" onClick={onClose}>
                Close
              </Button>
            )}
          </div>
          
          <div className="flex items-center gap-4 text-sm">
            <div className="flex items-center gap-2">
              <Code className="w-4 h-4" />
              <span className="font-medium">{match.source_id}</span>
            </div>
            <ArrowRight className="w-4 h-4 text-muted-foreground" />
            <div className="flex items-center gap-2">
              <Code className="w-4 h-4" />
              <span className="font-medium">{match.target_id}</span>
            </div>
          </div>
        </CardHeader>
      </Card>

      {/* Tab Navigation */}
      <div className="flex items-center gap-2">
        <Button
          variant={activeTab === 'overview' ? 'default' : 'outline'}
          size="sm"
          onClick={() => setActiveTab('overview')}
        >
          <FileText className="w-4 h-4 mr-2" />
          Overview
        </Button>
        <Button
          variant={activeTab === 'similarity' ? 'default' : 'outline'}
          size="sm"
          onClick={() => setActiveTab('similarity')}
        >
          <Target className="w-4 h-4 mr-2" />
          Similarity Analysis
        </Button>
        <Button
          variant={activeTab === 'changes' ? 'default' : 'outline'}
          size="sm"
          onClick={() => setActiveTab('changes')}
        >
          <BarChart3 className="w-4 h-4 mr-2" />
          Change Analysis
        </Button>
      </div>

      {/* Tab Content */}
      {activeTab === 'overview' && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Overall Metrics */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Overall Metrics</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">Overall Similarity</span>
                <div className="flex items-center gap-2">
                  <Badge className={`${overallLevel.bg} ${overallLevel.color}`}>
                    {overallLevel.label}
                  </Badge>
                  <span className="font-bold">{(similarity.overall_similarity * 100).toFixed(1)}%</span>
                </div>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">Match Confidence</span>
                <span className="font-bold">{(match.confidence * 100).toFixed(1)}%</span>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">Match Type</span>
                <Badge variant="outline">{match.match_type}</Badge>
              </div>
              
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">Context Similarity</span>
                <span className="font-bold">{(match.context_similarity * 100).toFixed(1)}%</span>
              </div>
            </CardContent>
          </Card>

          {/* Match Characteristics */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Match Characteristics</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div className="flex items-center gap-2">
                  {similarity.similarity_breakdown.exact_match ? (
                    <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                  ) : (
                    <div className="w-3 h-3 bg-gray-300 rounded-full"></div>
                  )}
                  <span className="text-sm">Exact Match</span>
                </div>
                
                <div className="flex items-center gap-2">
                  {similarity.similarity_breakdown.high_confidence_match ? (
                    <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                  ) : (
                    <div className="w-3 h-3 bg-gray-300 rounded-full"></div>
                  )}
                  <span className="text-sm">High Confidence</span>
                </div>
                
                <div className="flex items-center gap-2">
                  {similarity.similarity_breakdown.potential_rename ? (
                    <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                  ) : (
                    <div className="w-3 h-3 bg-gray-300 rounded-full"></div>
                  )}
                  <span className="text-sm">Potential Rename</span>
                </div>
                
                <div className="flex items-center gap-2">
                  {similarity.similarity_breakdown.potential_move ? (
                    <div className="w-3 h-3 bg-blue-500 rounded-full"></div>
                  ) : (
                    <div className="w-3 h-3 bg-gray-300 rounded-full"></div>
                  )}
                  <span className="text-sm">Potential Move</span>
                </div>
              </div>
              
              {similarity.similarity_breakdown.structural_changes.length > 0 && (
                <div>
                  <div className="text-sm font-medium mb-2">Structural Changes</div>
                  <div className="space-y-1">
                    {similarity.similarity_breakdown.structural_changes.slice(0, 3).map((change, index) => (
                      <Badge key={index} variant="outline" className="text-xs">
                        {change}
                      </Badge>
                    ))}
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      )}

      {activeTab === 'similarity' && (
        <div className="space-y-6">
          {/* Signature Similarity */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Hash className="w-5 h-5" />
                Signature Similarity
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                <div className="text-center">
                  <div className="text-2xl font-bold text-blue-600">
                    {(similarity.signature_similarity.overall_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Overall</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.signature_similarity.name_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Name</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.signature_similarity.parameter_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Parameters</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.signature_similarity.return_type_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Return Type</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.signature_similarity.modifier_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Modifiers</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.signature_similarity.complexity_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Complexity</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Body Similarity */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Layers className="w-5 h-5" />
                Body Similarity (AST Analysis)
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center">
                  <div className="text-2xl font-bold text-green-600">
                    {(similarity.body_similarity.overall_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Overall</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.body_similarity.structural_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Structural</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.body_similarity.semantic_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Semantic</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {similarity.body_similarity.edit_distance}
                  </div>
                  <div className="text-sm text-muted-foreground">Edit Distance</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Context Similarity */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <MapPin className="w-5 h-5" />
                Context Similarity
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="text-center">
                  <div className="text-2xl font-bold text-purple-600">
                    {(similarity.context_similarity.overall_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Overall Context</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.context_similarity.dependency_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Dependencies</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.context_similarity.usage_pattern_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Usage Patterns</div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Semantic Metrics */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Zap className="w-5 h-5" />
                Semantic Metrics
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.semantic_metrics.variable_usage_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Variable Usage</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.semantic_metrics.control_flow_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Control Flow</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.semantic_metrics.data_flow_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Data Flow</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold">
                    {(similarity.semantic_metrics.complexity_similarity * 100).toFixed(1)}%
                  </div>
                  <div className="text-sm text-muted-foreground">Complexity</div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {activeTab === 'changes' && (
        <div className="space-y-6">
          {/* Change Summary */}
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Change Summary</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {similarity.similarity_breakdown.structural_changes.length > 0 && (
                  <div>
                    <h4 className="font-medium mb-2 flex items-center gap-2">
                      <Layers className="w-4 h-4" />
                      Structural Changes
                    </h4>
                    <div className="space-y-1">
                      {similarity.similarity_breakdown.structural_changes.map((change, index) => (
                        <div key={index} className="flex items-center gap-2 text-sm">
                          <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
                          <span>{change}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {similarity.similarity_breakdown.semantic_changes.length > 0 && (
                  <div>
                    <h4 className="font-medium mb-2 flex items-center gap-2">
                      <Zap className="w-4 h-4" />
                      Semantic Changes
                    </h4>
                    <div className="space-y-1">
                      {similarity.similarity_breakdown.semantic_changes.map((change, index) => (
                        <div key={index} className="flex items-center gap-2 text-sm">
                          <div className="w-2 h-2 bg-orange-500 rounded-full"></div>
                          <span>{change}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {similarity.similarity_breakdown.structural_changes.length === 0 && 
                 similarity.similarity_breakdown.semantic_changes.length === 0 && (
                  <div className="text-center py-8 text-muted-foreground">
                    <FileText className="w-12 h-12 mx-auto mb-4 opacity-50" />
                    <p>No detailed change information available</p>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
