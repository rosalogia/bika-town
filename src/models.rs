use crate::rendering::{DirectionalAnimation, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Copy)]
pub enum PlayerState {
    Moving = 0,
    Idle = 1,
    Attack = 2,
    Death = 3,
    TakingDamage = 4,
}

struct PlayerSprites {
    sprites: Vec<DirectionalAnimation>,
}

impl PlayerSprites {
    pub fn get_sprites(&mut self, state: PlayerState) -> &mut DirectionalAnimation {
        &mut self.sprites[state as usize]
    }

    pub fn new<'a, T>(
        texture_map: &mut HashMap<String, Texture<'a>>,
        texture_creator: &'a TextureCreator<T>,
        sprite_dir: &str,
        wh_list: Vec<Vec<(u32, u32)>>,
    ) -> Self {
        let names = vec!["Movement", "Idle", "Attack", "Death", "Taking damage"];

        let mut sprites = Vec::with_capacity(5);

        for (name, wh) in names.iter().zip(wh_list.into_iter()) {
            sprites.push(
                DirectionalAnimation::new(sprite_dir, wh, name, texture_creator, texture_map)
                    .unwrap(),
            );
        }

        Self { sprites }
    }
}

pub struct Player {
    x: i32,
    y: i32,
    movement_speed: i32,
    pub direction: Direction,
    sprites: PlayerSprites,
    pub state: PlayerState,
}

impl Player {
    pub fn new<'a, T>(
        x: i32,
        y: i32,
        texture_map: &mut HashMap<String, Texture<'a>>,
        texture_creator: &'a TextureCreator<T>,
        sprite_dir: &str,
        wh_list: Vec<Vec<(u32, u32)>>,
    ) -> Self {
        let sprites = PlayerSprites::new(texture_map, texture_creator, sprite_dir, wh_list);
        let direction = Direction::Down;
        let state = PlayerState::Idle;

        Player {
            x,
            y,
            movement_speed: 1,
            sprites,
            direction,
            state,
        }
    }

    pub fn render_frame(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_map: &HashMap<String, Texture>,
    ) {
        self.sprites
            .get_sprites(self.state)
            .get_sprite(self.direction)
            .draw_animated(self.x, self.y, texture_map, canvas);
    }

    pub fn control(&mut self, event_pump: &sdl2::EventPump) -> Result<(), String> {
        use sdl2::keyboard::Scancode;

        let kb_state = event_pump.keyboard_state();

        self.movement_speed = if kb_state.is_scancode_pressed(Scancode::Space) {
            2
        } else {
            1
        };
        let move_by = self.movement_speed * 4;

        if kb_state.is_scancode_pressed(Scancode::Up) || kb_state.is_scancode_pressed(Scancode::W) {
            self.move_to(self.x, self.y - move_by)?;
            return Ok(self.direction = Direction::Up);
        }

        if kb_state.is_scancode_pressed(Scancode::Down) || kb_state.is_scancode_pressed(Scancode::S)
        {
            self.move_to(self.x, self.y + move_by)?;
            return Ok(self.direction = Direction::Down);
        }

        if kb_state.is_scancode_pressed(Scancode::Left) || kb_state.is_scancode_pressed(Scancode::A)
        {
            self.move_to(self.x - move_by, self.y)?;
            return Ok(self.direction = Direction::Left);
        }

        if kb_state.is_scancode_pressed(Scancode::Right)
            || kb_state.is_scancode_pressed(Scancode::D)
        {
            self.move_to(self.x + move_by, self.y)?;
            return Ok(self.direction = Direction::Right);
        }

        if kb_state.is_scancode_pressed(Scancode::Z) {
            return Ok(self.state = PlayerState::Attack);
        }

        self.movement_speed = 1;
        Ok(self.state = PlayerState::Idle)
    }

    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), String> {
        if x < 0
            || x >= (WINDOW_WIDTH / 16 * 63) as i32
            || y < 0
            || y >= (WINDOW_HEIGHT / 16 * 63) as i32
        {
            return Err("Tried to move off the board".to_string());
        }

        self.state = PlayerState::Moving;
        let x_movement = (self.x as i32 - x as i32).abs();
        let y_movement = (self.y as i32 - y as i32).abs();

        if x_movement != 0 && y_movement != 0 {
            return Err(format!(
                "Invalid move attempted from ({}, {}) to ({}, {})",
                self.x, self.y, x, y
            ));
        }

        self.x = x;
        self.y = y;

        Ok(())
    }
}
