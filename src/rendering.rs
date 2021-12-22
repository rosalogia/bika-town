use crate::models::components::Direction;
use crate::util::GlobalState;
use sdl2::image::*;

pub static WINDOW_WIDTH: u32 = 1008;
pub static WINDOW_HEIGHT: u32 = 1008;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectionalAnimation(Vec<SpriteSheet>);

impl DirectionalAnimation {
    pub fn get_sprite(&mut self, direction: Direction) -> &mut SpriteSheet {
        &mut self.0[direction as usize]
    }
    pub fn new<'a>(
        sprite_dir: &str,
        wh_list: Vec<(u32, u32)>,
        sprite_name: &str,
        global_state: &'a mut GlobalState<'a>,
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
                            wh_list[Direction::Up as usize],
                            None,
                            global_state,
                            &sprite_path,
                        ))
                    }
                    "Down" => {
                        down = Some(SpriteSheet::new(
                            wh_list[Direction::Down as usize],
                            None,
                            global_state,
                            &sprite_path,
                        ))
                    }
                    "Left" => {
                        left = Some(SpriteSheet::new(
                            wh_list[Direction::Left as usize],
                            None,
                            global_state,
                            &sprite_path,
                        ))
                    }
                    "Right" => {
                        right = Some(SpriteSheet::new(
                            wh_list[Direction::Right as usize],
                            None,
                            global_state,
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

#[derive(Clone, Debug, PartialEq)]
pub struct SpriteSheet {
    sprite_width: u32,
    sprite_height: u32,
    sprites: Vec<sdl2::rect::Rect>,
    pub texture_id: String,
    animation_counter: u32,
}

impl SpriteSheet {
    pub fn new<'a>(
        sprite_wh: (u32, u32),
        texture_id: Option<&str>,
        global_state: &'a mut GlobalState<'a>,
        path: &str,
    ) -> Self {
        let texture = global_state.texture_creator.load_texture(path).unwrap();
        let (max_width, max_height) = {
            let tq = texture.query();
            (tq.width, tq.height)
        };

        let (sprite_width, sprite_height) = sprite_wh;

        let texture_id_key = String::from(match texture_id {
            Some(id) => id,
            None => path,
        });

        let texture_id = texture_id_key.clone();
        global_state.texture_map.insert(texture_id_key, texture);

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

        Self {
            sprite_width,
            sprite_height,
            sprites,
            texture_id,
            animation_counter: 0,
        }
    }

    pub fn draw_to(&self, i: usize, x: i32, y: i32, global_state: &mut GlobalState) {
        let texture = global_state.texture_map.get(&self.texture_id).unwrap();
        let sprite = self.sprites[i];
        let dst = sdl2::rect::Rect::new(x, y, self.sprite_width, self.sprite_height);
        global_state.canvas.copy(&texture, sprite, dst).unwrap();
    }

    pub fn draw_animated(&mut self, x: i32, y: i32, global_state: &mut GlobalState) {
        self.draw_to(
            self.animation_counter as usize % self.sprites.len(),
            x,
            y,
            global_state,
        );

        self.animation_counter += 1;
    }

    pub fn draw_map(&self, global_state: &mut GlobalState, tilemap_path: &str) {
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
                    global_state,
                );
            }
        }
    }
}
