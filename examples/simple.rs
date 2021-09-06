extern crate quadtree_f32;

use quadtree_f32::{Item, ItemId, Rect, QuadTree};
use std::time::Instant;

fn main() {
    // generate 1000 rects from
    /*
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
    */

    let items = vec![
        (ItemId(0), Item::Rect(Rect {
            max_x: 196.27501,
            max_y: 137.79999,
            min_x: 195.27501,
            min_y: 136.79999
        }))
    ];

    /*
11:

162: Rect(Rect {
    max_x: 195.15001,
    max_y: 137.0,
    min_x: 194.15001,
    min_y: 136.0
})
    */
    let now = Instant::now();
    let quadtree = QuadTree::new(items.into_iter());
    println!("constructed quadtree in {:?}", now.elapsed());
    let now = Instant::now();
    let result = quadtree.get_ids_that_overlap(&Rect {
        max_x: 195.15001 + 1.0,
        max_y: 137.0 + 1.0,
        min_x: 194.15001 - 1.0,
        min_y: 136.0 - 1.0
    });
    println!("query took in {:?}", now.elapsed());
    println!("result: {:?}", result);
}