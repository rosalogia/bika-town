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
    HUD(PlayerClass, Gender, PlayerStats),
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
                let animation = sprite_holder.get_directional(
                    format!("{}_{}", class.to_string(), gender.to_string()).as_str(),
                    state as usize,
                    position.direction,
                );

                animation.draw_animated(position.x, position.y, canvas);
            }
            RenderRequest::HUD(class, gender, stats) => {
                sprite_holder
                    .get_sprite(format!("{}_{}_ui", class.to_string(), gender.to_string()).as_str())
                    .draw_to(0, 0, 0, canvas);
                let (health_pct, mana_pct, exp_pct) = (
                    stats.health.as_percent(),
                    stats.mana.as_percent(),
                    stats.experience.as_percent(),
                );

                sprite_holder
                    .get_sprite("health_bar")
                    .draw_portion_of(0, 49, 5, health_pct, 1.0, canvas);

                sprite_holder
                    .get_sprite("magic_bar")
                    .draw_portion_of(0, 61, 20, mana_pct, 1.0, canvas);

                sprite_holder
                    .get_sprite("exp_bar")
                    .draw_portion_of(0, 49, 35, exp_pct, 1.0, canvas);
            }
        }
    }
}
