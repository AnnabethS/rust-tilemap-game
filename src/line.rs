use crate::Point;
use crate::WIN_WIDTH;
use crate::WIN_HEIGHT;

pub struct Line {
    m: f32,
    c: f32,
    from: Point
}

impl Line {
    // from p1 to p2
    pub fn new_from_points(p1: &Point, p2: &Point) -> Line {
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

    pub fn find_closest_edge_hit(&self, p: &Point) -> Point {
        if self.m.is_finite()  && self.m != 0.0{
            let left = Point::new(0.0, self.c);
            let right = Point::new(WIN_WIDTH as f32, (self.m * (WIN_WIDTH as f32)) + self.c);
            let top = Point::new((-self.c) / self.m, 0.0);
            let bot = Point::new((WIN_HEIGHT as f32 - self.c) / self.m, WIN_HEIGHT as f32);
            let mut shortest: &Point = &Point::new(f32::MAX, f32::MAX);
            for i in vec!(&left, &right, &top, &bot) {
                if p.x != i.x && p.y != i.y && !Point::between(&i, &p, &self.from)
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
