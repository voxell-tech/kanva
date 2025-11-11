use std::collections::HashMap;

use kurbo::{Affine, BezPath, Circle, Line, QuadBez, Rect, RoundedRect};

pub struct Kanva<Tag = String> {
    pub groups: HashMap<GroupId, KanvaGroup>,
    pub shapes: HashMap<ShapeId, KanvaShape>,
    pub group_tags: HashMap<Tag, GroupId>,
    pub shape_tags: HashMap<Tag, ShapeId>,
}

pub type Id = u32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupId(Id);

pub struct KanvaGroup {
    pub parent_id: Option<GroupId>,
    pub transform: Affine,
    pub shapes: Vec<ShapeId>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapeId(Id);

pub enum KanvaShape {
    Line(Line),
    Cirlcle(Circle),
    Rect(Rect),
    RoundedRect(RoundedRect),
    BezPath(BezPath),
    QuadBez(QuadBez),
}
