use super::*;

mod outliner;
mod polygon;

use polygon::{Polygon, TriangulatePolygon};

pub fn glyph(face: &ttf_parser::Face, glyph_id: ttf_parser::GlyphId, scale: f32) -> Result<Glyph> {
    let mut triangulator = Triangulator {
        pos: Vec2::ZERO,
        path: vec![],
        polygons: vec![],
        holes: vec![],
    };
    let hor_advance = face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32 / scale;
    match face.outline_glyph(glyph_id, &mut triangulator) {
        Some(rect) => {
            let (mut vertices, indices) = triangulator.triangulate()?;
            for v in &mut vertices {
                *v /= scale;
            }
            Ok(Glyph {
                hor_advance,
                mesh: Some(GlyphMesh {
                    rect: Rect {
                        min: Vec2::new(rect.x_min as f32 / scale, rect.y_min as f32 / scale),
                        max: Vec2::new(rect.x_max as f32 / scale, rect.y_max as f32 / scale),
                    },
                    vertices,
                    indices,
                }),
            })
        }
        None => Ok(Glyph {
            hor_advance,
            mesh: None,
        }),
    }
    // println!("{bbox:?}");
}

struct Triangulator {
    pos: Vec2,
    path: Polygon,
    polygons: Vec<Polygon>,
    holes: Vec<Polygon>,
}
impl Triangulator {
    #[inline]
    fn insert_path(&mut self, path: Polygon) {
        let mut sum = 0.0;
        let mut ai = path.len() - 1;
        for bi in 0..path.len() {
            let a = path[ai];
            let b = path[bi];

            sum += (b.x - a.x) * (b.y + a.y);

            ai = bi;
        }

        if sum <= 0.0 {
            self.holes.push(path);
        } else {
            self.polygons.push(path);
        }
    }
    #[inline]
    fn triangulate(mut self) -> Result<(Vec<Vec2>, Vec<usize>)> {
        // println!("Holes: {}", self.holes.len());

        for mut polygon_a in self.holes {
            for polygon_b in &mut self.polygons {
                if polygon_inside(&polygon_a, polygon_b) {
                    let mut best_len = f32::MAX;
                    let mut best_a = 0;
                    let mut best_b = 0;

                    for ai in 0..polygon_a.len() {
                        for bi in 0..polygon_b.len() {
                            let a = polygon_a[ai];
                            let b = polygon_b[bi];
                            let len = a.distance_squared(b);

                            if !(len >= best_len
                                || line_intersects_polygon(a, b, &polygon_a)
                                || line_intersects_polygon(a, b, polygon_b))
                            {
                                best_len = len;
                                best_a = ai;
                                best_b = bi;
                            }
                        }
                    }

                    // println!("Len: {best_len}");

                    let a = polygon_a[best_a];
                    let b = polygon_b[best_b];

                    polygon_a.rotate_left(best_a);
                    polygon_b.rotate_left(best_b + 1);
                    polygon_b.extend(polygon_a);
                    polygon_b.push(a);
                    polygon_b.push(b);

                    break;
                }
            }
        }

        // let mut points: Vec<[i16; 2]> = vec![];
        // for polygon in &self.polygons {
        //     for point in polygon {
        //         let point = [point.x as i16, point.y as i16];

        //         if !points.contains(&point) {
        //             let mut closest_pair = 0;
        //             let mut closest_pair_distance = u32::MAX;

        //             let ai = points.len().wrapping_sub(1);
        //             for bi in 0..points.len() {
        //                 let a = points[ai];
        //                 let b = points[bi];
        //                 let distance = ((point[0].abs_diff(a[0]) as u32).pow(2)
        //                     + (point[1].abs_diff(a[1]) as u32).pow(2))
        //                     + ((point[0].abs_diff(b[0]) as u32).pow(2)
        //                         + (point[1].abs_diff(b[1]) as u32).pow(2));
        //                 if distance < closest_pair_distance {
        //                     closest_pair_distance = distance;
        //                     closest_pair = bi;
        //                 }
        //             }
        //             points.insert(closest_pair, point);
        //         }
        //     }
        // }

        let mut points_map: HashMap<u64, usize> = HashMap::default();
        let mut triangles = vec![];

        for polygon in self.polygons {
            polygon.triangulate(&mut points_map, &mut triangles)?;
        }

        let mut vertices: Vec<_> = points_map.into_iter().collect();
        vertices.sort_by(|a, b| a.1.cmp(&b.1));
        // if vertices.len() > 0 {
        //     println!("{:?}", vertices);
        //     exit(0);
        // }
        let vertices = vertices
            .into_iter()
            .map(|(p, _)| unsafe { mem::transmute(p) })
            .collect();

        Ok((vertices, triangles))
    }
}

#[inline(always)]
fn lines_intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    #[inline(always)]
    fn ccw(a: Vec2, b: Vec2, c: Vec2) -> bool {
        let ab = b - a;
        let ac = c - a;
        ab.x * ac.y > ab.y * ac.x
        // (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}
#[inline(always)]
pub fn line_intersects_polygon(a: Vec2, b: Vec2, polygon: &[Vec2]) -> bool {
    let mut i = polygon.len() - 1;
    for j in 0..polygon.len() {
        if lines_intersect(a, b, polygon[i], polygon[j]) {
            return true;
        }
        i = j;
    }
    false
}

#[inline(always)]
fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let mut c = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let a = polygon[j];
        let b = polygon[i];

        if (b.y > point.y) != (a.y > point.y) {
            let pa = point - a;
            let ba = b - a;
            if pa.x < ba.x * pa.y / ba.y {
                c = !c;
            }
        }

        j = i;
    }
    c
}
#[inline(always)]
pub fn polygon_inside(a: &[Vec2], b: &[Vec2]) -> bool {
    for &point in a {
        if !point_in_polygon(point, b) {
            return false;
        }
    }
    true
}
