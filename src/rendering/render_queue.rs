use super::*;
use crate::models::components::*;
use sdl2::render::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderRequest {
    Player {
        class: PlayerClass,
        gender: Gender,
        position: Position,
        state: PlayerState,
    },
}

pub type RenderQueue = Vec<RenderRequest>;

pub fn render_queue_items<'a>(
    canvas: &mut WindowCanvas,
    render_queue: &mut RenderQueue,
    sprite_holder: &mut SpriteHolder<'a>,
) {
    while let Some(render_request) = render_queue.pop() {
        match render_request {
            RenderRequest::Player {
                class,
                gender,
                position,
                state,
            } => {
                let gender_str = if gender == Gender::Male {
                    "male"
                } else {
                    "female"
                };

                let animation = match class {
                    PlayerClass::Warrior => sprite_holder.get_directional(
                        format!("warrior_{}", gender_str).as_str(),
                        state as usize,
                        position.direction,
                    ),
                    _ => {
                        panic!("Unimplemented");
                    }
                };

                animation.draw_animated(position.x, position.y, canvas);
            }
        }
    }
}
