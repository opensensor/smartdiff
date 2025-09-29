# Smart Code Diff API Integration Guide

This guide provides comprehensive examples and best practices for integrating with the Smart Code Diff API.

## Table of Contents

- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [API Endpoints](#api-endpoints)
- [Code Examples](#code-examples)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Best Practices](#best-practices)

## Quick Start

The Smart Code Diff API is a RESTful API that accepts JSON requests and returns JSON responses. All endpoints are available at the base URL.

### Base URL
- Development: `http://localhost:3000`
- Production: `https://api.smartcodediff.com`

### Health Check

Before making any requests, verify the API is running:

```bash
curl -X GET http://localhost:3000/api/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "smart-code-diff",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "memory_usage": {
    "used_mb": 256.5,
    "available_mb": 1024.0,
    "peak_mb": 512.0
  },
  "components": {
    "parser": {
      "status": "healthy",
      "last_check": "2024-01-15T10:30:00Z",
      "details": "Parser engine operational"
    }
  }
}
```

## Authentication

Currently, the API does not require authentication. This may change in future versions.

## API Endpoints

### 1. File Comparison

Compare two source code files with comprehensive analysis.

**Endpoint:** `POST /api/compare`

**Request Example:**
```bash
curl -X POST http://localhost:3000/api/compare \
  -H "Content-Type: application/json" \
  -d '{
    "file1": {
      "path": "Calculator.java",
      "content": "public class Calculator {\n    public int add(int a, int b) {\n        return a + b;\n    }\n}"
    },
    "file2": {
      "path": "Calculator.java", 
      "content": "public class Calculator {\n    public int add(int a, int b) {\n        return a + b;\n    }\n    public int subtract(int a, int b) {\n        return a - b;\n    }\n}"
    },
    "options": {
      "threshold": 0.7,
      "ignore_whitespace": true,
      "detect_moves": true
    }
  }'
```

**Response Structure:**
```json
{
  "similarity": 0.85,
  "analysis": {
    "files": {
      "source": {
        "path": "Calculator.java",
        "lines": 5,
        "functions": 1,
        "classes": 1,
        "complexity": 1.0
      },
      "target": {
        "path": "Calculator.java",
        "lines": 8,
        "functions": 2,
        "classes": 1,
        "complexity": 2.0
      },
      "language": "java",
      "similarity": {
        "overall": 0.85,
        "structure": 0.90,
        "content": 0.80,
        "semantic": 0.85
      }
    },
    "functions": {
      "total_functions": 2,
      "matched_functions": 1,
      "function_matches": [
        {
          "id": "func-1",
          "source_function": {
            "name": "add",
            "signature": "public int add(int a, int b)",
            "start_line": 2,
            "end_line": 4,
            "complexity": 1,
            "parameters": ["int a", "int b"],
            "return_type": "int"
          },
          "target_function": {
            "name": "add",
            "signature": "public int add(int a, int b)",
            "start_line": 2,
            "end_line": 4,
            "complexity": 1,
            "parameters": ["int a", "int b"],
            "return_type": "int"
          },
          "similarity": {
            "overall": 1.0,
            "structure": 1.0,
            "content": 1.0,
            "semantic": 1.0
          },
          "change_type": "unchanged"
        }
      ],
      "average_similarity": 1.0
    },
    "changes": {
      "total_changes": 1,
      "change_types": {
        "added": 1
      },
      "detailed_changes": [
        {
          "id": "change-1",
          "change_type": "added",
          "description": "Function 'subtract' added",
          "confidence": 0.95,
          "location": {
            "file": "Calculator.java",
            "start_line": 5,
            "end_line": 7,
            "function": "subtract"
          },
          "impact": "low"
        }
      ],
      "impact_assessment": {
        "risk_level": "low",
        "breaking_changes": 0,
        "effort_estimate": "low",
        "affected_components": ["Calculator"]
      }
    },
    "refactoring_patterns": [],
    "structure": {
      "source_structure": {
        "id": "root-1",
        "name": "Calculator.java",
        "node_type": "file",
        "children": [],
        "metadata": {}
      },
      "target_structure": {
        "id": "root-2", 
        "name": "Calculator.java",
        "node_type": "file",
        "children": [],
        "metadata": {}
      },
      "matches": []
    }
  },
  "execution_time_ms": 42
}
```

### 2. Multi-File Analysis

Analyze multiple files with cross-file dependency detection.

**Endpoint:** `POST /api/analyze`

**Request Example:**
```bash
curl -X POST http://localhost:3000/api/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "files": [
      {
        "path": "Calculator.java",
        "content": "public class Calculator { ... }"
      },
      {
        "path": "MathUtils.java", 
        "content": "public class MathUtils { ... }"
      }
    ],
    "options": {
      "include_complexity": true,
      "include_dependencies": true,
      "include_signatures": true,
      "similarity_threshold": 0.8
    }
  }'
```

### 3. Configuration Management

Update system configuration dynamically.

**Endpoint:** `POST /api/configure`

**Request Example:**
```bash
curl -X POST http://localhost:3000/api/configure \
  -H "Content-Type: application/json" \
  -d '{
    "parser": {
      "max_file_size": 10485760,
      "parse_timeout": 30,
      "enable_error_recovery": true
    },
    "semantic": {
      "max_resolution_depth": 10,
      "enable_cross_file_analysis": true,
      "symbol_cache_size": 1000
    },
    "diff_engine": {
      "default_similarity_threshold": 0.7,
      "enable_refactoring_detection": true,
      "enable_cross_file_tracking": true,
      "max_tree_depth": 20
    }
  }'
```

## Code Examples

### JavaScript/Node.js

```javascript
const axios = require('axios');

class SmartCodeDiffClient {
  constructor(baseURL = 'http://localhost:3000') {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  async compareFiles(file1, file2, options = {}) {
    try {
      const response = await this.client.post('/api/compare', {
        file1,
        file2,
        options
      });
      return response.data;
    } catch (error) {
      throw new Error(`Comparison failed: ${error.response?.data?.error || error.message}`);
    }
  }

  async analyzeFiles(files, options = {}) {
    try {
      const response = await this.client.post('/api/analyze', {
        files,
        options
      });
      return response.data;
    } catch (error) {
      throw new Error(`Analysis failed: ${error.response?.data?.error || error.message}`);
    }
  }

  async getHealth() {
    try {
      const response = await this.client.get('/api/health');
      return response.data;
    } catch (error) {
      throw new Error(`Health check failed: ${error.message}`);
    }
  }
}

// Usage example
const client = new SmartCodeDiffClient();

async function example() {
  // Check API health
  const health = await client.getHealth();
  console.log('API Status:', health.status);

  // Compare two files
  const comparison = await client.compareFiles(
    {
      path: 'old.js',
      content: 'function hello() { console.log("Hello"); }'
    },
    {
      path: 'new.js', 
      content: 'function hello() { console.log("Hello World"); }'
    },
    {
      threshold: 0.8,
      ignore_whitespace: true
    }
  );

  console.log('Similarity:', comparison.similarity);
  console.log('Changes:', comparison.analysis.changes.total_changes);
}

example().catch(console.error);
```

### Python

```python
import requests
import json

class SmartCodeDiffClient:
    def __init__(self, base_url='http://localhost:3000'):
        self.base_url = base_url
        self.session = requests.Session()
        self.session.headers.update({'Content-Type': 'application/json'})
    
    def compare_files(self, file1, file2, options=None):
        """Compare two source code files."""
        if options is None:
            options = {}
        
        payload = {
            'file1': file1,
            'file2': file2,
            'options': options
        }
        
        response = self.session.post(f'{self.base_url}/api/compare', 
                                   json=payload)
        response.raise_for_status()
        return response.json()
    
    def analyze_files(self, files, options=None):
        """Analyze multiple files."""
        if options is None:
            options = {}
        
        payload = {
            'files': files,
            'options': options
        }
        
        response = self.session.post(f'{self.base_url}/api/analyze',
                                   json=payload)
        response.raise_for_status()
        return response.json()
    
    def get_health(self):
        """Get API health status."""
        response = self.session.get(f'{self.base_url}/api/health')
        response.raise_for_status()
        return response.json()

# Usage example
client = SmartCodeDiffClient()

# Check API health
health = client.get_health()
print(f"API Status: {health['status']}")

# Compare files
result = client.compare_files(
    file1={
        'path': 'calculator.py',
        'content': '''
def add(a, b):
    return a + b

def multiply(a, b):
    return a * b
'''
    },
    file2={
        'path': 'calculator.py',
        'content': '''
def add(a, b):
    """Add two numbers."""
    return a + b

def multiply(a, b):
    """Multiply two numbers."""
    return a * b

def subtract(a, b):
    """Subtract two numbers."""
    return a - b
'''
    },
    options={
        'threshold': 0.7,
        'ignore_whitespace': True,
        'detect_moves': True
    }
)

print(f"Similarity: {result['similarity']:.2f}")
print(f"Changes detected: {result['analysis']['changes']['total_changes']}")
```

## Error Handling

The API returns standard HTTP status codes and JSON error responses:

### Common Error Responses

**400 Bad Request:**
```json
{
  "error": "Invalid request format",
  "details": "Missing required field 'file1'",
  "error_code": "INVALID_REQUEST"
}
```

**500 Internal Server Error:**
```json
{
  "error": "Analysis failed",
  "details": "Parser error: Unsupported language",
  "error_code": "ANALYSIS_ERROR"
}
```

### Error Handling Best Practices

1. **Always check HTTP status codes**
2. **Parse error responses for detailed information**
3. **Implement retry logic for transient errors**
4. **Log errors for debugging**

```javascript
try {
  const result = await client.compareFiles(file1, file2);
  return result;
} catch (error) {
  if (error.response?.status === 400) {
    console.error('Invalid request:', error.response.data.details);
  } else if (error.response?.status === 500) {
    console.error('Server error:', error.response.data.error);
    // Implement retry logic
  } else {
    console.error('Network error:', error.message);
  }
  throw error;
}
```

## Rate Limiting

Currently, there are no rate limits imposed. However, for optimal performance:

- **Batch requests** when possible using the `/api/analyze` endpoint
- **Implement client-side caching** for repeated comparisons
- **Use appropriate timeouts** for large file comparisons

## Best Practices

### 1. File Size Considerations

- **Maximum file size**: 10MB by default (configurable)
- **Large files**: Consider splitting into smaller chunks
- **Binary files**: Not supported, text files only

### 2. Performance Optimization

- **Use appropriate similarity thresholds** (0.7 is recommended)
- **Enable whitespace ignoring** for formatting changes
- **Cache results** for repeated comparisons

### 3. Language Support

Supported languages:
- Java
- Python  
- JavaScript
- C++
- C

### 4. Configuration Management

- **Update configuration** based on your use case
- **Monitor memory usage** for large-scale analysis
- **Adjust timeouts** for complex comparisons

### 5. Integration Patterns

**Webhook Integration:**
```javascript
// Example webhook handler for CI/CD integration
app.post('/webhook/code-review', async (req, res) => {
  const { oldCode, newCode } = req.body;
  
  try {
    const comparison = await client.compareFiles(oldCode, newCode);
    
    if (comparison.similarity < 0.5) {
      // Significant changes detected
      await notifyReviewers(comparison);
    }
    
    res.json({ status: 'processed', similarity: comparison.similarity });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});
```

**Batch Processing:**
```python
def analyze_repository(file_paths):
    """Analyze all files in a repository."""
    files = []
    for path in file_paths:
        with open(path, 'r') as f:
            files.append({
                'path': path,
                'content': f.read()
            })
    
    # Analyze in batches of 10 files
    batch_size = 10
    results = []
    
    for i in range(0, len(files), batch_size):
        batch = files[i:i + batch_size]
        result = client.analyze_files(batch, {
            'include_complexity': True,
            'include_dependencies': True
        })
        results.append(result)
    
    return results
```

For more examples and advanced usage patterns, see the [examples directory](../examples/) in the repository.
