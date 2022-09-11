use libm::acos;
use notan::draw::Draw;
use notan::math::*;

use notan::draw::*;
use notan::prelude::*;
use crate::{Point, WIN_WIDTH, WIN_HEIGHT};
use crate::line::Line;

pub use crate::rect::Rect;
pub use crate::State;

pub struct FoW {
    pub polys: Vec<Vec<Point>>,
    rt: RenderTexture,
    collision_rects: Vec<Rect>,
}

impl FoW {
    pub fn new(gfx: &mut Graphics, cr: Vec<Rect>) -> FoW {
        FoW {
            polys: Vec::new(),
            rt: gfx.create_render_texture(WIN_WIDTH, WIN_HEIGHT)
                   .with_filter(TextureFilter::Linear, TextureFilter::Linear)
                   .build()
                   .unwrap(),
            collision_rects: cr.clone(),
        }
    }

    pub fn update(&mut self, playerpos: Point, rects: &Vec<Rect>) {
        self.polys = Vec::new();
        for r in rects.iter() {
            self.polys.push(gen_fow_polygon(r.clone(), playerpos));
        }
    }

    pub fn draw(&self, d: &mut Draw) {
        for polygon in self.polys.iter() {
            let mut can_draw = polygon.len() > 0;
            for point in polygon.iter() {
                if !(point.x >= 0.0 && point.x <= WIN_WIDTH as f32 && point.y >= 0.0 && point.y <= WIN_HEIGHT as f32) {
                    can_draw = false;
                }
            }
            if can_draw {
                let mut path_builder = d.path();
                path_builder.fill().move_to(polygon[0].x, polygon[0].y);
                for point in polygon.iter() {
                    path_builder.line_to(point.x, point.y);
                }
                path_builder.line_to(polygon[0].x, polygon[0].y);
                path_builder.close().color(Color::BLACK);
            }
        }
    }

    pub fn draw_2(&mut self, gfx: &mut Graphics, d: &mut Draw) {
        let rt_draw:&mut Draw = &mut self.rt.create_draw();
        rt_draw.clear(Color::new(0.0, 0.0, 0.0, 0.0));

        for polygon in self.polys.iter() {
            let mut can_draw = polygon.len() > 0;
            for point in polygon.iter() {
                if !(point.x >= 0.0 && point.x <= WIN_WIDTH as f32 &&
                     point.y >= 0.0 && point.y <= WIN_HEIGHT as f32) {
                    can_draw = false;
                }
            }
            if can_draw {
                let mut path_builder = rt_draw.path();
                path_builder.fill().move_to(polygon[0].x, Self::unfuck_y(polygon[0].y));
                for point in polygon.iter() {
                    path_builder.line_to(point.x, Self::unfuck_y(point.y));
                }
                path_builder.line_to(polygon[0].x, Self::unfuck_y(polygon[0].y));
                path_builder.close().color(Color::BLACK);
            }
        }

        for r in self.collision_rects.iter() {
            rt_draw.rect((r.x, Self::unfuck_y(r.y) - 32.0), (r.w, r.h))
                .color(Color::BLACK);
        }

        gfx.render_to(&self.rt, rt_draw);

        d.image(&self.rt.texture()).alpha(0.5)
            .position(0.0, 0.0);
    }


    //TODO: figure out why this is necessary
    fn unfuck_y(y: f32) -> f32 {
        (WIN_HEIGHT as f32 / 2.0) + (-1.0 * (y - (WIN_HEIGHT as f32 / 2.0)))
    }
}


//FIXME: this is a stupid way to do this, it should be
//an immuatble reference to both
pub fn gen_fow_polygon(r: Rect, player: Point) -> Vec<Point>
{
    let (mut p1, mut p2) = wide_corners(r, player.x, player.y);
    let l1 = Line::new_from_points(&player, &p1);
    let l2 = Line::new_from_points(&player, &p2);
    let mut c1 = l1.find_closest_edge_hit(&p1);
    let mut c2 = l2.find_closest_edge_hit(&p2);
    if c1.x == c2.x || c1.y == c2.y {
        return vec!(p1, c1, c2, p2)
    }
    else if (c1.x == 0.0 && c2.x == WIN_WIDTH as f32) ||
            (c2.x == 0.0 && c1.x == WIN_WIDTH as f32) {

        /*
        *  /----------------\
        *  |                X
        *  X                |
        *  |                |
        *  |      o         |
        *  |                |
        *  \----------------/
        */

        if c2.x < c1.x {
            let swp_c = c1;
            c1 = c2;
            c2 = swp_c;

            let swp_p = p1;
            p1 = p2;
            p2 = swp_p;
        }

        if player.y <= c1.y && player.y <= c2.y {
            return vec!(p1, c1, Point::new(0.0, WIN_HEIGHT as f32), Point::new(WIN_WIDTH as f32, WIN_HEIGHT as f32), c2, p2);
        }
        else if player.y >= c1.y && player.y >= c2.y {
            return vec!(p1, c1, Point::new(0.0, 0.0), Point::new(WIN_WIDTH as f32, 0.0), c2, p2);
        }
        else {
            return Vec::new()
        }

    }
    else if (c1.y == 0.0 && c2.y == WIN_HEIGHT as f32) ||
            (c2.y == 0.0 && c1.y == WIN_HEIGHT as f32) {

        /*
        *  /-------------X--\
        *  |                |
        *  |                |
        *  | o              |
        *  |                |
        *  |                |
        *  \----X-----------/
        */

        if c2.y < c1.y {
            let swp_c = c1;
            c1 = c2;
            c2 = swp_c;

            let swp_p = p1;
            p1 = p2;
            p2 = swp_p;
        }

        if player.x <= c1.x && player.x <= c2.y {
            return vec!(p1, c1, Point::new(WIN_WIDTH as f32, 0.0), Point::new(WIN_WIDTH as f32, WIN_HEIGHT as f32), c2, p2)
        }
        else if player.x >= c1.x && player.x >= c2.x {
            return vec!(p1, c1, Point::new(0.0, 0.0), Point::new(0.0, WIN_HEIGHT as f32), c2, p2)
        }
        else {
            return Vec::new()
        }
    }
    else {
        // straddles one corner
        let t = 0.0;
        let b = WIN_HEIGHT as f32;
        let l = 0.0;
        let r = WIN_WIDTH as f32;

        if c1.y == t {
            if c2.x == l { // top and left
                return vec!(p1, c1, Point::new(0.0, 0.0), c2, p2)
            }
            else { // top and right
                return vec!(p1, c1, Point::new(WIN_WIDTH as f32, 0.0), c2, p2)
            }
        }
        if c1.x == l {
            if c2.y == t { // left and top
                return vec!(p1, c1, Point::new(0.0, 0.0), c2, p2)
            }
            else { // left and bot
                return vec!(p1, c1, Point::new(0.0, WIN_HEIGHT as f32), c2, p2)
            }
        }
        if c1.y == b {
            if c2.x == l { // bot and left
                return vec!(p1, c1, Point::new(0.0, WIN_HEIGHT as f32), c2, p2)
            }
            else { // bot and right
                return vec!(p1, c1, Point::new(WIN_WIDTH as f32, WIN_HEIGHT as f32), c2, p2)
            }
        }
        if c1.x == r {
            if c2.y == t { // right and top
                return vec!(p1, c1, Point::new(WIN_WIDTH as f32, 0.0), c2, p2)
            }
            else { // right and bot
                return vec!(p1, c1, Point::new(WIN_WIDTH as f32, WIN_HEIGHT as f32), c2, p2)
            }
        }
        unreachable!()
    }
}

pub fn wide_corners(r: Rect, x: f32, y: f32) -> (Point, Point) {
    let player = Point {x,y};
    let mut corners: Vec<Point> = Vec::new();
    corners.reserve(4);
    corners.push(Point::new(r.x, r.y));
    corners.push(Point::new(r.x + r.w, r.y));
    corners.push(Point::new(r.x, r.y + r.h));
    corners.push(Point::new(r.x + r.w, r.y + r.h));
    let mut c1: &Point = corners.get(0).unwrap();
    let mut c2: &Point = corners.get(1).unwrap();
    let mut widest_angle = 0.0;
    for i in 0..4 {
        for j in i..4 {
            if i==j { continue; }
            let current_angle = cosine_rule_angle(&player, &corners[i], &corners[j]);
            if current_angle >= widest_angle {
                widest_angle = current_angle;
                c1 = &corners[i];
                c2 = &corners[j];
            }
        }
    }
    (c1.clone(), c2.clone())
}

pub fn cosine_rule_angle(point_a: &Point, point_b: &Point, point_c: &Point) -> f64 {
    let a = point_b.dist(&point_c);
    let b = point_a.dist(&point_c);
    let c = point_a.dist(&point_b);
    acos(((b.powi(2) + c.powi(2) - a.powi(2)) / (2.0 * b * c)) as f64)
}
