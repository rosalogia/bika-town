use super::components::*;
use crate::rendering::{RenderRequest, WINDOW_HEIGHT, WINDOW_WIDTH};
use legion::*;

/// Produces a new player and pushes it to the world
pub fn new(
    world: &mut World,
    starting_position: (i32, i32),
    class: PlayerClass,
    gender: Gender,
) -> Entity {
    let (x, y) = starting_position;
    let position = Position {
        x,
        y,
        velocity: 1,
        direction: Direction::Down,
    };

    let state = PlayerState::Idle;
    let stats = PlayerStats::default();

    world.push((IsPlayerCharacter, class, gender, stats, position, state))
}

/// Attempts to move a player to (x, y) but will fail if the move is illegal.
/// The player's state is set to moving regardless of whether or not the move
/// actually executes.
fn move_to(position: &mut Position, state: &mut PlayerState, x: i32, y: i32) {
    *state = PlayerState::Moving;

    if x < 0
        || x >= (WINDOW_WIDTH / 16 * 63) as i32
        || y < 0
        || y >= (WINDOW_HEIGHT / 16 * 63) as i32
    {
        return println!("Tried to move off the board");
    }

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

/// A utility function for handling movement input.
/// This is just a messy match that I didn't want
/// cluttering the input system.
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

/// Legion systems that affect players
pub mod systems {
    use super::*;

    #[system(for_each)]
    pub fn animate_player(
        position: &Position,
        class: &PlayerClass,
        gender: &Gender,
        state: &PlayerState,
        #[resource] render_queue: &mut Vec<RenderRequest>,
    ) {
        let (class, gender, position, state) = (*class, *gender, *position, *state);

        let render_request = RenderRequest::Player {
            class,
            gender,
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
