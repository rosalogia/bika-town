use sdl2::pixels::Color;
use sdl2::render::Texture;

mod models;
mod rendering;

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

    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = vec![];

    let tile_sheet = SpriteSheet::new(
        (16, 16),
        &mut textures,
        &texture_creator,
        "Assets/Tiles.png",
    );

    let mut player = Player::new(
        0,
        0,
        &mut textures,
        &texture_creator,
        "Assets/Warrior",
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

    'running: loop {
        canvas.set_draw_color(Color::RGB(105, 6, 255));
        canvas.clear();

        // Load the tilemap file and draw it onto the canvas
        tile_sheet.draw_map(&textures, &mut canvas, "Assets/map.tmx");

        // Render the player's current animation frame
        player.render_frame(&mut canvas, &textures);

        let mut event_pump = sdl_ctx.event_pump().unwrap();

        // Hand off event handling to the player
        // until they're done moving
        match player.control(&event_pump) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
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

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60));
    }
}
