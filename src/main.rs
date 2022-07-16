use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::thread;
use std::time::Duration;

pub fn main() -> Result<(), String> {

    // Initialisation
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("GUI Playground", 1024, 768)
        .position_centered()
        .build()
        .expect("Unable to create window.");

    let mut canvas = window.into_canvas().build().expect("Unable to build canvas.");
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
                _ => {}
            }
        }
        
        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i));
        
        // Time management
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}




fn render(canvas: &mut WindowCanvas, colour: Color) {
    canvas.set_draw_color(colour);
    canvas.clear();
    canvas.present();
}