use sdl2::pixels::Color;
use sdl2::render::Texture;
mod models;
mod rendering;
use rendering::SpriteSheet;
use rendering::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let mut timer = sdl_ctx.timer().unwrap();
    let video = sdl_ctx.video().unwrap();
    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    // let ttf_ctx = sdl2::ttf::init().unwrap();

    let window = video
        .window("Bika Town", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = vec![];
    let tile_sheet = SpriteSheet::new(16, 16, &mut textures, &texture_creator, "Assets/Tiles.png");
    'running: loop {
        // let ticks = timer.ticks();
        // let seconds = ticks / 100;
        // let sprite = seconds % 4;
        canvas.set_draw_color(Color::RGB(105, 6, 255));
        canvas.clear();
        tile_sheet.draw_map(
            &mut canvas,
            "Assets/map.tmx",
            &textures[tile_sheet.texture_id],
        );

        let mut event_pump = sdl_ctx.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Q),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.present();
        timer.delay(16);
    }
}
