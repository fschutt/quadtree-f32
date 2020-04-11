//! Simple f32-based quadtree that can query rects and points
//! in Olog(n) time.
//!
//! Note: For simplicity sake, there is no way to update the tree
//! besides destroying and rebuilding it completely.

use std::fmt;
use std::collections::BTreeMap;

/// f32-based Point
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point { pub x: f32, pub y: f32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
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
        r.contains_point(&Point { x: self.min_x, y: self.min_y }) ||
        r.contains_point(&Point { x: self.max_x, y: self.min_y }) ||
        r.contains_point(&Point { x: self.max_x, y: self.max_y }) ||
        r.contains_point(&Point { x: self.max_x, y: self.min_x })
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
pub struct ItemId(usize);

/// A QuadTree that can store rectangles and points
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QuadTree {
    all_items: BTreeMap<ItemId, Item>,
    internal: QuadTreeInternal,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct QuadTreeInternal {
    bbox: Rect,
    knot: Knot,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Knot {
    HasItems(Vec<(Rect, ItemId)>),
    HasChildren(Box<[QuadTreeInternal;4]>),
}

impl Knot {
    fn query_items_overlap(&self, r: &Rect) -> Vec<ItemId> {
        match self {
            Knot::HasItems(i) => {
                i
                .iter()
                .filter_map(|(k, v)| if k.overlaps_rect(r) { Some(*v) } else { None })
                .collect()
            },
            Knot::HasChildren(children) => {
                children
                    .iter()
                    .filter(|quadrant| r.overlaps_rect(&quadrant.bbox))
                    .flat_map(|q| q.query_items_overlap(&r).into_iter())
                    .collect()
            }
        }
    }

    fn query_items_contained_by(&self, r: &Rect) -> Vec<ItemId> {
        match self {
            Knot::HasItems(i) => {
                i
                .iter()
                .filter_map(|(k, v)| if r.contains_rect(k) { Some(*v) } else { None })
                .collect()
            },
            Knot::HasChildren(children) => {
                children
                    .iter()
                    .filter(|quadrant| r.overlaps_rect(&quadrant.bbox))
                    .flat_map(|q| q.query_items_contained_by(&r).into_iter())
                    .collect()
            }
        }
    }
}

impl QuadTreeInternal {

    fn new(total_bbox: Rect, bboxes: Vec<(Rect, ItemId)>, max_items: usize) -> Self {

        let knot = if bboxes.len() <= max_items {
            Knot::HasItems(bboxes)
        } else {
            let [top_left, top_right, bottom_left, bottom_right] = total_bbox.quarter();

            let top_left_rects = bboxes.iter().filter(|(r, _)| top_left.contains_rect(r)).copied().collect();
            let top_right_rects = bboxes.iter().filter(|(r, _)| top_right.contains_rect(r)).copied().collect();
            let bottom_left_rects = bboxes.iter().filter(|(r, _)| bottom_left.contains_rect(r)).copied().collect();
            let bottom_right_rects = bboxes.iter().filter(|(r, _)| bottom_right.contains_rect(r)).copied().collect();

            Knot::HasChildren(Box::new([
                QuadTreeInternal::new(top_left, top_left_rects, max_items),
                QuadTreeInternal::new(top_right, top_right_rects, max_items),
                QuadTreeInternal::new(bottom_left, bottom_left_rects, max_items),
                QuadTreeInternal::new(bottom_right, bottom_right_rects, max_items),
            ]))
        };

        Self {
            bbox: total_bbox,
            knot
        }
    }

    fn query_items_overlap(&self, r: &Rect) -> Vec<ItemId> {
        if !r.overlaps_rect(&self.bbox) { Vec::new() } else { self.knot.query_items_overlap(r) }
    }

    fn query_items_contained_by(&self, r: &Rect) -> Vec<ItemId> {
        if !r.overlaps_rect(&self.bbox) { Vec::new() } else { self.knot.query_items_contained_by(r) }
    }
}

impl QuadTree {

    /// Constructs a new QuadTree with at most 10 items per box
    pub fn new(items: &[Item]) -> Self {
        Self::new_with_max_items_per_quad(items, 10)
    }

    /// For performance, how many items should be in each quad before it gets subdivided?
    pub fn new_with_max_items_per_quad(items: &[Item], max_items: usize) -> Self {
        let items_with_id_and_bbox = items.iter().enumerate().map(|(id, i)| (*i, i.get_bbox(), ItemId(id))).collect::<Vec<_>>();
        let all_items = items_with_id_and_bbox.iter().map(|(item, _bbox, id)| (*id, *item)).collect::<BTreeMap<_, _>>();
        let all_bboxes = items_with_id_and_bbox.into_iter().map(|(_item, bbox, id)| (bbox, id)).collect::<Vec<_>>();
        let sum_of_bboxes = all_bboxes.iter().fold(Rect::zero(), |f, a| f.union(&a.0));
        Self {
            all_items,
            internal: QuadTreeInternal::new(sum_of_bboxes, all_bboxes, max_items),
        }
    }

    /// Returns the extent of all items
    pub fn bbox(&self) -> Rect {
        self.internal.bbox
    }

    /// Query the IDs of all items that *overlap* the rect
    pub fn get_ids_that_overlap(&self, rect: &Rect) -> Vec<ItemId> {
        self.internal.query_items_overlap(rect)
    }

    /// Query the IDs of all items *completely contained* by the rect
    pub fn get_ids_contained_by(&self, rect: &Rect) -> Vec<ItemId> {
        self.internal.query_items_contained_by(rect)
    }

    /// Returns all points in the QuadTree that are within the bounds of `rect`
    pub fn get_points_contained_in(&self, rect: &Rect) -> Vec<Point> {
        self.internal.query_items_contained_by(rect)
        .into_iter()
        .filter_map(|id| self.all_items.get(&id))
        .filter_map(|item| match item { Item::Point(p) => Some(*p), Item::Rect(_) => None })
        .collect()
    }

    /// Returns all rectangles in the QuadTree that *overlap* `rect`
    pub fn get_rects_that_overlap(&self, rect: &Rect) -> Vec<Rect> {
        self.internal.query_items_overlap(rect)
        .into_iter()
        .filter_map(|id| self.all_items.get(&id))
        .filter_map(|item| match item { Item::Point(_) => None, Item::Rect(r) => Some(*r) })
        .collect()
    }

    /// Returns all rectangles in the QuadTree that *are completely contained by* `rect`
    pub fn get_rects_contained_by(&self, rect: &Rect) -> Vec<Rect> {
        self.internal.query_items_contained_by(rect)
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
