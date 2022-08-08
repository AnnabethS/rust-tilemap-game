use crate::Rect;

use std::fs;
use notan::draw::*;
use notan::prelude::*;

const TILE_SIZE: f32 = 32.0;

const MAP_OFFSET: f32 = 64.0;

#[derive(Clone)]
pub struct Tile {
    pub solid: bool,
    img: Texture,
}

#[derive(Clone)]
pub struct TileMap {
    pub map: Vec<usize>,
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
    dbg_tile: Tile
}

impl TileMap {
    pub fn new(width: u32, height: u32, gfx: &mut Graphics) -> TileMap {
        let dbg_tile = match TileMap::load_single_texture(gfx, include_bytes!("assets/debug.png")) {
            Ok(tex) => tex,
            Err(e) => panic!("couldnt load debug texture with error:\n{}", e),
        };
        let mut t = TileMap {
            map: Vec::new(),
            width,
            height,
            tiles: Vec::new(),
            dbg_tile: Tile {solid: false, img: dbg_tile },
        };
        t.map.reserve((width * height) as usize);
        match t.load_textures(gfx) {
            Ok(_) => println!("Loaded Textures for TileMap"),
            Err(e) => panic!("Failed loading textures for TileMap with error:\n{}", e),
        };
        t
    }

    pub fn new_from_file<'b>(path: &str, gfx: &'b mut Graphics, rects: &mut Vec<Rect>) -> TileMap {
        let map_string = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => panic!("could not load map at {}, had error: {}", path, e),
        };
        print!("loaded map:\n{}", map_string);
        let lines = map_string.lines();
        let mut height: u32 = 0;
        let mut width: u32 = 0;
        for _ in lines.clone().nth(0).unwrap().split(','){
            width+=1;
        }
        width -=1; // because otherwise it counts the newline too
        for _ in lines.clone() {
            height += 1;
        }
        let mut t = TileMap::new(width, height, gfx);
        t.load_terrain(map_string.as_str());
        println!("width: {}\nheight: {}", width, height);
        t.generate_collision_rects(rects);
        t
    }

    fn generate_collision_rects(&self, rects: &mut Vec<Rect>) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.tile_is_solid(x,y) {
                    rects.push(Rect::new_sq(
                        x as f32 * TILE_SIZE + MAP_OFFSET,
                        y as f32 * TILE_SIZE + MAP_OFFSET,
                        TILE_SIZE));
                }
            }
        }
    }

    fn tile_is_solid(&self, x: u32, y: u32) -> bool {
        self.tiles
            .get(self.map[(x + self.width * y) as usize]) // try get the tile we want
            .unwrap_or(&self.dbg_tile) // if we cant get it, fall back on dbg_tile
            .solid // check if its solid or not
    }

    pub fn load_terrain(&mut self, mapstring: &str) {
        for line in mapstring.lines() {
            for tile in line.split(',') {
                if tile != "" {
                    let index = match tile.parse::<usize>() {
                        Ok(i) => i,
                        Err(e) => panic!("Tried to parse '{:?}' with error:\n{}", tile, e),
                    };
                    self.map.push(index);
                }
            }
        }
    }

    fn load_textures(&mut self, g: &mut Graphics) -> Result<(), String> {
        let mut files:Vec<&[u8]> = Vec::new();
        files.push(include_bytes!("assets/tile1.png"));
        files.push(include_bytes!("assets/tile2.png"));
        for f in files {
            self.tiles.push(Tile{ solid: false, img: TileMap::load_single_texture(g, f).unwrap()} );
        }
        self.tiles[1].solid = true;
        Ok(())
    }

    fn load_single_texture(g: &mut Graphics, data: &[u8]) -> Result<Texture, String> {
        g
        .create_texture()
        .from_image(data)
        .build()
    }

    pub fn draw(&self, d: &mut Draw) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_single_tile(d, x, y);
            }
        }
    }

    fn draw_single_tile(&self, d: &mut Draw, x:u32, y:u32) {
        let index = (y*self.width + x) as usize;
        d.image(&self.tiles.get(self.map[index]).unwrap_or(&self.dbg_tile).img)
            .position(MAP_OFFSET + x as f32 * TILE_SIZE, MAP_OFFSET + y as f32 * TILE_SIZE)
            .size(TILE_SIZE, TILE_SIZE);
    }
}
