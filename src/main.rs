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
use fow::*;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

#[derive(AppState)]
pub struct State {
    map: tilemap::TileMap,
    collision_rects: Vec<Rect>,
    mouse_pos: Point,
    fow: FoW,
    // test_points: Vec<Point>,
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
        collision_rects: rects.clone(),
        mouse_pos: Point { x : 0.0, y : 0.0 },
        fow: FoW::new(gfx, rects)
    };
    s
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Escape) ||
    app.keyboard.is_down(KeyCode::Q) {
        process::exit(0);
    }
    (state.mouse_pos.x, state.mouse_pos.y) = app.mouse.position();
    state.fow.update(state.mouse_pos, &state.collision_rects);
}


fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut d = gfx.create_draw();
    d.clear(Color::BLACK);

    state.map.draw(&mut d);
    for r in &state.collision_rects {
        r.draw(&mut d);
    }

    d.circle(5.0).position(state.mouse_pos.x, state.mouse_pos.y).color(Color::BLUE);

    state.fow.draw_2(gfx, &mut d);
    // state.fow.draw(&mut d);

    gfx.render(&d);
}
