use legion::*;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use std::collections::HashMap;

mod models;
mod rendering;
mod util;

use models::*;
use rendering::*;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

    let window = video
        .window("Bika Town", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    // Texture and SpriteSheet loading and creation
    let mut world = World::default();
    let texture_creator = canvas.texture_creator();
    let mut directional_sprite_map: HashMap<u32, Vec<DirectionalAnimation>> = HashMap::new();
    let mut texture_map: HashMap<String, Texture> = HashMap::new();

    let mut global_state = util::GlobalState {
        last_id: 0,
        world,
        canvas,
        texture_creator,
        texture_map,
        directional_sprite_map,
    };

    let tile_sheet = SpriteSheet::new(
        (16, 16),
        Some("tiles"),
        &mut global_state,
        "Assets/Proprietary/Tiles/Tiles.png",
    );

    let warrior_ui = SpriteSheet::new(
        (103, 46),
        Some("UI"),
        &mut global_state,
        "Assets/Proprietary/UI/Detailed_option/Detailed_option_Warrior.png",
    );

    let health_bar = SpriteSheet::new(
        (41, 6),
        Some("Health Bar"),
        &mut global_state,
        "Assets/Proprietary/UI/Detailed_option/Health_bar.png",
    );

    let magic_bar = SpriteSheet::new(
        (40, 6),
        Some("Magic Bar"),
        &mut global_state,
        "Assets/Proprietary/UI/Detailed_option/Magic_bar.png",
    );

    let experience_bar = SpriteSheet::new(
        (41, 6),
        Some("Experience Bar"),
        &mut global_state,
        "Assets/Proprietary/UI/Detailed_option/Experience_bar.png",
    );

    let _player = player::new(
        &mut global_state,
        0,
        0,
        "Assets/Proprietary/Animation/Main_heroes/Warrior",
        vec![
            vec![(72 / 4, 28), (72 / 4, 28), (68 / 4, 27), (68 / 4, 27)],
            vec![(72 / 4, 27), (72 / 4, 27), (68 / 4, 27), (68 / 4, 27)],
            vec![(116 / 4, 29), (128 / 4, 36), (124 / 4, 31), (124 / 4, 31)],
            vec![
                (264 / 11, 28),
                (297 / 11, 49),
                (385 / 11, 35),
                (396 / 11, 35),
            ],
            vec![(36 / 2, 26), (36 / 2, 27), (34 / 2, 27), (34 / 2, 27)],
        ],
    );

    let mut then: std::time::Instant;
    let mut now: std::time::Instant;

    'running: loop {
        global_state.canvas.set_draw_color(Color::RGB(105, 6, 255));
        global_state.canvas.clear();

        then = std::time::Instant::now();

        // Load the tilemap file and draw it onto the canvas
        tile_sheet.draw_map(&mut global_state, "Assets/map.tmx");

        // Render the player's current animation frame
        // player.render_frame(&mut canvas, &texture_map);

        warrior_ui.draw_to(0, 0, 0, &mut global_state);
        health_bar.draw_to(0, 49, 5, &mut global_state);
        magic_bar.draw_to(0, 61, 20, &mut global_state);
        experience_bar.draw_to(0, 49, 35, &mut global_state);

        let mut event_pump = sdl_ctx.event_pump().unwrap();

        // match player.control(&mut event_pump) {
        //     // Hand off event handling to the player
        //     // until they're done moving
        //     Ok(_) => (),
        //     Err(e) => println!("{}", e),
        // }

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

        global_state.canvas.present();
        now = std::time::Instant::now();
        if now - then < std::time::Duration::new(0, 1_000_000_000 / 20) {
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 20) - (now - then));
        }
    }
}
