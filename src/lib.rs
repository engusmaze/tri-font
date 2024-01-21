use std::mem;

use anyhow::Result;
use fhash::RandomState;
use glam::Vec2;

mod read;
mod triangulate;

type HashMap<K, V> = hashbrown::HashMap<K, V, RandomState>;

#[derive(Debug, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}
#[derive(Debug, Clone)]
pub struct GlyphMesh {
    pub rect: Rect,
    pub vertices: Vec<Vec2>,
    pub indices: Vec<usize>,
}
#[derive(Debug, Clone)]
pub struct Glyph {
    pub hor_advance: f32,
    pub mesh: Option<GlyphMesh>,
}
#[derive(Debug)]
pub struct Font {
    pub map: HashMap<char, Glyph>,
    pub cell_height: f32,
}
