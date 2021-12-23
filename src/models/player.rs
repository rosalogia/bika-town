use super::components::*;
use crate::rendering::{DirectionalAnimation, RenderRequest, WINDOW_HEIGHT, WINDOW_WIDTH};
use legion::*;
use sdl2::render::{Texture, TextureCreator};
use std::collections::HashMap;

pub fn new<'a, T>(
    world: &mut World,
    texture_creator: &'a TextureCreator<T>,
    texture_map: &mut HashMap<String, Texture<'a>>,
    directional_sprite_map: &mut HashMap<u32, Vec<DirectionalAnimation>>,
    last_id: &mut u32,
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

    let id = *last_id + 1;
    *last_id += 1;
    let sprites = generate_sprites(
        id,
        texture_creator,
        texture_map,
        directional_sprite_map,
        sprite_dir,
        wh_list,
    );

    world.push((Id(id), IsPlayerCharacter, position, state, sprites))
}

pub fn generate_sprites<'a, T>(
    id: u32,
    texture_creator: &'a TextureCreator<T>,
    texture_map: &mut HashMap<String, Texture<'a>>,
    directional_sprite_map: &mut HashMap<u32, Vec<DirectionalAnimation>>,
    sprite_dir: &str,
    wh_list: Vec<Vec<(u32, u32)>>,
) {
    let names = vec!["Movement", "Idle", "Attack", "Death", "Taking damage"];

    let mut sprites = Vec::with_capacity(5);

    for (name, wh) in names.iter().zip(wh_list.into_iter()) {
        sprites.push(
            DirectionalAnimation::new(sprite_dir, wh, name, texture_creator, texture_map).unwrap(),
        );
    }

    directional_sprite_map.insert(id, sprites);
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
    pub fn animate_player(
        position: &Position,
        state: &PlayerState,
        id: &Id,
        #[resource] resource_queue: &mut Vec<RenderRequest>,
    ) {
        let Id(id) = *id;
        let position = *position;
        let state = *state;
        println!("Rendering player in state {:?}", state);

        let render_request = RenderRequest {
            id,
            position,
            state,
        };

        resource_queue.push(render_request);
    }

    #[system(for_each)]
    pub fn move_player_character(
        _: &IsPlayerCharacter,
        position: &mut Position,
        state: &mut PlayerState,
        #[resource] input: &Option<Input>,
    ) {
        let move_by = position.velocity * 4;

        if let Some(Input::Move(direction)) = input {
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
            position.velocity = 1;
            // *state = PlayerState::Idle;
        } else {
            *state = PlayerState::Idle;
        }
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
