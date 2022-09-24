use std::fmt;

use notan::prelude::*;
use notan::draw::*;

use crate::fow::Rect;
use crate::point::*;

pub struct Player {
    pub position: Point,
    speed: f32,
}

impl Player {
    pub fn update(&mut self, app: &App, collision_rects: &Vec<Rect>) {
        self.mv(app, collision_rects);
    }

    pub fn mv(&mut self, app: &App, collision_rects: &Vec<Rect>) {
        let vert;
        if app.keyboard.is_down(KeyCode::W) || app.keyboard.is_down(KeyCode::Up) {
            vert = -1.0;
        }
        else if app.keyboard.is_down(KeyCode::S) || app.keyboard.is_down(KeyCode::Down) {
            vert = 1.0;
        }
        else {
            vert = 0.0;
        }

        let horiz;

        if app.keyboard.is_down(KeyCode::A) || app.keyboard.is_down(KeyCode::Left) {
            horiz = -1.0;
        }
        else if app.keyboard.is_down(KeyCode::D) || app.keyboard.is_down(KeyCode::Right) {
            horiz = 1.0;
        }
        else {
            horiz = 0.0;
        }

        let mut mv_amount = Point::new(horiz * self.speed, vert * self.speed);

        if horiz != 0.0 && vert != 0.0 {
            let scale = libm::sqrtf(1.0);
            mv_amount.x *= scale;
            mv_amount.y *= scale;
        }

        if self.can_do_move(&mv_amount, collision_rects) {
            self.position += mv_amount;
        }
        else if self.can_do_move(&Point::new(mv_amount.x, 0.0), collision_rects) {
            self.position.x += mv_amount.x;
        }
        else if self.can_do_move(&Point::new(0.0, mv_amount.y), collision_rects) {
            self.position.y += mv_amount.y;
        }

    }

    pub fn new(x: f32, y: f32) -> Player {
        Player {
            position: Point::new(x, y),
            speed: 3.0
        }
    }

    pub fn draw(&self, d: &mut Draw) {
        d.circle(10.0)
            .position(self.position.x, self.position.y)
            .fill()
            .color(Color::PINK);
    }

    fn can_do_move(&self, mv_amnt: &Point, collision_rects: &Vec<Rect>) -> bool {
        let new_pos = self.position + *mv_amnt;
        for r in collision_rects.iter() {
            if r.contains_point(&new_pos) {
                return false;
            }
        }
        true
    }
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player at x={}, y={}",
               self.position.x, self.position.y)
    }
}
