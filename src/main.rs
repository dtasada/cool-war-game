use sdl2::{event::Event, keyboard::Keycode};

fn start_game() {
    println!("starting game")
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 example", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    timestamp,
                    ..
                } => match keycode {
                    Keycode::Space => start_game(),
                    Keycode::T => println!("timestamp: {}", timestamp),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
