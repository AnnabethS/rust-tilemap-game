use std::fmt;

use libm::acos;

use crate::WIN_WIDTH;
use crate::WIN_HEIGHT;
pub use crate::rect::Rect;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

struct Line {
    m: f32,
    c: f32,
    from: Point
}

impl Line {
    // from p1 to p2
    fn new_from_points(p1: &Point, p2: &Point) -> Line {
        let m;
        if p1.x == p2.x {
            m = f32::INFINITY;
        }
        else if p1.y == p2.y {
            m = 0.0;
        }
        else{
            m = (p1.y - p2.y) / (p1.x - p2.x);
        }

        let c = p1.y - (p1.x * m);
        Line {m, c, from: p1.clone()}
    }

    // problem is entirely vert or horizontal lines

    fn find_closest_edge_hit(&self, p: &Point) -> Point {
        if self.m.is_finite()  && self.m != 0.0{
            let left = Point::new(0.0, self.c);
            let right = Point::new(WIN_WIDTH as f32, (self.m * (WIN_WIDTH as f32)) + self.c);
            let top = Point::new((-self.c) / self.m, 0.0);
            let bot = Point::new((WIN_HEIGHT as f32 - self.c) / self.m, WIN_HEIGHT as f32);
            println!("l: {}, r: {}, t: {}, b: {}", left, right, top, bot);
            let mut shortest: &Point = &Point::new(f32::MAX, f32::MAX);
            for i in vec!(&left, &right, &top, &bot) {
                if p.x != i.x && p.y != i.y && !between(&i, &p, &self.from)
                    // && i.x >= 0.0 && i.x < WIN_WIDTH as f32 && i.y >= 0.0 && i.y < WIN_HEIGHT as f32
                {
                    if shortest.dist(p) > i.dist(p) {
                        shortest = i;
                    }
                }
            };
            return *shortest;
        }
        else if self.m.is_infinite() {
            // line is vertical
            if p.y < self.from.y {
                return Point {x: p.x, y: 0.0};
            }
            else {
                return Point {x: p.x, y: WIN_HEIGHT as f32};
            }
        }
        else if self.m == 0.0 {
            // line is horizontal
            if p.x < self.from.x {
                return Point {x: 0.0, y: p.y};
            }
            else {
                return Point {x: WIN_WIDTH as f32, y: p.y};
            }
        }
        else {
            unreachable!()
        }
    }
}

fn between(p1: &Point, p2: &Point, check: &Point) -> bool {
    (p1.x < check.x && check.x < p2.x) || (p2.x < check.x && check.x < p1.x) &&
        (p1.y < check.y && check.y < p2.y) || (p2.y < check.y && check.y < p1.y)
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn dist(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn gen_fow_polygon(r: Rect, player: Point) -> Vec<Point>
{
    let (p1, p2) = wide_corners(r, player.x, player.y);
    let l1 = Line::new_from_points(&player, &p1);
    let l2 = Line::new_from_points(&player, &p2);
    let c1 = l1.find_closest_edge_hit(&p1);
    let c2 = l2.find_closest_edge_hit(&p2);
    vec!(p1, c1, c2, p2)
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
