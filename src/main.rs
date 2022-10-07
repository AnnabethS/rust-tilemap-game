pub mod tilemap;
pub mod rect;
pub mod fow;
pub mod point;
pub mod line;
pub mod player;

use notan::draw::*;
use notan::prelude::*;
use std::process;
use rect::*;
use point::*;
use fow::*;
use player::*;

const WIN_WIDTH: i32 = 1280;
const WIN_HEIGHT: i32 = 720;

#[derive(AppState)]
pub struct State {
    map: tilemap::TileMap,
    collision_rects: Vec<Rect>,
    fow: FoW,
    player: Player
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
        map: tilemap::TileMap::new_from_file("maps/test.map", gfx, &mut rects),
        collision_rects: rects.clone(),
        fow: FoW::new(gfx, rects),
        player: Player::new(50.0, 50.0),
    };
    s
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Escape) ||
    app.keyboard.is_down(KeyCode::Q) {
        process::exit(0);
    }
    state.fow.update(state.player.position, &state.collision_rects);
    state.player.update(app, &state.collision_rects);
}


fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut d = gfx.create_draw();
    d.clear(Color::BLACK);

    state.map.draw(&mut d);
    for r in &state.collision_rects {
        r.draw(&mut d);
    }

    state.player.draw(&mut d);

    state.fow.draw(gfx, &mut d);

    gfx.render(&d);
}
