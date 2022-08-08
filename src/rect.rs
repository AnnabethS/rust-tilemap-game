use notan::prelude::*;
use notan::draw::*;

#[derive(Copy, Clone)]
pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
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
}
