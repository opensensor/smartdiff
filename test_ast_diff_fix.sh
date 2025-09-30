#!/bin/bash

# Test script to verify AST diff fix

echo "Testing AST diff fix..."
echo ""

echo "=== Testing LCS Algorithm ==="
curl -s -X POST http://localhost:8080/api/ast/diff \
  -H "Content-Type: application/json" \
  -d '{
    "source_content": "function hello() {\n  console.log(\"old\");\n  return 42;\n}",
    "target_content": "function hello() {\n  console.log(\"new\");\n  return 99;\n}",
    "source_file_path": "test.js",
    "target_file_path": "test.js",
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
  }' | jq '{
    summary: .summary,
    line_count: (.line_mappings | length),
    first_3_lines: [.line_mappings[0:3][] | {change_type, source_line, target_line}]
  }'

echo ""
echo "=== Testing AST Algorithm (should match LCS now) ==="
curl -s -X POST http://localhost:8080/api/ast/diff \
  -H "Content-Type: application/json" \
  -d '{
    "source_content": "function hello() {\n  console.log(\"old\");\n  return 42;\n}",
    "target_content": "function hello() {\n  console.log(\"new\");\n  return 99;\n}",
    "source_file_path": "test.js",
    "target_file_path": "test.js",
    "language": "javascript",
    "options": {
      "enable_semantic_analysis": true,
      "enable_structural_analysis": true,
      "generate_line_mapping": true,
      "diff_algorithm": "ast",
      "use_tree_edit_distance": true,
      "use_hungarian_matching": true,
      "ignore_whitespace": false
    }
  }' | jq '{
    summary: .summary,
    line_count: (.line_mappings | length),
    first_3_lines: [.line_mappings[0:3][] | {change_type, source_line, target_line, ast_node_type}]
  }'

echo ""
echo "✓ Expected: Both should show 4 lines total, with 2 modified lines"
echo "✓ Expected: Line numbers should be sequential (1, 2, 3, 4) not duplicated"
echo "✓ Expected: AST version should have ast_node_type field populated"

