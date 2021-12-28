use legion::*;
use sdl2::pixels::Color;

mod models;
mod rendering;
mod util;

use models::components::{Gender, InputQueue, PlayerClass};
use models::*;
use rendering::*;
use util::*;

fn main() {
    // SDL2 initialization
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

    let window = video
        .window("Bika Town", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // The canvas is what we draw on, we'll have to pass it
    // to any function that wants to blit a texture onto the
    // screen
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    // ECS set-up
    let mut world = World::default();
    let mut resources = Resources::default();
    // Create empty render and input queue resources
    resources.insert::<RenderQueue>(vec![]);
    resources.insert::<InputQueue>(vec![]);

    // Texture and SpriteSheet loading and creation
    let texture_creator = canvas.texture_creator();
    let sprite_info = SpriteInfo::from("Assets/SpriteData/sprites.ron").unwrap();
    let mut sprite_holder = SpriteHolder::from(&texture_creator, sprite_info);

    // Create the player character;
    // this function pushes the new
    // player to the world automatically
    let _player = player::new(&mut world, (0, 0), PlayerClass::Warrior, Gender::Based);

    let mut schedule = Schedule::builder()
        .add_system(player::systems::player_input_system())
        .add_system(player::systems::animate_player_system())
        .add_system(player::systems::draw_hud_system())
        .build();

    let mut then: std::time::Instant;
    let mut now: std::time::Instant;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        then = std::time::Instant::now();

        // Load the tilemap file and draw it onto the canvas
        sprite_holder
            .get_sprite("tiles")
            .draw_map(&mut canvas, "Assets/map.tmx");

        // Run all the systems
        schedule.execute(&mut world, &mut resources);

        // Handle quit events, then pass event pump
        // to an input handler function
        let mut event_pump = sdl_ctx.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Q),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        use core::ops::DerefMut;
        let mut input_vector_ref = resources.get_mut::<InputQueue>().unwrap();
        let mut input_vector = input_vector_ref.deref_mut();
        handle_input(&mut event_pump, &mut input_vector);

        // Retrieve and dereference the render queue
        // then render everything within it
        let mut render_queue_reference = resources.get_mut::<RenderQueue>().unwrap();
        let mut render_queue = render_queue_reference.deref_mut();
        render_queue_items(&mut canvas, &mut render_queue, &mut sprite_holder);

        // Draw to the screen
        canvas.present();

        // Ensure 20fps
        now = std::time::Instant::now();
        if now - then < std::time::Duration::new(0, 1_000_000_000 / 20) {
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 20) - (now - then));
        }
    }
}

// Yes this could be nicer
fn handle_input(event_pump: &mut sdl2::EventPump, input_vector: &mut InputQueue) {
    use models::components::*;
    use sdl2::keyboard::Scancode;

    let kb_state = event_pump.keyboard_state();

    if kb_state.is_scancode_pressed(Scancode::Up) || kb_state.is_scancode_pressed(Scancode::W) {
        input_vector.push(Input::Move(Direction::Up));
    } else if kb_state.is_scancode_pressed(Scancode::Down)
        || kb_state.is_scancode_pressed(Scancode::S)
    {
        input_vector.push(Input::Move(Direction::Down));
    } else if kb_state.is_scancode_pressed(Scancode::Left)
        || kb_state.is_scancode_pressed(Scancode::A)
    {
        input_vector.push(Input::Move(Direction::Left));
    } else if kb_state.is_scancode_pressed(Scancode::Right)
        || kb_state.is_scancode_pressed(Scancode::D)
    {
        input_vector.push(Input::Move(Direction::Right));
    } else if kb_state.is_scancode_pressed(Scancode::Z) {
        input_vector.push(Input::Attack);
    }

    if kb_state.is_scancode_pressed(Scancode::Space) {
        input_vector.push(Input::Run);
    }

    for event in event_pump.poll_iter() {
        match event {
            _ => {}
        }
    }
}
