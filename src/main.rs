pub mod tilemap;
pub mod rect;
pub mod fow;
pub mod point;
pub mod line;

use notan::draw::*;
use notan::prelude::*;
use std::process;
use rect::*;
use point::*;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

#[derive(AppState)]
struct State {
    map: tilemap::TileMap,
    collision_rects: Vec<Rect>,
    mouse_pos: Point,
    test_points: Vec<Point>,
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
        test_points: Vec::new(),
    };
    s
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Escape) ||
    app.keyboard.is_down(KeyCode::Q) {
        process::exit(0);
    }
    (state.mouse_pos.x, state.mouse_pos.y) = app.mouse.position();
    state.test_points = fow::gen_fow_polygon(state.collision_rects[4], state.mouse_pos);
}


fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut d = gfx.create_draw();
    d.clear(Color::BLACK);

    state.map.draw(&mut d);
    for r in &state.collision_rects {
        r.draw(&mut d);
    }

    d.circle(5.0).position(state.mouse_pos.x, state.mouse_pos.y).color(Color::BLUE);

    let mut can_draw = state.test_points.len() > 0;
    for p in state.test_points.iter() {
        if !(p.x >= 0.0 && p.x <= WIN_WIDTH as f32 && p.y >= 0.0 && p.y <= WIN_HEIGHT as f32) {
            can_draw = false;
        }
    }
    if can_draw {
        let mut x = d.path();
        x.fill().move_to(state.test_points[0].x, state.test_points[0].y);
        for p in state.test_points.iter() {
            x.line_to(p.x, p.y);
        }
        x.line_to(state.test_points[0].x, state.test_points[0].y);
        x.close().color(Color::PINK).alpha(0.5);
    }


    gfx.render(&d);
}
