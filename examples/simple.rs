extern crate quadtree_f32;

use quadtree_f32::{Item, ItemId, Rect, QuadTree};
use std::time::Instant;

fn main() {
    // generate 1000 rects from
    let items = (0..100).flat_map(|x| {
        (0..100).map(move |y| {
            let id = ItemId(x * 100 + y);
            let rect = Item::Rect(Rect {
                max_y: y as f32 + 1.0,
                min_y: y as f32,
                max_x: x as f32 + 1.0,
                min_x: x as f32,
            });
            (id, rect)
        })
    }).collect::<Vec<_>>();

    let now = Instant::now();
    let quadtree = QuadTree::new(items.into_iter());
    println!("constructed quadtree in {:?}", now.elapsed());
    let now = Instant::now();
    let result = quadtree.get_ids_that_overlap(&Rect {
        max_x: 22.5,
        min_x: 21.5,
        max_y: 50.7,
        min_y: 49.7,
    });
    println!("query took in {:?}", now.elapsed());
    println!("result: {:?}", result);
}