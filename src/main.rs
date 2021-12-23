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
    let mut last_id = 0;
    let mut resources = Resources::default();
    resources.insert::<Option<components::Input>>(None);

    // resources.insert(global_state);

    let tile_sheet = SpriteSheet::new(
        (16, 16),
        Some("tiles"),
        &texture_creator,
        &mut texture_map,
        "Assets/Proprietary/Tiles/Tiles.png",
    );

    let warrior_ui = SpriteSheet::new(
        (103, 46),
        Some("UI"),
        &texture_creator,
        &mut texture_map,
        "Assets/Proprietary/UI/Detailed_option/Detailed_option_Warrior.png",
    );

    let health_bar = SpriteSheet::new(
        (41, 6),
        Some("Health Bar"),
        &texture_creator,
        &mut texture_map,
        "Assets/Proprietary/UI/Detailed_option/Health_bar.png",
    );

    let magic_bar = SpriteSheet::new(
        (40, 6),
        Some("Magic Bar"),
        &texture_creator,
        &mut texture_map,
        "Assets/Proprietary/UI/Detailed_option/Magic_bar.png",
    );

    let experience_bar = SpriteSheet::new(
        (41, 6),
        Some("Experience Bar"),
        &texture_creator,
        &mut texture_map,
        "Assets/Proprietary/UI/Detailed_option/Experience_bar.png",
    );

    let _player = player::new(
        &mut world,
        &texture_creator,
        &mut texture_map,
        &mut directional_sprite_map,
        &mut last_id,
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

    let mut schedule = Schedule::builder()
        .add_system(player::systems::move_player_character_system())
        .add_system(player::systems::animate_player_system())
        .build();

    let mut then: std::time::Instant;
    let mut now: std::time::Instant;

    'running: loop {
        canvas.set_draw_color(Color::RGB(105, 6, 255));
        canvas.clear();

        then = std::time::Instant::now();

        // Load the tilemap file and draw it onto the canvas
        tile_sheet.draw_map(&mut canvas, &texture_map, "Assets/map.tmx");

        // Render the player's current animation frame
        // player.render_frame(&mut canvas, &texture_map);

        let mut event_pump = sdl_ctx.event_pump().unwrap();

        // match player.control(&mut event_pump) {
        //     // Hand off event handling to the player
        //     // until they're done moving
        //     Ok(_) => (),
        //     Err(e) => println!("{}", e),
        // }

        use models::components::*;
        use sdl2::keyboard::Scancode;

        let kb_state = event_pump.keyboard_state();

        if kb_state.is_scancode_pressed(Scancode::Up) || kb_state.is_scancode_pressed(Scancode::W) {
            resources.insert(Some(Input::Move(Direction::Up)));
        }

        if kb_state.is_scancode_pressed(Scancode::Down) || kb_state.is_scancode_pressed(Scancode::S)
        {
            resources.insert(Some(Input::Move(Direction::Down)));
        }

        if kb_state.is_scancode_pressed(Scancode::Left) || kb_state.is_scancode_pressed(Scancode::A)
        {
            resources.insert(Some(Input::Move(Direction::Left)));
        }

        if kb_state.is_scancode_pressed(Scancode::Right)
            || kb_state.is_scancode_pressed(Scancode::D)
        {
            resources.insert(Some(Input::Move(Direction::Right)));
        }

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

        schedule.execute(&mut world, &mut resources);
        resources.insert::<Option<Input>>(None);

        render_queue_items(&mut canvas, &texture_map, &mut directional_sprite_map);
        
        warrior_ui.draw_to(0, 0, 0, &mut canvas, &texture_map);
        health_bar.draw_to(0, 49, 5, &mut canvas, &texture_map);
        magic_bar.draw_to(0, 61, 20, &mut canvas, &texture_map);
        experience_bar.draw_to(0, 49, 35, &mut canvas, &texture_map);


        canvas.present();
        now = std::time::Instant::now();
        if now - then < std::time::Duration::new(0, 1_000_000_000 / 20) {
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 20) - (now - then));
        }
    }
}
