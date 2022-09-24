use notan::prelude::*;
use notan::draw::*;

use crate::point::*;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn new_sq(x: f32, y: f32, s: f32) -> Self {
        Self { x, y, w:s, h:s }
    }

    pub fn draw(&self, d: &mut Draw) {
        d.rect((self.x, self.y), (self.w, self.h)).color(Color::RED).stroke(1.0);
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.x && p.x < self.x + self.w &&
        p.y >= self.y && p.y < self.y + self.h
    }
}
