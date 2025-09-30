# Default: Ignore Whitespace ON

## Change

Updated the default value of `ignoreWhitespace` from `false` to `true` in the diff viewer.

## Rationale

### Why Ignore Whitespace by Default?

1. **Formatting changes are noise** - Most of the time, users care about logical changes, not indentation or spacing
2. **Common use case** - Code formatters (prettier, black, rustfmt, etc.) often change whitespace without changing logic
3. **Better signal-to-noise ratio** - Focusing on meaningful changes makes code review more efficient
4. **Easy to toggle off** - Users who want to see whitespace changes can simply uncheck the box

### Real-World Scenarios:

**Scenario 1: Code Formatter Applied**
- Developer runs `cargo fmt` or `prettier` on a file
- Without ignore whitespace: Hundreds of "changed" lines
- With ignore whitespace: Only actual logic changes shown ✅

**Scenario 2: Indentation Change**
- Code block moved inside a new function/if statement
- Without ignore whitespace: Every line shows as modified
- With ignore whitespace: Only lines with actual changes shown ✅

**Scenario 3: Line Ending Changes**
- Windows (CRLF) vs Unix (LF) line endings
- Without ignore whitespace: Every line appears different
- With ignore whitespace: No false positives ✅

## Implementation

**File:** `nextjs-frontend/src/components/diff/BeyondCompareFunctionDiff.tsx`

**Change:**
```typescript
// Before:
const [ignoreWhitespace, setIgnoreWhitespace] = React.useState(false);

// After:
const [ignoreWhitespace, setIgnoreWhitespace] = React.useState(true);
```

**Line:** 53

## User Experience

### Before:
- Whitespace changes shown by default
- User had to manually check "Ignore Whitespace" for every diff
- Noisy diffs with formatting changes

### After:
- Whitespace changes ignored by default ✅
- Cleaner diffs focused on logic changes ✅
- User can uncheck if they need to see whitespace ✅

## UI Behavior

The checkbox in the diff viewer:
- **Checked by default** - "Ignore Whitespace" is ON
- **Unchecking** - Shows whitespace changes (for when you need to verify formatting)
- **State persists** - During the current session (not across page reloads)

## Edge Cases

### When You Might Want to See Whitespace:

1. **Reviewing formatter changes** - Verifying that auto-formatting was applied correctly
2. **Debugging whitespace-sensitive languages** - Python, Makefile, YAML, etc.
3. **Checking indentation consistency** - Ensuring code style guidelines are followed
4. **Investigating rendering issues** - HTML/CSS where whitespace can affect layout

For these cases, users can simply uncheck the "Ignore Whitespace" box.

## Testing

To verify the change:

1. **Open a diff** - The "Ignore Whitespace" checkbox should be checked by default
2. **View a file with formatting changes** - Should show only logical changes
3. **Uncheck the box** - Should now show whitespace changes
4. **Re-check the box** - Should hide whitespace changes again

## Benefits

- ✅ **Less noise** - Diffs focus on what matters
- ✅ **Faster review** - Don't waste time on formatting changes
- ✅ **Better defaults** - Matches most common use case
- ✅ **Still flexible** - Easy to toggle when needed

## Related Changes

This complements the other default improvements:
- Default filter: "Modified" (not "All")
- Default sort: Most different first
- Default file display: Only files with matching functions

All defaults now optimize for the most common use case: **reviewing meaningful code changes efficiently**.

## Documentation Updated

- `CHANGES_SUMMARY.md` - Added whitespace default to summary
- `IGNORE_WHITESPACE_FEATURE.md` - Updated default value documentation
- `DEFAULT_IGNORE_WHITESPACE.md` - This document

## Conclusion

Ignoring whitespace by default makes the diff viewer more useful out-of-the-box. Users can still see whitespace changes when needed, but the default behavior now matches the most common use case: focusing on logical changes while filtering out formatting noise.

