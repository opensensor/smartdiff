// File system types
export interface FileSystemEntry {
  name: string;
  path: string;
  is_directory: boolean;
  size?: number;
  modified?: string;
  language?: string;
  children?: FileSystemEntry[];
}

export interface BrowseDirectoryRequest {
  path: string;
  recursive: boolean;
  include_hidden: boolean;
  max_depth?: number;
}

export interface BrowseDirectoryResponse {
  entries: FileSystemEntry[];
  total_files: number;
  total_directories: number;
  processing_time_ms: number;
}

// Function and code analysis types
export interface FunctionSignature {
  name: string;
  parameters: Parameter[];
  return_type?: Type;
  modifiers: string[];
  generic_parameters: string[];
}

export interface Parameter {
  name: string;
  param_type: Type;
  default_value?: string;
  is_variadic: boolean;
}

export interface Type {
  name: string;
  generic_args: Type[];
  is_nullable: boolean;
  is_array: boolean;
  array_dimensions: number;
}

export interface Function {
  signature: FunctionSignature;
  body: ASTNode;
  dependencies: string[];
  hash: string;
  location: FunctionLocation;
}

export interface FunctionLocation {
  file_path: string;
  start_line: number;
  end_line: number;
  start_column: number;
  end_column: number;
}

export interface ASTNode {
  node_type: string;
  children: ASTNode[];
  metadata: NodeMetadata;
}

export interface NodeMetadata {
  line: number;
  column: number;
  attributes: Record<string, string>;
}

// Graph-based matching types
export interface GraphMatchResult {
  matches: FunctionMatch[];
  moves: FunctionMove[];
  renames: FunctionRename[];
  additions: string[];
  deletions: string[];
  overall_similarity: number;
  dependency_changes: DependencyChange[];
}

export interface FunctionMatch {
  source_id: string;
  target_id: string;
  similarity: ComprehensiveSimilarityScore;
  context_similarity: number;
  confidence: number;
  match_type: MatchType;
}

export interface FunctionMove {
  function_id: string;
  source_file: string;
  target_file: string;
  similarity: number;
  confidence: number;
}

export interface FunctionRename {
  old_name: string;
  new_name: string;
  function_id: string;
  similarity: number;
  confidence: number;
}

export interface DependencyChange {
  change_type: 'Added' | 'Removed' | 'Modified' | 'Strengthened' | 'Weakened';
  source_function: string;
  target_function: string;
  edge_type: string;
  strength_change: number;
}

export interface ComprehensiveSimilarityScore {
  overall_similarity: number;
  signature_similarity: FunctionSignatureSimilarity;
  body_similarity: ASTSimilarityScore;
  context_similarity: ContextSimilarityScore;
  semantic_metrics: SemanticSimilarityMetrics;
  confidence: number;
  match_type: MatchType;
  similarity_breakdown: DetailedSimilarityBreakdown;
}

export interface FunctionSignatureSimilarity {
  overall_similarity: number;
  name_similarity: number;
  parameter_similarity: number;
  return_type_similarity: number;
  modifier_similarity: number;
  complexity_similarity: number;
  is_potential_match: boolean;
}

export interface ASTSimilarityScore {
  overall_similarity: number;
  structural_similarity: number;
  semantic_similarity: number;
  edit_distance: number;
  normalized_edit_distance: number;
}

export interface ContextSimilarityScore {
  overall_similarity: number;
  dependency_similarity: number;
  usage_pattern_similarity: number;
  call_graph_similarity: number;
}

export interface SemanticSimilarityMetrics {
  variable_usage_similarity: number;
  control_flow_similarity: number;
  data_flow_similarity: number;
  complexity_similarity: number;
}

export interface DetailedSimilarityBreakdown {
  exact_match: boolean;
  high_confidence_match: boolean;
  potential_rename: boolean;
  potential_move: boolean;
  structural_changes: string[];
  semantic_changes: string[];
}

export type MatchType = 
  | 'Exact'
  | 'Similar' 
  | 'Renamed'
  | 'Moved'
  | 'MovedAndRenamed'
  | 'Refactored';

// UI State types
export interface ComparisonState {
  sourceDirectory?: string;
  targetDirectory?: string;
  selectedSourceFiles: string[];
  selectedTargetFiles: string[];
  comparisonResult?: GraphMatchResult;
  isLoading: boolean;
  error?: string;
}

export interface ViewState {
  activeView: 'files' | 'diff' | 'graph' | 'analysis';
  sidebarCollapsed: boolean;
  splitPaneSize: number;
  selectedFunction?: string;
  highlightedMatches: string[];
}

// API Response types
export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
  processing_time_ms: number;
}

export interface HealthResponse {
  status: string;
  version: string;
  uptime_seconds: number;
}
