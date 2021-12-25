use serde::{Deserialize, Serialize};

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
pub struct DirectionalSpriteInfo {
    pub path: String,
    pub name: String,
    pub sprite_dimensions: Vec<(String, Vec<(u32, u32)>)>,
}

#[derive(Deserialize, Serialize)]
pub struct SpriteSheetInfo {
    pub path: String,
    pub name: String,
    pub sprite_dimensions: (u32, u32),
}

#[derive(Deserialize, Serialize)]
pub struct SpriteInfo {
    pub directional_sprites: Vec<DirectionalSpriteInfo>,
    pub sprites: Vec<SpriteSheetInfo>,
}

impl SpriteInfo {
    pub fn from(file: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(file)?;
        let sprite_info = ron::from_str(&contents)?;
        Ok(sprite_info)
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
