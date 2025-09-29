import axios, { AxiosInstance, AxiosResponse } from 'axios';
import {
  ApiResponse,
  BrowseDirectoryRequest,
  BrowseDirectoryResponse,
  HealthResponse,
  GraphMatchResult,
} from '@/types';

class ApiClient {
  private client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000',
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        console.log(`API Request: ${config.method?.toUpperCase()} ${config.url}`);
        return config;
      },
      (error) => {
        console.error('API Request Error:', error);
        return Promise.reject(error);
      }
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response: AxiosResponse<ApiResponse<any>>) => {
        console.log(`API Response: ${response.status} ${response.config.url}`);
        return response;
      },
      (error) => {
        console.error('API Response Error:', error);
        return Promise.reject(error);
      }
    );
  }

  // Health check
  async health(): Promise<HealthResponse> {
    const response = await this.client.get<ApiResponse<HealthResponse>>('/api/health');
    return response.data.data;
  }

  // File system operations
  async browseDirectory(request: BrowseDirectoryRequest): Promise<BrowseDirectoryResponse> {
    const response = await this.client.post<ApiResponse<BrowseDirectoryResponse>>(
      '/api/filesystem/browse',
      request
    );
    return response.data.data;
  }

  async readFile(filePath: string): Promise<string> {
    const response = await this.client.post<ApiResponse<{ content: string }>>(
      '/api/filesystem/read',
      { path: filePath }
    );
    return response.data.data.content;
  }

  async readMultipleFiles(filePaths: string[]): Promise<Record<string, string>> {
    const response = await this.client.post<ApiResponse<{ files: Record<string, string> }>>(
      '/api/filesystem/read-multiple',
      { paths: filePaths }
    );
    return response.data.data.files;
  }

  async searchFiles(
    directory: string,
    pattern: string,
    searchType: 'name' | 'content' | 'extension' = 'name'
  ): Promise<string[]> {
    const response = await this.client.post<ApiResponse<{ files: string[] }>>(
      '/api/filesystem/search',
      {
        directory,
        pattern,
        search_type: searchType,
      }
    );
    return response.data.data.files;
  }

  // Diff and comparison operations
  async compareDirectories(
    sourceDir: string,
    targetDir: string,
    options?: {
      recursive?: boolean;
      includeHidden?: boolean;
      fileExtensions?: string[];
    }
  ): Promise<GraphMatchResult> {
    const response = await this.client.post<ApiResponse<GraphMatchResult>>(
      '/api/diff/compare-directories',
      {
        source_directory: sourceDir,
        target_directory: targetDir,
        recursive: options?.recursive ?? true,
        include_hidden: options?.includeHidden ?? false,
        file_extensions: options?.fileExtensions,
      }
    );
    return response.data.data;
  }

  async compareFiles(
    sourceFile: string,
    targetFile: string,
    options?: {
      enableGraphMatching?: boolean;
      similarityThreshold?: number;
    }
  ): Promise<GraphMatchResult> {
    const response = await this.client.post<ApiResponse<GraphMatchResult>>(
      '/api/diff/compare-files',
      {
        source_file: sourceFile,
        target_file: targetFile,
        enable_graph_matching: options?.enableGraphMatching ?? true,
        similarity_threshold: options?.similarityThreshold ?? 0.7,
      }
    );
    return response.data.data;
  }

  // Function analysis
  async analyzeFunctions(filePath: string): Promise<any> {
    const response = await this.client.post<ApiResponse<any>>(
      '/api/analysis/functions',
      { file_path: filePath }
    );
    return response.data.data;
  }

  async getDependencyGraph(filePath: string): Promise<any> {
    const response = await this.client.post<ApiResponse<any>>(
      '/api/analysis/dependency-graph',
      { file_path: filePath }
    );
    return response.data.data;
  }

  // Language detection
  async detectLanguage(filePath: string): Promise<string> {
    const response = await this.client.post<ApiResponse<{ language: string }>>(
      '/api/language/detect',
      { file_path: filePath }
    );
    return response.data.data.language;
  }
}

// Create singleton instance
export const apiClient = new ApiClient();

// Export individual methods for easier use in React Query
export const api = {
  health: () => apiClient.health(),
  browseDirectory: (request: BrowseDirectoryRequest) => apiClient.browseDirectory(request),
  readFile: (filePath: string) => apiClient.readFile(filePath),
  readMultipleFiles: (filePaths: string[]) => apiClient.readMultipleFiles(filePaths),
  searchFiles: (directory: string, pattern: string, searchType?: 'name' | 'content' | 'extension') =>
    apiClient.searchFiles(directory, pattern, searchType),
  compareDirectories: (sourceDir: string, targetDir: string, options?: any) =>
    apiClient.compareDirectories(sourceDir, targetDir, options),
  compareFiles: (sourceFile: string, targetFile: string, options?: any) =>
    apiClient.compareFiles(sourceFile, targetFile, options),
  analyzeFunctions: (filePath: string) => apiClient.analyzeFunctions(filePath),
  getDependencyGraph: (filePath: string) => apiClient.getDependencyGraph(filePath),
  detectLanguage: (filePath: string) => apiClient.detectLanguage(filePath),
};
