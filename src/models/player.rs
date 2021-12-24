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
    let position = Position {
        x,
        y,
        velocity: 1,
        direction: Direction::Down,
    };
    let state = PlayerState::Idle;

    let stats = PlayerStats::default();

    *last_id += 1;
    let id = *last_id;
    let sprites = generate_sprites(
        id,
        texture_creator,
        texture_map,
        directional_sprite_map,
        sprite_dir,
        wh_list,
    );

    world.push((Id(id), IsPlayerCharacter, stats, position, state, sprites))
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

fn move_to(position: &mut Position, state: &mut PlayerState, x: i32, y: i32) {
    if x < 0
        || x >= (WINDOW_WIDTH / 16 * 63) as i32
        || y < 0
        || y >= (WINDOW_HEIGHT / 16 * 63) as i32
    {
        return println!("Tried to move off the board");
    }

    *state = PlayerState::Moving;

    let x_movement = (position.x as i32 - x as i32).abs();
    let y_movement = (position.y as i32 - y as i32).abs();

    if x_movement != 0 && y_movement != 0 {
        return println!(
            "Invalid move attempted from ({}, {}) to ({}, {})",
            position.x, position.y, x, y
        );
    }

    position.x = x;
    position.y = y;
}

fn handle_movement_input(direction: &Direction, position: &mut Position, state: &mut PlayerState) {
    let move_by = position.velocity * 4;

    match direction {
        Direction::Up => {
            move_to(position, state, position.x, position.y - move_by);
        }
        Direction::Down => {
            move_to(position, state, position.x, position.y + move_by);
        }
        Direction::Left => {
            move_to(position, state, position.x - move_by, position.y);
        }
        Direction::Right => {
            move_to(position, state, position.x + move_by, position.y);
        }
    }
    position.direction = *direction;
}

pub mod systems {
    use super::*;

    #[system(for_each)]
    pub fn animate_player(
        position: &Position,
        state: &PlayerState,
        id: &Id,
        #[resource] render_queue: &mut Vec<RenderRequest>,
    ) {
        let Id(id) = *id;
        let position = *position;
        let state = *state;
        println!("Rendering player in state {:?}", state);

        let render_request = RenderRequest::Player {
            id,
            position,
            state,
        };

        render_queue.push(render_request);
    }

    #[system(for_each)]
    pub fn draw_player_ui(
        _: &IsPlayerCharacter,
        stats: &PlayerStats,
        #[resource] render_queue: &mut Vec<RenderRequest>,
    ) {
        println!(
            "Current: {}, Max: {}",
            stats.health.current, stats.health.max
        );
        let render_request = RenderRequest::PlayerUi(*stats);
        render_queue.push(render_request);
    }

    #[system(for_each)]
    pub fn player_input(
        _: &IsPlayerCharacter,
        position: &mut Position,
        state: &mut PlayerState,
        #[resource] input: &mut Vec<Input>,
    ) {
        if input.len() == 0 {
            *state = PlayerState::Idle;
            position.velocity = 1;
        } else {
            while let Some(input) = input.pop() {
                match input {
                    Input::Move(direction) => {
                        handle_movement_input(&direction, position, state);
                    }
                    Input::Run => {
                        position.velocity = 2;
                    }
                    Input::Attack => {
                        *state = PlayerState::Attack;
                    }
                    _ => {}
                }
            }
        }
    }
}
