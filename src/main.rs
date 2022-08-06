pub mod tilemap;

use notan::draw::*;
use notan::prelude::*;
use std::process;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

#[derive(AppState)]
struct State {
    map: tilemap::TileMap<'static>,
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
    let s = State {
        map: tilemap::TileMap::new_from_file("test.map", gfx)
    };
    s
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Escape) ||
    app.keyboard.is_down(KeyCode::Q) {
        process::exit(0);
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    state.map.draw(gfx);
}

