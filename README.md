# QuadTree-f32

A fast, dependency-free 2D spatial indexing QuadTree implementation in Rust for efficient storage and querying of points and rectangles.

[![Crates.io](https://img.shields.io/crates/v/quadtree-f32.svg)](https://crates.io/crates/quadtree-f32)
[![Documentation](https://docs.rs/quadtree-f32/badge.svg)](https://docs.rs/quadtree-f32)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Dynamic Operations**: Insert and remove items on-the-fly with automatic tree restructuring
- **Dual Geometry Support**: Store both points and axis-aligned rectangles
- **ID-Based Management**: Associate items with unique identifiers for efficient retrieval
- **Spatial Queries**: Find overlapping items, points within regions, and spatial relationships
- **DBSCAN Clustering**: Built-in density-based clustering algorithm
- **Float Precision Options**: Use `f32` (default) or `f64` with the `f64` feature
- **Zero Dependencies**: Pure Rust implementation with no external crates

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
quadtree-f32 = "0.4.1"

# For f64 precision
quadtree-f32 = { version = "0.4.1", features = ["f64"] }
```

## Quick Start

```rust
use quadtree_f32::{QuadTree, Rect, Point, Item, ItemId};

// Create a new QuadTree.
let mut quadtree = QuadTree::new();
// The QuadTree will automatically determine its bounds from the items inserted.
// The first item will set the initial bounding box, which will expand as needed
// when new items are added outside the current bounds.

// Insert a point
let point_id = ItemId(1);
let point = Item::Point(Point::new(25.0, 25.0));
quadtree.insert(point_id, point);

// Insert a rectangle
let rect_id = ItemId(2);
let rect = Item::Rect(Rect { min_x: 10.0, min_y: 10.0, max_x: 20.0, max_y: 20.0 });
quadtree.insert(rect_id, rect);

// Query for overlapping items
let query_area = Rect { min_x: 15.0, min_y: 15.0, max_x: 30.0, max_y: 30.0 };
let overlapping_ids = quadtree.get_ids_that_overlap(&query_area);
println!("Found {} overlapping items", overlapping_ids.len());
```

## Core API

### Construction
```rust
let quadtree = QuadTree::new();
```
The bounding box of the QuadTree is managed automatically. It's initialized by the first item inserted and expands as necessary.

### Insertion & Removal
```rust
quadtree.insert(item_id, item);
let removed = quadtree.remove(item_id, item);
```

### Spatial Queries
```rust
// Find all items overlapping a region
let ids = quadtree.get_ids_that_overlap(&query_rect);

// Find points contained within a region
let points = quadtree.get_points_contained_by(&query_rect);

// Find rectangles overlapping a region
let rects = quadtree.get_rects_that_overlap(&query_rect);
```

### Item Retrieval
```rust
// Get specific item by ID
let item = quadtree.get_item_by_id(item_id);

// Get all items or IDs
let all_items = quadtree.get_all_items();
let all_ids = quadtree.get_all_ids();
```

## DBSCAN Clustering

Perform density-based clustering on stored items:

```rust
use quadtree_f32::{QuadTree, Item, ItemId, Point}; // Added Point to use

let mut quadtree = QuadTree::new(); // Add this line

// Add several nearby points
for i in 0..10 {
    let id = ItemId(i);
    // Ensure Point::new is used correctly if it was missing
    let point = Item::Point(Point::new(10.0 + i as f32 * 0.5, 10.0));
    quadtree.insert(id, point);
}

// Cluster with epsilon=2.0, minimum 3 items per cluster
let clusters = quadtree.get_clusters(2.0, 3);

for (i, cluster) in clusters.iter().enumerate() {
    println!("Cluster {}: {:?}", i, cluster);
}
```

## Performance Characteristics

- **Insertion**: O(log n) average case
- **Removal**: O(log n) average case  
- **Spatial Queries**: O(log n + k) where k is the number of results
- **Tree Rebalancing**: Automatic merging when nodes become sparse

The quadtree automatically subdivides when leaf nodes exceed the maximum capacity (4 items) and merges nodes when they become sufficiently sparse after removals.

## Coordinate System

The quadtree uses a standard 2D coordinate system where:
- `min_x`, `min_y` represent the bottom-left corner
- `max_x`, `max_y` represent the top-right corner
- Points have zero-area bounding boxes where `min == max`

## Memory Management

The implementation uses `Box<QuadtreeNode>` for tree nodes and `Vec` for item storage, providing efficient memory usage that grows and shrinks with the data.

## Thread Safety

This implementation is not thread-safe. For concurrent access, wrap in appropriate synchronization primitives:

```rust
use std::sync::{Arc, Mutex};
let shared_tree = Arc::new(Mutex::new(QuadTree::new())); // Example for a new tree
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please ensure tests pass and follow the existing code style.
