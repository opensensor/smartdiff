#!/bin/bash

# Test script to verify ignore_whitespace functionality

echo "Testing ignore_whitespace functionality..."
echo ""

# Test 1: With ignore_whitespace = false (should show differences)
echo "Test 1: ignore_whitespace = false (should detect whitespace changes)"
curl -s -X POST http://localhost:8080/api/ast/diff \
  -H "Content-Type: application/json" \
  -d '{
    "source_content": "function hello() {\n  console.log(\"hello\");\n}",
    "target_content": "function hello() {\n    console.log(\"hello\");\n}",
    "source_file_path": "test1.js",
    "target_file_path": "test1.js",
    "language": "javascript",
    "options": {
      "enable_semantic_analysis": false,
      "enable_structural_analysis": false,
      "generate_line_mapping": true,
      "diff_algorithm": "lcs",
      "use_tree_edit_distance": false,
      "use_hungarian_matching": false,
      "ignore_whitespace": false
    }
  }' | jq '.summary'

echo ""
echo "---"
echo ""

# Test 2: With ignore_whitespace = true (should NOT show differences)
echo "Test 2: ignore_whitespace = true (should NOT detect whitespace changes)"
curl -s -X POST http://localhost:8080/api/ast/diff \
  -H "Content-Type: application/json" \
  -d '{
    "source_content": "function hello() {\n  console.log(\"hello\");\n}",
    "target_content": "function hello() {\n    console.log(\"hello\");\n}",
    "source_file_path": "test2.js",
    "target_file_path": "test2.js",
    "language": "javascript",
    "options": {
      "enable_semantic_analysis": false,
      "enable_structural_analysis": false,
      "generate_line_mapping": true,
      "diff_algorithm": "lcs",
      "use_tree_edit_distance": false,
      "use_hungarian_matching": false,
      "ignore_whitespace": true
    }
  }' | jq '.summary'

echo ""
echo "---"
echo ""
echo "Expected results:"
echo "  Test 1: Should show modified_lines > 0 (whitespace detected)"
echo "  Test 2: Should show modified_lines = 0 (whitespace ignored)"

