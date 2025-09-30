# Graph Visualization Tuning Guide

## Quick Reference

This guide helps you tune the D3 force-directed graph parameters for optimal performance and visual quality.

## Key Parameters

### 1. Large Graph Threshold

**Location:** All graph components  
**Default:** `50 nodes`

```typescript
const isLargeGraph = nodeCount > 50;
```

**When to adjust:**
- Increase (e.g., 100) if you have powerful hardware
- Decrease (e.g., 30) for mobile or low-end devices
- Monitor CPU usage and render time to find optimal value

### 2. Charge Strength (Node Repulsion)

**Purpose:** Controls how strongly nodes push away from each other

```typescript
const chargeStrength = isLargeGraph ? -100 : -300;
```

**Effects:**
- **More negative** (-500): Nodes spread far apart
- **Less negative** (-50): Nodes cluster tightly
- **Default small**: -300 (good spacing)
- **Default large**: -100 (compact layout)

**Adjust if:**
- Nodes too spread out → Increase (less negative)
- Nodes overlapping → Decrease (more negative)

### 3. Link Distance

**Purpose:** Preferred distance between connected nodes

```typescript
const linkDistance = isLargeGraph ? 50 : 100;
```

**Effects:**
- **Larger** (150): Spread out graph
- **Smaller** (30): Compact graph
- **Default small**: 100px
- **Default large**: 50px

**Adjust if:**
- Graph too spread → Decrease
- Graph too cramped → Increase

### 4. Collision Radius

**Purpose:** Minimum distance between node centers

```typescript
const collisionRadius = isLargeGraph ? 20 : 30;
```

**Effects:**
- **Larger** (40): More spacing, no overlap
- **Smaller** (10): Tighter packing, possible overlap
- **Default small**: 30px
- **Default large**: 20px

**Adjust if:**
- Nodes overlapping → Increase
- Too much empty space → Decrease

### 5. Alpha Decay

**Purpose:** How quickly the simulation settles

```typescript
.alphaDecay(isLargeGraph ? 0.05 : 0.0228)
```

**Effects:**
- **Higher** (0.1): Settles faster, may not reach optimal layout
- **Lower** (0.01): Settles slower, better layout quality
- **Default small**: 0.0228 (D3 default)
- **Default large**: 0.05 (faster settling)

**Adjust if:**
- Simulation too slow → Increase
- Layout not optimal → Decrease

### 6. Velocity Decay

**Purpose:** Friction/damping of node movement

```typescript
.velocityDecay(0.4)
```

**Effects:**
- **Higher** (0.6): More friction, slower movement
- **Lower** (0.2): Less friction, faster movement
- **Default**: 0.4 (balanced)

**Adjust if:**
- Nodes moving too fast → Increase
- Simulation too slow → Decrease

### 7. Distance Max (Charge)

**Purpose:** Maximum distance for charge force calculations

```typescript
.distanceMax(isLargeGraph ? 200 : 500)
```

**Effects:**
- **Larger** (1000): Long-range repulsion, slower
- **Smaller** (100): Short-range repulsion, faster
- **Default small**: 500px
- **Default large**: 200px

**Adjust if:**
- Performance issues → Decrease
- Nodes clustering too much → Increase

### 8. Centering Force Strength

**Purpose:** Pull toward center of viewport

```typescript
.force('x', d3.forceX(width / 2).strength(0.05))
.force('y', d3.forceY(height / 2).strength(0.05))
```

**Effects:**
- **Higher** (0.2): Strong pull to center
- **Lower** (0.01): Weak pull to center
- **Default**: 0.05 (gentle centering)

**Adjust if:**
- Graph drifting off-center → Increase
- Too centered, not spreading → Decrease

### 9. Collision Strength

**Purpose:** How strongly collision detection prevents overlap

```typescript
.force('collision', d3.forceCollide()
  .radius(collisionRadius)
  .strength(0.7))
```

**Effects:**
- **Higher** (0.9): Strong overlap prevention
- **Lower** (0.3): Weak overlap prevention
- **Default**: 0.7 (good balance)

**Adjust if:**
- Nodes overlapping → Increase
- Nodes too spread → Decrease

### 10. Link Strength

**Purpose:** How strongly links pull connected nodes together

```typescript
.strength(d => d.strength * (isLargeGraph ? 0.3 : 1))
```

**Effects:**
- **Higher** (1.0): Strong pull, tight clusters
- **Lower** (0.1): Weak pull, loose clusters
- **Default small**: 1.0 (full strength)
- **Default large**: 0.3 (weaker for performance)

**Adjust if:**
- Connected nodes too far → Increase
- Graph too clustered → Decrease

## Common Scenarios

### Scenario 1: Graph Too Spread Out

**Symptoms:**
- Nodes far apart
- Lots of empty space
- Hard to see relationships

**Solutions:**
```typescript
// Increase charge strength (less negative)
const chargeStrength = isLargeGraph ? -50 : -200;

// Decrease link distance
const linkDistance = isLargeGraph ? 30 : 70;

// Increase centering force
.force('x', d3.forceX(width / 2).strength(0.1))
.force('y', d3.forceY(height / 2).strength(0.1))
```

### Scenario 2: Nodes Overlapping

**Symptoms:**
- Nodes on top of each other
- Labels unreadable
- Hard to click individual nodes

**Solutions:**
```typescript
// Decrease charge strength (more negative)
const chargeStrength = isLargeGraph ? -150 : -400;

// Increase collision radius
const collisionRadius = isLargeGraph ? 30 : 40;

// Increase collision strength
.force('collision', d3.forceCollide()
  .radius(collisionRadius)
  .strength(0.9))
```

### Scenario 3: Simulation Too Slow

**Symptoms:**
- Takes >10 seconds to settle
- High CPU usage
- Browser lag

**Solutions:**
```typescript
// Increase alpha decay
.alphaDecay(isLargeGraph ? 0.08 : 0.04)

// Decrease distance max
.distanceMax(isLargeGraph ? 150 : 300)

// Increase velocity decay
.velocityDecay(0.5)

// Lower large graph threshold
const isLargeGraph = nodeCount > 30;
```

### Scenario 4: Nodes Drifting Off-Screen

**Symptoms:**
- Nodes outside viewport
- Have to zoom out to see all nodes
- Graph keeps expanding

**Solutions:**
```typescript
// Increase centering force
.force('x', d3.forceX(width / 2).strength(0.1))
.force('y', d3.forceY(height / 2).strength(0.1))

// Ensure boundary constraints are active
const padding = 50;
nodes.forEach(d => {
  d.x = Math.max(padding, Math.min(width - padding, d.x!));
  d.y = Math.max(padding, Math.min(height - padding, d.y!));
});

// Decrease charge strength
const chargeStrength = isLargeGraph ? -80 : -250;
```

## Performance Tuning

### For Maximum Performance (500+ nodes)

```typescript
const isLargeGraph = nodeCount > 30; // Lower threshold
const chargeStrength = -50; // Weak repulsion
const linkDistance = 30; // Short links
const collisionRadius = 15; // Tight packing

simulation
  .alphaDecay(0.1) // Fast settling
  .velocityDecay(0.5) // High friction
  .force('charge', d3.forceManyBody()
    .strength(chargeStrength)
    .distanceMax(100)) // Very short range
```

### For Maximum Quality (10-30 nodes)

```typescript
const chargeStrength = -400; // Strong repulsion
const linkDistance = 120; // Long links
const collisionRadius = 35; // Generous spacing

simulation
  .alphaDecay(0.015) // Slow settling
  .velocityDecay(0.3) // Low friction
  .force('charge', d3.forceManyBody()
    .strength(chargeStrength)
    .distanceMax(800)) // Long range
```

## Monitoring and Debugging

### Check Simulation Status

```typescript
simulation.on('tick', () => {
  console.log('Alpha:', simulation.alpha());
  // Alpha < 0.01 means simulation is settling
});

simulation.on('end', () => {
  console.log('Simulation complete');
});
```

### Measure Performance

```typescript
const startTime = performance.now();
simulation.on('end', () => {
  const duration = performance.now() - startTime;
  console.log(`Settled in ${duration}ms`);
});
```

### Count Links

```typescript
console.log(`Nodes: ${nodes.length}, Links: ${links.length}`);
console.log(`Links per node: ${(links.length / nodes.length).toFixed(2)}`);
```

## Best Practices

1. **Start with defaults** - Only tune if you have specific issues
2. **Change one parameter at a time** - Easier to understand effects
3. **Test with real data** - Use actual changeset sizes
4. **Monitor performance** - Watch CPU usage and settle time
5. **Document changes** - Note why you changed parameters
6. **Consider user hardware** - Mobile devices need more aggressive optimization

## Further Reading

- [D3 Force Documentation](https://github.com/d3/d3-force)
- [Graph Performance Optimizations](./graph-performance-optimizations.md)
- [Optimization Comparison](./graph-optimization-comparison.md)

