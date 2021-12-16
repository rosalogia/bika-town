use crate::rendering::{DirectionalAnimation, WINDOW_HEIGHT, WINDOW_WIDTH};
// use sdl2::render::*;

enum PlayerState {
    Attack,
    Death,
    Idle,
    Moving,
    TakingDamage,
}

struct PlayerSprites {
    attack: DirectionalAnimation,
    down: DirectionalAnimation,
    left: DirectionalAnimation,
    right: DirectionalAnimation,
}

pub struct Player {
    x: i8,
    y: i8,
    sprites: PlayerSprites,
    sprite_width: u32,
    sprite_height: u32,
}

impl Player {
    pub fn move_to(&mut self, x: i8, y: i8) -> Result<(), String> {
        if x < 0
            || x >= (WINDOW_WIDTH / self.sprite_width) as i8
            || y < 0
            || y >= (WINDOW_HEIGHT / self.sprite_height) as i8
        {
            return Err("Tried to move off the board".to_string());
        }

        let x_movement = (self.x as i32 - x as i32).abs();
        let y_movement = (self.y as i32 - y as i32).abs();

        if x_movement != 0 && y_movement != 0 || x_movement > 1 || y_movement > 1 {
            return Err(format!(
                "Invalid move attempted from ({}, {}) to ({}, {})",
                self.x, self.y, x, y
            ));
        }

        self.x = x;
        self.y = y;

        // if state.grid[x as usize][y as usize] == TileType::Tomato {
        //     state.grid[x as usize][y as usize] = TileType::Grass;
        //     state.score += 1;
        //     state.tomatoes -= 1;

        //     if state.tomatoes == 0 {
        //         state.level += 1;
        //         let rng = rand::thread_rng();
        //         init_grid(rng, state);
        //     }
        // }

        Ok(())
    }
}
