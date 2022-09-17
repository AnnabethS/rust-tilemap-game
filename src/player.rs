use std::fmt;

use notan::prelude::*;

use crate::{point::*, State};

pub struct Player {
    position: Point,
    speed: f32,
}

impl Player {
    pub fn update(&mut self, app: &App) {
        self.mv(app);
    }

    pub fn mv(&mut self, app: &App) {
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

        self.position += mv_amount;
    }

    pub fn new(x: f32, y: f32) -> Player {
        Player {
            position: Point::new(x, y),
            speed: 5.0
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player at x={}, y={}",
               self.position.x, self.position.y)
    }
}
