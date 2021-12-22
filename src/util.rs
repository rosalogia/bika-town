use crate::rendering::DirectionalAnimation;
use legion::World;
use sdl2::render::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GlobalState<'a> {
    pub last_id: u32,
    pub world: World,
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<sdl2::video::WindowContext>,
    pub texture_map: HashMap<String, Texture<'a>>,
    pub directional_sprite_map: HashMap<u32, Vec<DirectionalAnimation>>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    window_dimensions: (u32, u32),
    default_tile_dimensions: (u32, u32),
    asset_dir: String,
}

impl Config {
    pub fn from(file: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file)?;
        let config: Config = ron::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Deserialize, Serialize)]
pub struct TileSheet {
    tile_dimensions: (u32, u32),
    relative_path: String,
}

impl TileSheet {
    pub fn from(file: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file)?;
        let tile_sheet: TileSheet = ron::from_str(&contents)?;
        Ok(tile_sheet)
    }
}
