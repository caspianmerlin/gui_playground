use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture};
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), String> {

    // Initialisation
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("GUI Playground", 1024, 768)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player {
        position: Point::new(0,0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 5,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;


    // Game loop
    'running: loop {

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.position = player.position.offset(-player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.position = player.position.offset(player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.position = player.position.offset(0, -player.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    player.position = player.position.offset(0, player.speed);
                },
                _ => {}
            }
        }
        
        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;
        
        // Time management
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}




fn render(canvas: &mut WindowCanvas, colour: Color, texture: &Texture, player: &Player) -> Result<(), String> {

    //Calculating position
    let (width, height) = canvas.output_size()?;
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    // Rendering
    canvas.set_draw_color(colour);
    canvas.clear();
    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.present();
    Ok(())
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
}