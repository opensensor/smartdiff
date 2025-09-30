# Graph Performance Optimizations

## Overview

This document describes the performance optimizations implemented for the D3 force-directed graph visualizations to handle large changesets efficiently.

## Problems Identified

### 1. **Unbounded Force Simulation**
- The simulation ran continuously without proper alpha decay
- Nodes would drift infinitely far apart
- High CPU usage even after the graph should have settled

### 2. **O(n²) Link Creation**
- Creating links between all functions in the same file resulted in quadratic complexity
- For a file with 100 functions, this created 4,950 links
- Caused severe performance degradation with large changesets

### 3. **No Boundary Constraints**
- Nodes could move off-screen indefinitely
- Users had to zoom out excessively to find nodes
- Poor user experience with large graphs

### 4. **Fixed Force Parameters**
- Same force strengths used regardless of graph size
- Large graphs became unmanageable with strong repulsion forces

## Solutions Implemented

### 1. **Adaptive Force Parameters**

The graph now detects when it's dealing with a large dataset (>50 nodes) and adjusts forces accordingly:

```typescript
const nodeCount = filteredNodes.length;
const isLargeGraph = nodeCount > 50;

// Adjust forces based on graph size
const chargeStrength = isLargeGraph ? -100 : -300;
const linkDistance = isLargeGraph ? 50 : 100;
const collisionRadius = isLargeGraph ? 20 : 30;
```

**Benefits:**
- Large graphs use weaker repulsion forces to keep nodes closer together
- Shorter link distances prevent excessive spreading
- Smaller collision radii allow denser packing

### 2. **Optimized Link Creation**

Changed from O(n²) to O(n) for large file groups:

```typescript
// For small groups (≤10 nodes), connect all nodes
if (nodes.length <= 10) {
  for (let i = 0; i < nodes.length - 1; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      // Create link
    }
  }
} else {
  // For large groups, only connect adjacent nodes in a chain
  for (let i = 0; i < nodes.length - 1; i++) {
    // Create single link to next node
  }
}
```

**Benefits:**
- Reduces link count from O(n²) to O(n) for large files
- Maintains visual grouping without performance penalty
- Example: 100 functions now create 99 links instead of 4,950

### 3. **Boundary Constraints**

Added viewport constraints to keep nodes visible:

```typescript
simulation.on('tick', () => {
  const padding = 50;
  nodes.forEach(d => {
    d.x = Math.max(padding, Math.min(width - padding, d.x!));
    d.y = Math.max(padding, Math.min(height - padding, d.y!));
  });
  // ... update positions
});
```

**Benefits:**
- Nodes stay within viewport bounds
- No more off-screen nodes
- Better user experience

### 4. **Centering Forces**

Added weak centering forces to prevent drift:

```typescript
.force('x', d3.forceX(width / 2).strength(0.05))
.force('y', d3.forceY(height / 2).strength(0.05))
```

**Benefits:**
- Gentle pull toward center prevents excessive spreading
- Doesn't interfere with natural clustering
- Keeps graph compact and viewable

### 5. **Faster Alpha Decay**

Increased alpha decay for large graphs:

```typescript
.alphaDecay(isLargeGraph ? 0.05 : 0.0228)
.velocityDecay(0.4)
```

**Benefits:**
- Large graphs settle faster
- Reduces CPU usage
- Simulation stops when stable

### 6. **Distance Limiting**

Limited the maximum distance for charge forces:

```typescript
.force('charge', d3.forceManyBody()
  .strength(chargeStrength)
  .distanceMax(isLargeGraph ? 200 : 500))
```

**Benefits:**
- Prevents long-range repulsion in large graphs
- Improves performance by reducing force calculations
- Nodes only repel nearby nodes

### 7. **Stronger Collision Detection**

Increased collision force strength:

```typescript
.force('collision', d3.forceCollide()
  .radius(collisionRadius)
  .strength(0.7))
```

**Benefits:**
- Prevents node overlap
- Creates clearer visual separation
- More stable final layout

## Performance Metrics

### Before Optimizations
- **50 nodes**: Slow but usable
- **100 nodes**: Very slow, nodes spread far apart
- **200+ nodes**: Unusable, browser lag, nodes off-screen

### After Optimizations
- **50 nodes**: Fast and responsive
- **100 nodes**: Good performance, compact layout
- **200+ nodes**: Usable, settles quickly, stays on-screen

### Link Count Reduction Example
For a file with 100 functions:
- **Before**: 4,950 links (100 × 99 / 2)
- **After**: 99 links (linear chain)
- **Reduction**: 98% fewer links

## Files Modified

1. **nextjs-frontend/src/components/graph/FunctionGraphViewer.tsx**
   - Added adaptive force parameters
   - Optimized link creation algorithm
   - Added boundary constraints
   - Implemented centering forces

2. **nextjs-frontend/src/components/graph/FunctionGraph.tsx**
   - Added adaptive force parameters
   - Added boundary constraints
   - Implemented centering forces
   - Faster alpha decay for large graphs

3. **static/app.js**
   - Applied same optimizations to legacy graph implementation
   - Ensures consistent performance across all graph views

## Configuration

The threshold for "large graph" detection is currently set at 50 nodes. This can be adjusted:

```typescript
const isLargeGraph = nodeCount > 50; // Adjust this threshold as needed
```

## Future Improvements

1. **Progressive Rendering**: Render only visible nodes for very large graphs (1000+ nodes)
2. **Level of Detail**: Show simplified nodes when zoomed out
3. **Clustering**: Automatically group related nodes into clusters
4. **WebGL Rendering**: Use WebGL for graphs with 500+ nodes
5. **Virtual Scrolling**: Implement viewport culling for massive graphs

## Testing Recommendations

Test the graph with various dataset sizes:
- Small (10-20 nodes): Verify visual quality
- Medium (50-100 nodes): Check performance and layout
- Large (200-500 nodes): Ensure usability and responsiveness
- Very Large (1000+ nodes): Identify breaking points

## Monitoring

Watch for these indicators of performance issues:
- Simulation taking >5 seconds to settle
- Nodes appearing off-screen
- Browser lag during interaction
- High CPU usage after graph settles

## Conclusion

These optimizations significantly improve the performance and usability of the D3 graph visualizations for large changesets. The graph now:
- Settles faster
- Stays within viewport bounds
- Uses less CPU
- Handles larger datasets
- Provides better user experience

