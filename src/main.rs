use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
}

impl Player {
    const FRAMES: i32 = 4;

    const ANIMATION_TIME: i32 = 120;

    fn update(&mut self, ticks: Duration) {
        println!("Ticks: {:?}", ticks);
        let col: i32 = (ticks.as_millis() as i32 / Player::ANIMATION_TIME % Player::FRAMES) * 64;
        self.sprite = Rect::new(col, 0, 64, 64);
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );
    canvas.copy(texture, player.sprite, screen_rect)?;
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not build video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not build canvas 2");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/jeff-sprite.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 64, 64),
    };

    let start = Instant::now();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        player.update(start.elapsed());

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time Mgmt
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
