//! Simple f32-based quadtree that can query rects and points
//! in Olog(n) time.
//!
//! Note: For simplicity sake, there is no way to update the tree
//! besides destroying and rebuilding it completely.

use std::fmt;
use std::collections::BTreeMap;
use std::iter::Iterator;

/// f32-based Point
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point { pub x: f32, pub y: f32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Point {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Item that can be inserted into the QuadTree
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Item {
    Rect(Rect),
    Point(Point),
}

impl Item {
    /// Returns the bounding box of the item (in the case of a point, min_x == max_x)
    pub fn get_bbox(&self) -> Rect {
        match self {
            Item::Rect(r) => *r,
            Item::Point(p) => Rect {
                min_x: p.x,
                max_x: p.x,
                min_y: p.y,
                max_y: p.y,
            }
        }
    }
}

/// Rectangle (2d bounding box) that can be inserted into the QuadTree
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x: {} - {}, y: {} - {}]", self.max_x, self.min_x, self.max_y, self.min_y)
    }
}

impl Rect {

    /// Returns 4 rects, each representing one quadrant
    fn quarter(&self) -> [Rect;4] {
        let middle_x = (self.max_x + self.min_x) / 2.0;
        let middle_y = (self.max_y + self.min_y) / 2.0;
        [
            // top left
            Rect {
                max_x: self.max_x,
                min_x: middle_x,
                max_y: self.max_y,
                min_y: middle_y,
            },
            // top right
            Rect {
                max_x: middle_x,
                min_x: self.min_x,
                max_y: self.max_y,
                min_y: middle_y,
            },
            // bottom left
            Rect {
                max_x: self.max_x,
                min_x: middle_x,
                max_y: middle_y,
                min_y: self.min_y,
            },
            // bottom right
            Rect {
                max_x: middle_x,
                min_x: self.min_x,
                max_y: middle_y,
                min_y: self.min_y,
            },
        ]
    }

    /// Returns a rect with all points set to 0.0
    pub const fn zero() -> Self {
        Self {
            max_x: 0.0,
            max_y: 0.0,
            min_x: 0.0,
            min_y: 0.0,
        }
    }

    /// Returns the height of the rectangle
    #[inline]
    pub fn get_width(&self) -> f32 {
        self.max_x - self.min_x
    }

    /// Returns the height of the rectangle
    #[inline]
    pub fn get_height(&self) -> f32 {
        self.max_y - self.min_y
    }

    /// Returns whether the rect contains a point
    #[inline]
    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x &&
        p.y >= self.min_y && p.y <= self.max_y
    }

    /// Returns whether the rect *completely contains* another rect
    pub fn contains_rect(&self, r: &Rect) -> bool {
        r.min_x >= self.min_x && r.max_x <= self.max_x &&
        r.min_y >= self.min_y && r.max_x <= self.max_y
    }

    /// Returns whether the rect *overlaps* another rect
    pub fn overlaps_rect(&self, r: &Rect) -> bool {

        // test if self overlaps r
        let min_y_in_range = r.min_y >= self.min_y && r.min_y <= self.max_y;
        let max_y_in_range = r.max_y >= self.min_y && r.max_y <= self.max_y;
        let min_x_in_range = r.min_x >= self.min_x && r.min_x <= self.max_x;
        let max_x_in_range = r.max_x >= self.min_x && r.max_x <= self.max_x;

        // test if r overlaps self
        let self_min_y_in_range = self.min_y >= r.min_y && self.min_y <= r.max_y;
        let self_max_y_in_range = self.max_y >= r.min_y && self.max_y <= r.max_y;
        let self_min_x_in_range = self.min_x >= r.min_x && self.min_x <= r.max_x;
        let self_max_x_in_range = self.max_x >= r.min_x && self.max_x <= r.max_x;

        min_y_in_range || max_y_in_range || min_x_in_range || max_x_in_range ||
        self_min_y_in_range || self_max_y_in_range || self_min_x_in_range || self_max_x_in_range
    }

    /// Unions two rectangles together
    pub fn union(&self, r: &Rect) -> Rect {
        Rect {
            max_x: r.max_x.max(self.max_x),
            max_y: r.max_y.max(self.max_y),
            min_x: r.min_x.min(self.min_x),
            min_y: r.min_y.min(self.min_y),
        }
    }
}

/// Instead of making a generic tree, the quadtree only
/// keeps items (rects or points) and their associated IDs.
/// After insertion, you can query the IDs of items within
/// a certain region.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(pub usize);

// ID for indexing into the quadtree.index
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InternalId(usize);

/// A QuadTree that can store rectangles and points
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QuadTree {
    all_items: BTreeMap<ItemId, Item>,
    bbox: Rect,
    knots: Vec<Knot>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Knot {
    HasItems { bbox: Rect, items: Vec<(Rect, ItemId)> },
    HasChildren([(InternalId, Rect);4]),
}

fn construct_quadtree(
    items: Vec<(Rect, ItemId)>,
    bbox: Rect,
    max_len: usize
) -> Vec<Knot> {

    let mut items = vec![Knot::HasItems { bbox, items }];

    loop {

        let mut items_to_push = Vec::new();
        let current_items_len = items.len();

        for knot in items.iter_mut() {
            match knot.clone() {
                Knot::HasItems { bbox, items } if items.len() > max_len => {
                    // rect has too many items split it into 4 quarters and replace the original knot with
                    let [top_left, top_right, bottom_left, bottom_right] = bbox.quarter();

                    let top_left_rects = items.iter().filter(|(r, _)| top_left.overlaps_rect(r)).copied().collect();
                    let top_right_rects = items.iter().filter(|(r, _)| top_right.overlaps_rect(r)).copied().collect();
                    let bottom_left_rects = items.iter().filter(|(r, _)| bottom_left.overlaps_rect(r)).copied().collect();
                    let bottom_right_rects = items.iter().filter(|(r, _)| bottom_right.overlaps_rect(r)).copied().collect();

                    let top_left_knot = Knot::HasItems { bbox: top_left, items: top_left_rects };
                    let top_left_knot_id = current_items_len + items_to_push.len();
                    items_to_push.push(top_left_knot);

                    let top_right_knot = Knot::HasItems { bbox: top_right, items: top_right_rects };
                    let top_right_knot_id = current_items_len + items_to_push.len();
                    items_to_push.push(top_right_knot);

                    let bottom_left_knot = Knot::HasItems { bbox: bottom_left, items: bottom_left_rects };
                    let bottom_left_knot_id = current_items_len + items_to_push.len();
                    items_to_push.push(bottom_left_knot);

                    let bottom_right_knot = Knot::HasItems { bbox: bottom_right, items: bottom_right_rects };
                    let bottom_right_knot_id = current_items_len + items_to_push.len();
                    items_to_push.push(bottom_right_knot);

                    *knot = Knot::HasChildren([
                        (InternalId(top_left_knot_id), top_left),
                        (InternalId(top_right_knot_id), top_right),
                        (InternalId(bottom_left_knot_id), bottom_left),
                        (InternalId(bottom_right_knot_id), bottom_right),
                    ]);
                },
                _ => { },
            }
        }

        if items_to_push.is_empty() {
            break;
        } else {
            items.append(&mut items_to_push);
        }
    }

    items
}

#[inline]
fn get_ids<F: Fn(&Rect, &Rect) -> bool>(
    tree: &QuadTree,
    query_rect: &Rect,
    query: F,
) -> Vec<ItemId> {
    let mut items_to_search = vec![InternalId(0)];
    let mut result = Vec::new();

    while !items_to_search.is_empty() {

        let mut knots_to_resolve = Vec::new();

        for id in items_to_search {
            match &tree.knots[id.0] {
                Knot::HasItems { bbox, items } => {
                    if query(bbox, query_rect) {
                        for (item_rect, item_id) in items.iter() {
                            if query(item_rect, query_rect) { result.push(*item_id); }
                        }
                    }
                },
                Knot::HasChildren([(tl_id, tl_rect), (tr_id, tr_rect), (bl_id, bl_rect), (br_id, br_rect)]) => {
                    if query(tl_rect, query_rect) { knots_to_resolve.push(*tl_id); }
                    if query(tr_rect, query_rect) { knots_to_resolve.push(*tr_id); }
                    if query(bl_rect, query_rect) { knots_to_resolve.push(*bl_id); }
                    if query(br_rect, query_rect) { knots_to_resolve.push(*br_id); }
                },
            }
        }

        items_to_search = knots_to_resolve;
    }

    result.sort();
    result.dedup();

    result
}

impl QuadTree {

    /// Constructs a new QuadTree with at most 10 items per box
    pub fn new<I: Iterator<Item=(ItemId, Item)>>(items: I) -> Self {
        Self::new_with_max_items_per_quad(items, 10)
    }

    /// For performance, how many items should be in each quad before it gets subdivided?
    pub fn new_with_max_items_per_quad<I: Iterator<Item=(ItemId, Item)>>(items: I, max_items: usize) -> Self {
        let items_with_id_and_bbox = items.into_iter().map(|(id, i)| (i, i.get_bbox(), id)).collect::<Vec<_>>();
        let all_items = items_with_id_and_bbox.iter().map(|(item, _bbox, id)| (*id, *item)).collect::<BTreeMap<_, _>>();
        let all_bboxes = items_with_id_and_bbox.into_iter().map(|(_item, bbox, id)| (bbox, id)).collect::<Vec<_>>();
        let sum_of_bboxes = all_bboxes.iter().fold(Rect::zero(), |f, a| f.union(&a.0));
        Self {
            all_items,
            bbox: sum_of_bboxes,
            knots: construct_quadtree(all_bboxes, sum_of_bboxes, max_items),
        }
    }

    /// Returns the extent of all items
    pub fn bbox(&self) -> Rect {
        self.bbox
    }

    /// Query the IDs of all items that *overlap* the rect
    pub fn get_ids_that_overlap(&self, rect: &Rect) -> Vec<ItemId> {
        get_ids(self, rect, |a, b| a.overlaps_rect(b))
    }

    /// Query the IDs of all items *completely contained* by the rect
    pub fn get_ids_contained_by(&self, rect: &Rect) -> Vec<ItemId> {
        get_ids(self, rect, |a, b| a.contains_rect(b))
    }

    /// Returns all points in the QuadTree that are within the bounds of `rect`
    pub fn get_points_contained_by(&self, rect: &Rect) -> Vec<Point> {
        self.get_ids_contained_by(rect)
        .into_iter()
        .filter_map(|id| self.all_items.get(&id))
        .filter_map(|item| match item { Item::Point(p) => Some(*p), Item::Rect(_) => None })
        .collect()
    }

    /// Returns all rectangles in the QuadTree that *overlap* `rect`
    pub fn get_rects_that_overlap(&self, rect: &Rect) -> Vec<Rect> {
        self.get_ids_that_overlap(rect)
        .into_iter()
        .filter_map(|id| self.all_items.get(&id))
        .filter_map(|item| match item { Item::Point(_) => None, Item::Rect(r) => Some(*r) })
        .collect()
    }

    /// Returns all rectangles in the QuadTree that *are completely contained by* `rect`
    pub fn get_rects_contained_by(&self, rect: &Rect) -> Vec<Rect> {
        self.get_ids_contained_by(rect)
        .into_iter()
        .filter_map(|id| self.all_items.get(&id))
        .filter_map(|item| match item { Item::Point(_) => None, Item::Rect(r) => Some(*r) })
        .collect()
    }

    pub fn get_all_ids(&self) -> Vec<ItemId> {
        self.all_items.keys().copied().collect()
    }

    pub fn get_all_items(&self) -> Vec<Item> {
        self.all_items.values().copied().collect()
    }

    pub fn get_item(&self, id: &ItemId) -> Option<&Item> {
        self.all_items.get(id)
    }
}
