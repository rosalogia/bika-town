use sdl2::image::*;
use sdl2::render::*;

pub static WINDOW_WIDTH: u32 = 1008;
pub static WINDOW_HEIGHT: u32 = 1008;

pub struct DirectionalAnimation {
    pub up: SpriteSheet,
    pub down: SpriteSheet,
    pub left: SpriteSheet,
    pub right: SpriteSheet,
}

impl DirectionalAnimation {
    pub fn new<'a, T>(
        sprite_dir: &str,
        (lr_width, lr_height): (u32, u32),
        (ud_width, ud_height): (u32, u32),
        sprite_name: &str,
        texture_creator: &'a TextureCreator<T>,
        textures: &mut Vec<Texture<'a>>,
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
                            ud_width,
                            ud_height,
                            textures,
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Down" => {
                        down = Some(SpriteSheet::new(
                            ud_width,
                            ud_height,
                            textures,
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Left" => {
                        left = Some(SpriteSheet::new(
                            lr_width,
                            lr_height,
                            textures,
                            texture_creator,
                            &sprite_path,
                        ))
                    }
                    "Right" => {
                        right = Some(SpriteSheet::new(
                            lr_width,
                            lr_height,
                            textures,
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
            (Some(up), Some(down), Some(left), Some(right)) => Ok(DirectionalAnimation {
                up,
                down,
                left,
                right,
            }),
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

pub struct SpriteSheet {
    sprite_width: u32,
    sprite_height: u32,
    sprites: Vec<sdl2::rect::Rect>,
    pub texture_id: usize,
    path: String,
    animation_counter: u32,
}

impl SpriteSheet {
    pub fn new<'a, T>(
        sprite_width: u32,
        sprite_height: u32,
        textures: &mut Vec<Texture<'a>>,
        texture_creator: &'a TextureCreator<T>,
        path: &str,
    ) -> Self {
        let texture = texture_creator.load_texture(path).unwrap();
        let (max_width, max_height) = {
            let tq = texture.query();
            (tq.width, tq.height)
        };

        textures.push(texture);
        let texture_id = textures.len() - 1;

        let (h_size, v_size) = ((max_width / sprite_width), (max_height / sprite_height));
        let mut sprites: Vec<sdl2::rect::Rect> = Vec::with_capacity((h_size * v_size) as usize);

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

        let path = String::from(path);

        Self {
            sprite_width,
            sprite_height,
            sprites,
            texture_id,
            path,
            animation_counter: 0,
        }
    }

    pub fn draw_to(
        &self,
        i: usize,
        x: i32,
        y: i32,
        textures: &Vec<Texture>,
        canvas: &mut WindowCanvas,
    ) {
        let texture = &textures[self.texture_id];
        let sprite = self.sprites[i];
        let dst = sdl2::rect::Rect::new(x, y, self.sprite_width, self.sprite_height);
        canvas.copy(&texture, sprite, dst).unwrap();
    }

    pub fn draw_animated(
        &mut self,
        x: i32,
        y: i32,
        textures: &Vec<Texture>,
        canvas: &mut WindowCanvas,
    ) {
        self.draw_to(
            self.animation_counter as usize % self.sprites.len(),
            x,
            y,
            textures,
            canvas,
        );

        self.animation_counter += 1;
    }

    pub fn draw_map(&self, textures: &Vec<Texture>, canvas: &mut WindowCanvas, tilemap_path: &str) {
        let tilemap = tiled::parse_file(std::path::Path::new(tilemap_path)).unwrap();
        let layer_data = match &tilemap.layers[0].tiles {
            tiled::LayerData::Finite(tiles) => tiles,
            _ => {
                panic!("Ooops");
            }
        };

        for j in 0..(WINDOW_HEIGHT / 16) {
            for i in 0..(WINDOW_WIDTH / 16) {
                let gid = layer_data[j as usize][i as usize].gid;
                let tileset = tilemap.get_tileset_by_gid(gid).unwrap();

                self.draw_to(
                    (gid - tileset.first_gid) as usize,
                    (i * 16) as i32,
                    (j * 16) as i32,
                    textures,
                    canvas,
                );
            }
        }
    }
}
