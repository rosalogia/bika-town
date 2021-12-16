use sdl2::pixels::Color;
use sdl2::render::Texture;
mod models;
mod rendering;
use rendering::*;

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
    canvas.set_scale(2.0, 2.0).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = vec![];
    let tile_sheet = SpriteSheet::new(16, 16, &mut textures, &texture_creator, "Assets/Tiles.png");
    let mut warrior_walk = DirectionalAnimation::new(
        "Assets/Warrior/",
        (17, 27),
        (18, 28),
        "Movement",
        &texture_creator,
        &mut textures,
    )
    .unwrap();

    'running: loop {
        // let ticks = timer.ticks();
        // let seconds = ticks / 100;
        // let sprite = seconds % 4;
        canvas.set_draw_color(Color::RGB(105, 6, 255));
        canvas.clear();

        tile_sheet.draw_map(&textures, &mut canvas, "Assets/map.tmx");
        warrior_walk
            .right
            .draw_animated(0, 0, &textures, &mut canvas);

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
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
