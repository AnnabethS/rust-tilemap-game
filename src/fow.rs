use libm::acos;

use crate::{Point, WIN_WIDTH, WIN_HEIGHT};
use crate::line::Line;

pub use crate::rect::Rect;


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
    return vec!(p1, c1, c2, p2)
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
