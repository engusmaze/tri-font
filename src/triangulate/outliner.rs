use core::mem;

use super::*;

impl ttf_parser::OutlineBuilder for Triangulator {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        let pos = Vec2::new(x, y);
        self.pos = pos;
    }
    #[inline]
    fn line_to(&mut self, x: f32, y: f32) {
        let pos = Vec2::new(x, y);
        self.pos = pos;
        self.path.push(pos);
    }

    #[inline]
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let b = Vec2::new(x1, y1);
        let c = Vec2::new(x2, y2);
        for point in quad(self.pos, b, c) {
            self.path.push(point);
        }
        // let s1 = lerp(self.pos, b, 0.5);
        // let s2 = lerp(b, c, 0.5);
        // self.path.push(lerp(s1, s2, 0.5).floor());

        self.pos = c;
        self.path.push(c);
    }
    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("Curve to X1: {x1} Y1: {y1} X2: {x2} Y2: {y2} X: {x} Y: {y}");
    }
    #[inline]
    fn close(&mut self) {
        let path = mem::take(&mut self.path);
        self.insert_path(path);
    }
}

#[inline(always)]
pub fn quad(a: Vec2, b: Vec2, c: Vec2) -> QuadIterator {
    QuadIterator {
        t: 0.0,
        current_point: a,
        a,
        b,
        c,
    }
}

#[inline(always)]
fn lerp(a: Vec2, b: Vec2, delta: f32) -> Vec2 {
    a + delta * (b - a)
}

pub struct QuadIterator {
    t: f32,
    current_point: Vec2,
    a: Vec2,
    b: Vec2,
    c: Vec2,
}
impl Iterator for QuadIterator {
    type Item = Vec2;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while self.t < 1.0 {
            let s1 = lerp(self.a, self.b, self.t);
            let s2 = lerp(self.b, self.c, self.t);
            let next_point = lerp(s1, s2, self.t);

            if next_point.distance_squared(self.current_point) >= (64.0 * 64.0)
                && next_point.distance_squared(self.c) >= (32.0 * 32.0)
            {
                self.current_point = next_point;
                return Some(next_point.floor());
            }
            self.t += 0.01;
            // return Some(next_point);
        }
        None
    }
}
