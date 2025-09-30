# Ignore Whitespace Feature

## Overview

Added an "Ignore Whitespace" option to the diff viewer that allows users to toggle whether whitespace changes should be considered when comparing code.

## Changes Made

### Frontend Changes

#### 1. `nextjs-frontend/src/components/diff/BeyondCompareFunctionDiff.tsx`

**Added State:**
- Added `ignoreWhitespace` state variable (default: `true` - whitespace ignored by default)

**Updated API Call:**
- Added `ignore_whitespace` parameter to the Rust backend API request options
- Added `ignoreWhitespace` to the `useEffect` dependency array to trigger re-fetch when toggled

**Added UI Toggle:**
- Added a checkbox control in the diff viewer header
- Positioned next to the View Mode toggle (Unified/Side-by-Side)
- Styled consistently with the existing UI

```tsx
{/* Ignore Whitespace Toggle */}
<div className="flex items-center gap-2">
  <label className="flex items-center gap-2 cursor-pointer">
    <input
      type="checkbox"
      checked={ignoreWhitespace}
      onChange={(e) => setIgnoreWhitespace(e.target.checked)}
      className="w-4 h-4 rounded border-slate-600 bg-slate-700 text-blue-600 focus:ring-2 focus:ring-blue-500 focus:ring-offset-0 cursor-pointer"
    />
    <span className="text-xs font-medium text-slate-300">Ignore Whitespace</span>
  </label>
</div>
```

### Backend Changes

#### 1. `crates/web-ui/src/models.rs`

**Updated `ASTDiffOptions` struct:**
- Added `ignore_whitespace: bool` field with `#[serde(default)]` attribute
- Defaults to `false` when not provided

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ASTDiffOptions {
    // ... existing fields ...
    /// Ignore whitespace when comparing lines
    #[serde(default)]
    pub ignore_whitespace: bool,
}
```

#### 2. `crates/web-ui/src/handlers.rs`

**Updated `ast_diff` handler:**
- Passes `ignore_whitespace` option to `generate_lcs_line_mappings` function

**Updated `generate_lcs_line_mappings` function:**
- Added `ignore_whitespace: bool` parameter
- Passes the parameter to `compute_diff_operations`

**Updated `compute_diff_operations` function:**
- Added `ignore_whitespace: bool` parameter
- Conditionally compares lines with or without trimming based on the flag
- Passes the parameter to `compute_lcs_table`

```rust
let lines_equal = if ignore_whitespace {
    source_lines[i - 1].trim() == target_lines[j - 1].trim()
} else {
    source_lines[i - 1] == target_lines[j - 1]
};
```

**Updated `compute_lcs_table` function:**
- Added `ignore_whitespace: bool` parameter
- Conditionally compares lines with or without trimming in the LCS algorithm

## How It Works

### When `ignore_whitespace = false` (default):
- Lines are compared exactly as they appear
- Whitespace changes (indentation, trailing spaces, etc.) are detected as modifications
- Example: `"  console.log()"` vs `"    console.log()"` → **Modified**

### When `ignore_whitespace = true`:
- Lines are trimmed before comparison
- Only non-whitespace content is compared
- Whitespace-only changes are ignored
- Example: `"  console.log()"` vs `"    console.log()"` → **Unchanged**

## Testing

A test script `test_ignore_whitespace.sh` was created to verify the functionality:

### Test Results:

**Test 1: ignore_whitespace = false**
```json
{
  "total_lines": 3,
  "added_lines": 0,
  "deleted_lines": 0,
  "modified_lines": 1,
  "unchanged_lines": 2,
  "structural_changes": 0,
  "semantic_changes": 0
}
```
✅ Correctly detects whitespace change as a modification

**Test 2: ignore_whitespace = true**
```json
{
  "total_lines": 3,
  "added_lines": 0,
  "deleted_lines": 0,
  "modified_lines": 0,
  "unchanged_lines": 3,
  "structural_changes": 0,
  "semantic_changes": 0
}
```
✅ Correctly ignores whitespace change

## Usage

1. Open the diff viewer by clicking on a function pair in the comparison view
2. Look for the "Ignore Whitespace" checkbox in the diff viewer header
3. Toggle the checkbox to enable/disable whitespace ignoring
4. The diff will automatically refresh when the option is changed

## Implementation Notes

- The feature currently applies to the **LCS-based diff algorithm** (default)
- The AST-based diff algorithm uses structural similarity and is less affected by whitespace
- The option is passed through the entire diff pipeline from frontend to backend
- The implementation uses Rust's string trimming for whitespace normalization
- The feature is backward compatible - if not specified, defaults to `false`

## Future Enhancements

Potential improvements:
- Add more granular whitespace options (ignore leading, ignore trailing, ignore all)
- Add option to ignore blank lines
- Add option to ignore case
- Persist user preference in local storage
- Add keyboard shortcut to toggle the option

