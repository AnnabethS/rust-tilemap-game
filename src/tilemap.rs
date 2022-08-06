use std::fs;
use notan::draw::*;
use notan::prelude::*;

pub struct Tile<'a> {
    pub solid: bool,
    img: &'a Texture,
}

pub struct TileMap<'a> {
    pub map: Vec<Tile<'a>>,
    pub width: u32,
    pub height: u32,
    tiles: Vec<Texture>,
}

impl <'a>TileMap<'a> {
    pub fn new(width: u32, height: u32, gfx: &mut Graphics) -> TileMap<'a> {
        let mut t = TileMap {
            map: Vec::new(),
            width,
            height,
            tiles: Vec::new()
        };
        t.map.reserve((width * height) as usize);
        match t.load_textures(gfx) {
            Ok(_) => println!("Loaded Textures for TileMap"),
            Err(e) => panic!("Failed loading textures for TileMap with error:\n{}", e),
        };
        t
    }

    pub fn new_from_file<'b>(path: &str, gfx: &'b mut Graphics) -> TileMap<'a> {
        let t = TileMap::new(32, 32, gfx);
        let map_string = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => panic!("could not load map at {}, had error: {}", path, e),
        };
        print!("loaded map:\n{}", map_string);
        t
    }

    fn load_textures(&mut self, g: &mut Graphics) -> Result<(), String> {
        let mut files:Vec<&[u8]> = Vec::new();
        files.push(include_bytes!("assets/tile1.png"));
        files.push(include_bytes!("assets/tile2.png"));
        for f in files {
            self.tiles.push(TileMap::load_single_texture(g, f)?);
        }
        Ok(())
    }

    fn load_single_texture(g: &mut Graphics, data: &[u8]) -> Result<Texture, String> {
        g
        .create_texture()
        .from_image(data)
        .build()
    }

    pub fn draw(&self, g: &mut Graphics) {
        let mut d = g.create_draw();
        d.clear(Color::BLACK);
        d.image(&self.tiles[0]).position(64.0, 64.0).size(64.0, 64.0);
        d.image(&self.tiles[1]).position(128.0, 64.0).size(64.0, 64.0);
        g.render(&d);
    }
}
