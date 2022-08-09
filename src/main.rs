pub mod tilemap;
pub mod rect;
pub mod fow;

use notan::draw::*;
use notan::prelude::*;
use std::process;
use rect::*;
use libm::atan2;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

#[derive(AppState)]
struct State {
    map: tilemap::TileMap,
    collision_rects: Vec<Rect>,
    mouse_pos: Point,
    corners: (Point, Point),
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(WindowConfig::new().size(WIN_WIDTH,WIN_HEIGHT).vsync().title("snake"))
        .draw(draw)
        .update(update)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let mut rects: Vec<Rect> = Vec::new();
    let s = State {
        map: tilemap::TileMap::new_from_file("test.map", gfx, &mut rects),
        collision_rects: rects,
        mouse_pos: Point { x : 0.0, y : 0.0 },
        corners: ( Point::new(0.0, 0.0), Point::new(0.0, 0.0)),
    };
    s
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Escape) ||
    app.keyboard.is_down(KeyCode::Q) {
        process::exit(0);
    }

    (state.mouse_pos.x, state.mouse_pos.y) = app.mouse.position();
    state.corners = wide_corners(state.collision_rects[0], state.mouse_pos.x, state.mouse_pos.y);

}

fn wide_corners(r: Rect, x: f32, y: f32) -> (Point, Point) {
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
        for j in 0..4 {
            if i==j { continue; }
            let current_angle = atan2((y - corners[i].y) as f64, (x - corners[i].x) as f64) -
                                atan2((y - corners[j].y) as f64, (x - corners[j].x) as f64);
            if current_angle > widest_angle {
                widest_angle = current_angle;
                c1 = &corners[i];
                c2 = &corners[j];
            }
        }
    }
    (c1.clone(), c2.clone())
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut d = gfx.create_draw();
    d.clear(Color::BLACK);
    state.map.draw(&mut d);
    for r in &state.collision_rects {
        r.draw(&mut d);
    }
    // d.path()
    //     .move_to(64.0, 64.0)
    //     .line_to(32.0, 128.0)
    //     .line_to(45.0, 160.0)
    //     .line_to(64.0+32.0, 128.0)
    //     .close().fill().color(Color::PURPLE);
    d.circle(5.0).position(state.mouse_pos.x, state.mouse_pos.y).color(Color::BLUE);
    d.circle(5.0).position(state.corners.0.x, state.corners.0.y).color(Color::GREEN);
    d.circle(5.0).position(state.corners.1.x, state.corners.1.y).color(Color::GREEN);
    d.line(state.mouse_pos.to_tuple(), state.corners.0.to_tuple());
    d.line(state.mouse_pos.to_tuple(), state.corners.1.to_tuple());
    gfx.render(&d);
}

