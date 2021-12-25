use crate::models::components::*;
use crate::util::*;
use sdl2::image::*;
use sdl2::render::*;
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub static WINDOW_WIDTH: u32 = 1008;
pub static WINDOW_HEIGHT: u32 = 1008;

/// A container for a sprite-sheet texture
/// with methods for drawing any sprite from
/// the sheet to the screen
pub struct SpriteSheet<'a> {
    /// The width and height of each sprite in the sheet
    sprite_dimensions: (u32, u32),
    /// A vector of SDL2 rectangles, each encompassing the region
    /// of the sprite-sheet texture corresponding to one sprite
    sprites: Vec<sdl2::rect::Rect>,
    /// The texture loaded from disk corresponding to the sprite-sheet
    texture: Texture<'a>,
    /// A counter used for determining which frame of an animation to play
    /// if the sprite-sheet represents an animation
    animation_counter: u32,
}

impl<'a> SpriteSheet<'a> {
    /// Produces a new SpriteSheet by loading the texture
    /// from disk and cutting it into rectangles sized
    /// according to the given `sprite_dimensions`.
    pub fn new(
        sprite_dimensions: (u32, u32),
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &str,
    ) -> Self {
        let texture = texture_creator.load_texture(path).unwrap();
        let (max_width, max_height) = {
            let tq = texture.query();
            (tq.width, tq.height)
        };

        let (sprite_width, sprite_height) = sprite_dimensions;

        // We do some math to figure out how many sprites are on the sheet ...
        let (h_size, v_size) = ((max_width / sprite_width), (max_height / sprite_height));
        let mut sprites: Vec<sdl2::rect::Rect> = Vec::with_capacity((h_size * v_size) as usize);

        // ... then start pushing them into the sprites vector
        for j in 0..v_size {
            for i in 0..h_size {
                let r = sdl2::rect::Rect::new(
                    (i * sprite_width) as i32,
                    (j * sprite_height) as i32,
                    sprite_width,
                    sprite_height,
                );
                sprites.push(r);
            }
        }

        Self {
            sprite_dimensions,
            sprites,
            texture,
            animation_counter: 0,
        }
    }

    /// Draws the i-th sprite on the sheet to the screen at (x, y)
    pub fn draw_to(&self, i: usize, x: i32, y: i32, canvas: &mut WindowCanvas) {
        let sprite = self.sprites[i];
        let dst = sdl2::rect::Rect::new(x, y, self.sprite_dimensions.0, self.sprite_dimensions.1);
        canvas.copy(&self.texture, sprite, dst).unwrap();
    }

    /// Draws only a portion of the i-th sprite to the screen at (x, y), where
    /// the portion is specified by percent width and percent height. Useful
    /// for drawing various UI bars.
    pub fn draw_portion_of(
        &self,
        i: usize,
        x: i32,
        y: i32,
        percent_width: f32,
        percent_height: f32,
        canvas: &mut WindowCanvas,
    ) {
        let sprite = self.sprites[i];
        let width = (percent_width * (self.sprite_dimensions.0 as f32)) as u32;
        let height = (percent_height * (self.sprite_dimensions.1 as f32)) as u32;
        let dst = sdl2::rect::Rect::new(x, y, width, height);
        canvas.copy(&self.texture, sprite, dst).unwrap();
    }

    /// Assumes that each sprite in the sheet is the frame of an animation
    /// and draws the next frame to the screen at (x, y) as determined by
    /// the `animation_counter`.
    pub fn draw_animated(&mut self, x: i32, y: i32, canvas: &mut WindowCanvas) {
        self.draw_to(
            self.animation_counter as usize % self.sprites.len(),
            x,
            y,
            canvas,
        );

        self.animation_counter += 1;
    }

    /// Assumes that the sprite-sheet corresponds to a tileset being used
    /// to draw a tilemap, and draws a tilemap given the path to a .tmx (tiled)
    /// file.
    pub fn draw_map(&self, canvas: &mut WindowCanvas, tilemap_path: &str) {
        // There's some funkiness with how .tmx files work that's show-cased in this
        // function
        let tilemap = tiled::parse_file(std::path::Path::new(tilemap_path)).unwrap();

        // We should really only ever have maps with finite layer-data
        // See [tiled::LayerData](https://docs.rs/tiled/latest/tiled/enum.LayerData.html)
        // for more info on what this is
        let layer_data = match &tilemap.layers[0].tiles {
            tiled::LayerData::Finite(tiles) => tiles,
            _ => {
                panic!("Ooops");
            }
        };

        let (sprite_width, sprite_height) = self.sprite_dimensions;

        // .tmx files store coordinates in (y, x) where each starts at
        // the top left corner and moves down and to the right.
        for j in 0..(WINDOW_HEIGHT / sprite_height) {
            for i in 0..(WINDOW_WIDTH / sprite_width) {
                // In an ideal world, the "group-id" of each tile
                // in the layer data corresponds to a sprite in this
                // sprite-sheet. However, there is some nuance that
                // gets introduced when you have two tilesets being
                // used to produce one tilemap, and thus we actually
                // have to find the first gid in the tileset and
                // subtract that gid from each gid in the layer data
                // before we know we have a sprite in our sheet.
                let gid = layer_data[j as usize][i as usize].gid;
                let tileset = tilemap.get_tileset_by_gid(gid).unwrap();

                self.draw_to(
                    (gid - tileset.first_gid) as usize,
                    (i * sprite_width) as i32,
                    (j * sprite_height) as i32,
                    canvas,
                );
            }
        }
    }
}

/// A container for sets of sprite-sheets corresponding to
/// animations that have directional variants
pub struct DirectionalAnimation<'a>(pub Vec<SpriteSheet<'a>>);

impl<'a> DirectionalAnimation<'a> {
    /// Attempts to produce a set of sprite-sheets
    /// corresponding to the directional variants
    /// of a certain animation. This function assumes
    /// your assets directory eventually looks something
    /// like this:
    ///
    /// ```text
    /// ├── Down
    /// │   └── sprite_name.png
    /// ├── Left
    /// │   └── sprite_name.png
    /// ├── Right
    /// │   └── sprite_name.png
    /// └── Up
    ///     └── sprite_name.png
    /// ```
    ///
    /// The order of the directories does not matter,
    /// but the four of them must exist and must
    /// each contain a png file whose name
    /// corresponds to the parameter `sprite_name`.
    ///
    /// The caller is also expected to provide a
    /// list of dimensions corresponding to the
    /// size of the up, down, left, and right variants
    /// of the animation respectively.
    ///
    /// Calling this function manually can be messy. It
    /// is instead best left to a caller who has access
    /// to [crate::util::DirectionalSpriteInfo] from a [crate::util::SpriteSheetInfo]
    /// struct.
    fn new(
        sprite_dir: &str,
        sprite_dimensions: Vec<(u32, u32)>,
        sprite_name: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, std::io::Error> {
        let mut up: Option<SpriteSheet> = None;
        let mut down: Option<SpriteSheet> = None;
        let mut left: Option<SpriteSheet> = None;
        let mut right: Option<SpriteSheet> = None;

        let sprite_dir = std::path::Path::new(sprite_dir);
        for entry in std::fs::read_dir(sprite_dir)? {
            let path = entry.unwrap().path();
            if path.is_dir() {
                let direction = path.file_stem().unwrap().to_str().unwrap();
                let sprite_path = match direction {
                    "Up" | "Down" | "Left" | "Right" => {
                        format!("{}/{}.png", path.to_str().unwrap(), sprite_name)
                    }
                    _ => {
                        continue;
                    }
                };

                match direction {
                    "Up" => {
                        up = Some(SpriteSheet::new(
                            sprite_dimensions[Direction::Up as usize],
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Down" => {
                        down = Some(SpriteSheet::new(
                            sprite_dimensions[Direction::Down as usize],
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Left" => {
                        left = Some(SpriteSheet::new(
                            sprite_dimensions[Direction::Left as usize],
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Right" => {
                        right = Some(SpriteSheet::new(
                            sprite_dimensions[Direction::Right as usize],
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        use std::io::{Error, ErrorKind};
        match (up, down, left, right) {
            (Some(up), Some(down), Some(left), Some(right)) => {
                let sprites = vec![up, down, left, right];
                Ok(DirectionalAnimation(sprites))
            }
            _ => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Unable to find directional directories in dir {}",
                    sprite_dir.to_str().unwrap()
                ),
            )),
        }
    }
}

/// A developer-facing resource for storing sprites.
/// Try to load this up with all your sprites as soon
/// as possible to avoid disk loading overhead later on.
pub struct SpriteHolder<'a> {
    sprite_map: HashMap<String, SpriteSheet<'a>>,
    directional_sprite_map: HashMap<String, Vec<DirectionalAnimation<'a>>>,
}

impl<'a> SpriteHolder<'a> {
    /// A convenience function for generating sprite-sheets corresponding
    /// to animations with directional variants and adding them to the
    /// internal `directional_sprite_map`
    fn generate_directional_sprites(
        texture_creator: &'a TextureCreator<WindowContext>,
        dsi: DirectionalSpriteInfo,
        directional_sprite_map: &mut HashMap<String, Vec<DirectionalAnimation<'a>>>,
    ) {
        let mut sprites = Vec::with_capacity(dsi.sprite_dimensions.len());

        for (name, wh) in dsi.sprite_dimensions.into_iter() {
            sprites.push(DirectionalAnimation::new(&dsi.path, wh, &name, texture_creator).unwrap());
        }

        directional_sprite_map.insert(dsi.name, sprites);
    }

    /// Returns an immutable reference to a sprite with the given name.
    ///
    /// # Panics
    ///
    /// Panics if a sprite with the given name doesn't exist in the `sprite_map`.
    pub fn get_sprite(&self, name: &str) -> &SpriteSheet<'a> {
        self.sprite_map.get(name).unwrap()
    }

    /// Returns a mutable reference to the given directional variant
    /// of the sprite with the given name.
    ///
    /// # Panics
    ///
    /// Panics if a sprite with the given name doesn't exist in the `directional_sprite_map`.
    pub fn get_directional(
        &mut self,
        name: &str,
        modifier: usize,
        direction: Direction,
    ) -> &mut SpriteSheet<'a> {
        &mut self.directional_sprite_map.get_mut(name).unwrap()[modifier].0[direction as usize]
    }

    /// Convenience function for producing a [SpriteHolder] from [crate::util::SpriteInfo].
    /// This is the preferred way to obtain a [SpriteHolder].
    pub fn from(
        texture_creator: &'a TextureCreator<WindowContext>,
        sprite_info: SpriteInfo,
    ) -> Self {
        let mut sprite_map = HashMap::new();
        for ssi in sprite_info.sprites.into_iter() {
            sprite_map.insert(
                ssi.name,
                SpriteSheet::new(ssi.sprite_dimensions, texture_creator, &ssi.path),
            );
        }

        let mut directional_sprite_map = HashMap::new();
        for dsi in sprite_info.directional_sprites.into_iter() {
            Self::generate_directional_sprites(texture_creator, dsi, &mut directional_sprite_map);
        }

        Self {
            sprite_map,
            directional_sprite_map,
        }
    }
}
