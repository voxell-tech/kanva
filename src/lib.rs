use core::ops::{Deref, DerefMut};
use std::collections::HashMap;

use vello::Scene;
use vello::kurbo::{
    Affine, BezPath, Circle, CubicBez, Line, QuadBez, Rect, RoundedRect, Shape, Stroke,
};
use vello::peniko::{BlendMode, Brush, Fill};

pub struct Kanva {
    pub root: NodeId,
    pub nodes: HashMap<NodeId, MutDetect<Node>>,
    pub shapes: HashMap<ShapeId, MutDetect<KanvaShape>>,
    pub fills: HashMap<ShapeId, KanvaFill>,
    pub strokes: HashMap<ShapeId, KanvaStroke>,
}

impl Kanva {
    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id).map(|n| &**n)
    }

    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut MutDetect<Node>> {
        self.nodes.get_mut(id)
    }

    pub fn get_shape(&self, id: &ShapeId) -> Option<&KanvaShape> {
        self.shapes.get(id).map(|s| &**s)
    }

    pub fn render(&mut self) -> Scene {
        // if let Some(mut root) = self.nodes.get(&self.root) {}
        todo!()
    }
}

pub type Id = u64;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(Id);

pub struct Node {
    pub transform: Affine,
    pub children: Vec<NodeId>,
    pub shape_id: Option<ShapeId>,
}

// TODO: Think of a better way...
// impl MutDetect<Node> {
//     /// Render only the shape of the current node.
//     pub fn render_self(&mut self, scene: &mut Scene, kanva: &Kanva) {
//         if let Some(id) = self.shape_id.as_ref()
//             && let Some(shape) = kanva.shapes.get(id)
//         {
//             if let Some(fill) = kanva.fills.get(id) {
//                 shape.fill(FillBuilder::new(scene, self.transform, fill));
//             }

//             if let Some(stroke) = kanva.strokes.get(id) {
//                 shape.stroke(StrokeBuilder::new(scene, self.transform, stroke));
//             }
//         }

//         self.reset();
//     }
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapeId(Id);

pub enum KanvaShape {
    Cirlcle(Circle),
    Rect(Rect),
    RoundedRect(RoundedRect),
    Line(Line),
    QuadBez(QuadBez),
    CubicBez(CubicBez),
    BezPath(BezPath),
}

impl KanvaShape {
    pub fn fill(&self, fill_builder: FillBuilder) {
        match self {
            KanvaShape::Cirlcle(circle) => fill_builder.fill(circle),
            KanvaShape::Rect(rect) => fill_builder.fill(rect),
            KanvaShape::RoundedRect(rounded_rect) => fill_builder.fill(rounded_rect),
            KanvaShape::Line(line) => fill_builder.fill(line),
            KanvaShape::QuadBez(quad_bez) => fill_builder.fill(quad_bez),
            KanvaShape::CubicBez(cubic_bez) => fill_builder.fill(cubic_bez),
            KanvaShape::BezPath(bez_path) => fill_builder.fill(bez_path),
        }
    }

    pub fn stroke(&self, stroke_builder: StrokeBuilder) {
        match self {
            KanvaShape::Cirlcle(circle) => stroke_builder.stroke(circle),
            KanvaShape::Rect(rect) => stroke_builder.stroke(rect),
            KanvaShape::RoundedRect(rounded_rect) => stroke_builder.stroke(rounded_rect),
            KanvaShape::Line(line) => stroke_builder.stroke(line),
            KanvaShape::QuadBez(quad_bez) => stroke_builder.stroke(quad_bez),
            KanvaShape::CubicBez(cubic_bez) => stroke_builder.stroke(cubic_bez),
            KanvaShape::BezPath(bez_path) => stroke_builder.stroke(bez_path),
        }
    }
}

pub struct FillBuilder<'a> {
    scene: &'a mut Scene,
    transform: Affine,
    fill: &'a KanvaFill,
}

impl<'a> FillBuilder<'a> {
    pub fn new(scene: &'a mut Scene, transform: Affine, fill: &'a KanvaFill) -> Self {
        Self {
            scene,
            transform,
            fill,
        }
    }

    pub fn fill(self, shape: &impl Shape) {
        self.scene.fill(
            self.fill.style,
            self.transform,
            &self.fill.brush,
            self.fill.transform,
            shape,
        )
    }
}

pub struct KanvaFill {
    pub style: Fill,
    pub brush: Brush,
    pub transform: Option<Affine>,
}

pub struct StrokeBuilder<'a> {
    scene: &'a mut Scene,
    transform: Affine,
    stroke: &'a KanvaStroke,
}

impl<'a> StrokeBuilder<'a> {
    pub fn new(scene: &'a mut Scene, transform: Affine, stroke: &'a KanvaStroke) -> Self {
        Self {
            scene,
            transform,
            stroke,
        }
    }

    pub fn stroke(self, shape: &impl Shape) {
        self.scene.stroke(
            &self.stroke.style,
            self.transform,
            &self.stroke.brush,
            self.stroke.transform,
            shape,
        )
    }
}

pub struct KanvaStroke {
    pub style: Stroke,
    pub brush: Brush,
    pub transform: Option<Affine>,
}

pub struct Layer {
    pub clip: ShapeId,
    pub blend: Option<LayerBlend>,
}

pub struct LayerBlend {
    pub alpha: f32,
    pub blend_mode: BlendMode,
}

/// Mutation detection through [`DerefMut`] implementation.
pub struct MutDetect<T> {
    inner: T,
    mutated: bool,
}

impl<T> MutDetect<T> {
    /// Whether the inner value has been mutated.
    pub fn mutated(&self) -> bool {
        self.mutated
    }

    // /// Set mutated to `false` manually.
    // pub(crate) fn reset(&mut self) {
    //     self.mutated = false;
    // }
}

impl<T> Deref for MutDetect<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for MutDetect<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutated = true;
        &mut self.inner
    }
}
