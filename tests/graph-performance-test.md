# Graph Performance Test Plan

## Overview

This document outlines test cases to verify the graph performance optimizations are working correctly.

## Test Environment Setup

### Prerequisites
1. Browser with developer tools (Chrome/Firefox recommended)
2. Test datasets of various sizes
3. Performance monitoring enabled

### Test Data Sizes
- **Small**: 10-20 nodes
- **Medium**: 50-100 nodes
- **Large**: 200-300 nodes
- **Very Large**: 500+ nodes

## Test Cases

### Test 1: Small Graph Performance (10-20 nodes)

**Objective:** Verify small graphs still work well with optimizations

**Steps:**
1. Load a comparison with 10-20 function changes
2. Open the graph view
3. Observe initial render

**Expected Results:**
- ✓ Graph renders in <1 second
- ✓ All nodes visible on screen
- ✓ Smooth animation
- ✓ No overlap between nodes
- ✓ Links clearly visible

**Metrics:**
- Render time: <1s
- Settle time: <2s
- CPU usage: <30%
- All nodes on-screen: Yes

---

### Test 2: Medium Graph Performance (50-100 nodes)

**Objective:** Verify optimizations kick in for medium graphs

**Steps:**
1. Load a comparison with 50-100 function changes
2. Open the graph view
3. Monitor performance

**Expected Results:**
- ✓ Graph renders in <3 seconds
- ✓ All nodes stay within viewport
- ✓ Compact layout (not spread out)
- ✓ Simulation settles quickly
- ✓ Low CPU usage after settling

**Metrics:**
- Render time: <3s
- Settle time: <5s
- CPU usage: <40% (drops to <10% after settling)
- All nodes on-screen: Yes
- Link count: ~O(n) not O(n²)

**Verification:**
```javascript
// Open browser console
console.log('Nodes:', nodes.length);
console.log('Links:', links.length);
console.log('Links per node:', (links.length / nodes.length).toFixed(2));
// Should be ~1-2 links per node, not ~50
```

---

### Test 3: Large Graph Performance (200-300 nodes)

**Objective:** Verify large graphs are usable

**Steps:**
1. Load a comparison with 200-300 function changes
2. Open the graph view
3. Monitor CPU and memory usage
4. Test interaction (zoom, pan, drag)

**Expected Results:**
- ✓ Graph renders in <5 seconds
- ✓ Nodes stay within viewport bounds
- ✓ Simulation settles within 10 seconds
- ✓ Smooth zoom and pan
- ✓ CPU usage drops after settling

**Metrics:**
- Render time: <5s
- Settle time: <10s
- CPU usage: <50% (drops to <15% after settling)
- Memory usage: <200MB
- All nodes on-screen: Yes

---

### Test 4: Boundary Constraints

**Objective:** Verify nodes don't drift off-screen

**Steps:**
1. Load any graph with 50+ nodes
2. Wait for simulation to settle
3. Check node positions

**Expected Results:**
- ✓ All nodes within viewport
- ✓ No nodes at x < 50 or x > (width - 50)
- ✓ No nodes at y < 50 or y > (height - 50)
- ✓ Graph centered in viewport

**Verification:**
```javascript
// Check in browser console
const outOfBounds = nodes.filter(n => 
  n.x < 50 || n.x > 750 || n.y < 50 || n.y > 550
);
console.log('Out of bounds nodes:', outOfBounds.length);
// Should be 0
```

---

### Test 5: Link Creation Optimization

**Objective:** Verify O(n) link creation for large file groups

**Steps:**
1. Load a comparison with a file containing 50+ functions
2. Open browser console
3. Check link count

**Expected Results:**
- ✓ Link count is ~O(n) not O(n²)
- ✓ For 100 functions in one file: ~99 links, not ~4,950

**Verification:**
```javascript
// For a file with 100 functions
const expectedLinks = 99; // Chain
const actualLinks = links.filter(l => /* same file */).length;
console.log('Expected:', expectedLinks, 'Actual:', actualLinks);
// Should be close to 99, not 4,950
```

---

### Test 6: CPU Usage After Settling

**Objective:** Verify simulation stops consuming CPU

**Steps:**
1. Load any graph
2. Wait for simulation to settle (alpha < 0.01)
3. Monitor CPU usage for 30 seconds

**Expected Results:**
- ✓ CPU usage drops to <5% after settling
- ✓ No continuous animation
- ✓ Browser remains responsive

**Metrics:**
- CPU after settling: <5%
- Animation stopped: Yes
- Browser responsive: Yes

---

### Test 7: Interaction Performance

**Objective:** Verify smooth interaction with large graphs

**Steps:**
1. Load a graph with 100+ nodes
2. Test zoom in/out
3. Test pan
4. Test node drag
5. Test node click

**Expected Results:**
- ✓ Zoom is smooth (60fps)
- ✓ Pan is smooth (60fps)
- ✓ Drag updates in real-time
- ✓ Click registers immediately
- ✓ No lag or stuttering

---

### Test 8: Memory Leak Check

**Objective:** Verify no memory leaks when switching views

**Steps:**
1. Load a large graph
2. Switch to another view
3. Switch back to graph view
4. Repeat 10 times
5. Check memory usage

**Expected Results:**
- ✓ Memory usage stable
- ✓ No continuous growth
- ✓ Old simulations cleaned up

**Verification:**
```javascript
// In browser console, check memory over time
// Should not continuously increase
```

---

### Test 9: Visual Quality

**Objective:** Verify graph is visually clear and useful

**Steps:**
1. Load a medium graph (50-100 nodes)
2. Examine visual layout

**Expected Results:**
- ✓ Nodes clearly separated
- ✓ Labels readable
- ✓ Links visible but not overwhelming
- ✓ Color coding clear
- ✓ Related nodes grouped together

---

### Test 10: Comparison with Old Implementation

**Objective:** Verify improvements over old implementation

**Steps:**
1. Test same dataset with old code (if available)
2. Test same dataset with new code
3. Compare metrics

**Expected Results:**
- ✓ New code 50%+ faster
- ✓ New code uses less CPU
- ✓ New code keeps nodes on-screen
- ✓ New code settles faster

**Metrics to Compare:**
- Render time
- Settle time
- CPU usage
- Link count
- Nodes on-screen

---

## Performance Benchmarks

### Target Metrics

| Graph Size | Render Time | Settle Time | CPU (Peak) | CPU (Settled) | Links/Node |
|------------|-------------|-------------|------------|---------------|------------|
| 10 nodes   | <1s         | <2s         | <30%       | <5%           | ~1-2       |
| 50 nodes   | <2s         | <3s         | <40%       | <10%          | ~1-2       |
| 100 nodes  | <3s         | <5s         | <50%       | <15%          | ~1-2       |
| 200 nodes  | <5s         | <8s         | <60%       | <20%          | ~1-2       |
| 500 nodes  | <10s        | <15s        | <70%       | <25%          | ~1-2       |

### Regression Thresholds

If any of these are exceeded, investigate:
- Render time >2x target
- Settle time >2x target
- CPU usage >1.5x target
- Links/node >5 (indicates O(n²) problem)
- Any nodes off-screen

## Automated Testing

### Performance Test Script

```javascript
// Run in browser console
async function testGraphPerformance() {
  const startTime = performance.now();
  
  // Wait for graph to render
  await new Promise(resolve => setTimeout(resolve, 100));
  const renderTime = performance.now() - startTime;
  
  // Wait for simulation to settle
  const settleStart = performance.now();
  await new Promise(resolve => {
    const checkAlpha = setInterval(() => {
      if (simulation.alpha() < 0.01) {
        clearInterval(checkAlpha);
        resolve();
      }
    }, 100);
  });
  const settleTime = performance.now() - settleStart;
  
  // Check node positions
  const outOfBounds = nodes.filter(n => 
    n.x < 50 || n.x > 750 || n.y < 50 || n.y > 550
  ).length;
  
  // Calculate links per node
  const linksPerNode = links.length / nodes.length;
  
  // Report results
  console.log('Performance Test Results:');
  console.log('- Nodes:', nodes.length);
  console.log('- Links:', links.length);
  console.log('- Render time:', renderTime.toFixed(0), 'ms');
  console.log('- Settle time:', settleTime.toFixed(0), 'ms');
  console.log('- Links per node:', linksPerNode.toFixed(2));
  console.log('- Out of bounds:', outOfBounds);
  
  // Pass/fail
  const passed = 
    renderTime < 5000 &&
    settleTime < 10000 &&
    linksPerNode < 5 &&
    outOfBounds === 0;
  
  console.log('Test:', passed ? 'PASSED ✓' : 'FAILED ✗');
  
  return { renderTime, settleTime, linksPerNode, outOfBounds, passed };
}

// Run test
testGraphPerformance();
```

## Reporting Issues

If tests fail, report:
1. Graph size (node count)
2. Browser and version
3. Hardware specs
4. Actual vs expected metrics
5. Console errors
6. Screenshots if visual issues

## Success Criteria

All tests must pass with:
- ✓ Performance within target metrics
- ✓ All nodes visible on-screen
- ✓ Smooth interaction
- ✓ Low CPU usage after settling
- ✓ No memory leaks
- ✓ Visual quality maintained

