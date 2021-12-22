use super::components::*;
use crate::rendering::DirectionalAnimation;
use crate::rendering::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::util::GlobalState;
use legion::*;

pub fn new<'a>(
    global_state: &'a mut GlobalState<'a>,
    x: i32,
    y: i32,
    sprite_dir: &str,
    wh_list: Vec<Vec<(u32, u32)>>,
) -> Entity {
    let velocity = 1;
    let direction = Direction::Down;
    let state = PlayerState::Idle;
    let position = Position {
        x,
        y,
        velocity,
        direction,
    };

    let id = global_state.last_id + 1;
    global_state.last_id += 1;
    let sprites = generate_sprites(global_state.last_id, global_state, sprite_dir, wh_list);

    global_state
        .world
        .push((Id(id), IsPlayerCharacter, position, state, sprites))
}

pub fn generate_sprites<'a>(
    id: u32,
    global_state: &'a mut GlobalState<'a>,
    sprite_dir: &str,
    wh_list: Vec<Vec<(u32, u32)>>,
) {
    let names = vec!["Movement", "Idle", "Attack", "Death", "Taking damage"];

    let mut sprites = Vec::with_capacity(5);

    for (name, wh) in names.iter().zip(wh_list.into_iter()) {
        sprites.push(DirectionalAnimation::new(sprite_dir, wh, name, global_state).unwrap());
    }

    global_state.directional_sprite_map.insert(id, sprites);
}

fn move_to(position: &mut Position, state: &mut PlayerState, x: i32, y: i32) -> Result<(), String> {
    if x < 0
        || x >= (WINDOW_WIDTH / 16 * 63) as i32
        || y < 0
        || y >= (WINDOW_HEIGHT / 16 * 63) as i32
    {
        return Err("Tried to move off the board".to_string());
    }

    *state = PlayerState::Moving;

    let x_movement = (position.x as i32 - x as i32).abs();
    let y_movement = (position.y as i32 - y as i32).abs();

    if x_movement != 0 && y_movement != 0 {
        return Err(format!(
            "Invalid move attempted from ({}, {}) to ({}, {})",
            position.x, position.y, x, y
        ));
    }

    position.x = x;
    position.y = y;

    Ok(())
}

pub mod systems {
    use super::*;
    #[system(for_each)]
    pub fn move_player_character(
        _: &IsPlayerCharacter,
        position: &mut Position,
        state: &mut PlayerState,
        #[resource] input: &Input,
    ) {
        let move_by = position.velocity * 4;

        let Input::Move(direction) = input;

        match direction {
            Direction::Up => {
                move_to(position, state, position.x, position.y - move_by).unwrap();
            }
            Direction::Down => {
                move_to(position, state, position.x, position.y + move_by).unwrap();
            }
            Direction::Left => {
                move_to(position, state, position.x - move_by, position.y).unwrap();
            }
            Direction::Right => {
                move_to(position, state, position.x + move_by, position.y).unwrap();
            }
        }

        position.direction = *direction;
    }

    #[system(for_each)]
    pub fn player_run(_: &IsPlayerCharacter, position: &mut Position, #[resource] input: &Input) {
        match input {
            Input::Run => {
                position.velocity = 2;
            }
            _ => {}
        }
    }
}
