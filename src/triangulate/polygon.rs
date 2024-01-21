use std::mem;

use anyhow::{Error, Ok};

use super::*;

pub type Polygon = Vec<Vec2>;
pub trait TriangulatePolygon {
    fn triangulate(
        self,
        points: &mut HashMap<u64, usize>,
        triangles: &mut Vec<usize>,
    ) -> Result<()>;
}
impl TriangulatePolygon for Polygon {
    fn triangulate(
        mut self,
        points: &mut HashMap<u64, usize>,
        triangles: &mut Vec<usize>,
    ) -> Result<()> {
        'main_loop: while self.len() >= 3 {
            let mut ai = self.len() - 2;
            let mut bi = self.len() - 1;
            for ci in 0..self.len() {
                let a = self[ai];
                let b = self[bi];
                let c = self[ci];

                if is_cw(a, b, c) {
                    if !self.iter().any(|p| point_in_triangle(*p, a, b, c)) {
                        fn get_index(points: &mut HashMap<u64, usize>, point: Vec2) -> usize {
                            let i = points.len();
                            *points.entry(unsafe { mem::transmute(point) }).or_insert(i)
                        }

                        triangles.extend([
                            get_index(points, a),
                            get_index(points, b),
                            get_index(points, c),
                        ]);
                        self.remove(bi);

                        continue 'main_loop;
                    }
                }
                ai = bi;
                bi = ci;
            }
            return Err(Error::msg(format!("Len failed at: {}", self.len())));
        }
        Ok(())
    }
}

#[inline(always)]
pub fn is_cw(a: Vec2, b: Vec2, c: Vec2) -> bool {
    let ab = b - a;
    let ac = c - a;
    ab.x * ac.y <= ab.y * ac.x
}

#[inline(always)]
pub fn point_in_triangle(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    if point == a || point == b || point == c {
        return false;
    }

    let v0 = c - a;
    let v1 = b - a;
    let v2 = point - a;

    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}
