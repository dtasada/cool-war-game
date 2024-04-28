use sdl2::{event::Event, keyboard::Keycode, rect::Rect, VideoSubsystem};

mod engine;
use engine::*;

fn main() -> Result<(), String> {
    let context: sdl2::Sdl = sdl2::init().unwrap();
    let subsystem: VideoSubsystem = context.video().unwrap();
    let mut events = context.event_pump()?;

    let mut game = Game::new();

    let window = subsystem
        .window("cool war game", 1280, 720)
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .target_texture()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let mut map = Sprite::new(
        &texture_creator,
        0,
        0,
        canvas.output_size().unwrap().0,
        canvas.output_size().unwrap().1,
        "assets/map.jpg",
    );

    while game.running {
        for key in events.keyboard_state().pressed_scancodes() {
            match Keycode::from_scancode(key).unwrap() {
                Keycode::W => map.rect.y -= 1,
                Keycode::A => map.rect.x -= 1,
                Keycode::S => map.rect.y += 1,
                Keycode::D => map.rect.x += 1,
                _ => {}
            }
        }

        let mouse = events.mouse_state();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => game.running = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    timestamp,
                    ..
                } => match keycode {
                    Keycode::Space => {}
                    Keycode::T => println!("timestamp: {}", timestamp),
                    _ => {}
                },
                Event::MouseWheel { y, .. } => {
                    let hori_ratio = mouse.x() as u32 / map.rect.width();
                    let vert_ratio = mouse.y() as u32 / map.rect.height();

                    if y > 0 {
                        map.rect.set_width((map.rect.width() as f32 * 1.5) as u32);
                        map.rect.set_height((map.rect.height() as f32 * 1.5) as u32);
                    } else {
                        map.rect.set_width((map.rect.width() as f32 / 1.5) as u32);
                        map.rect.set_height((map.rect.height() as f32 / 1.5) as u32);
                    }

                    map.rect.set_x((map.rect.width() * hori_ratio) as i32);
                    map.rect.set_y((map.rect.height() * vert_ratio) as i32);
                }
                _ => {}
            }
        }

        canvas.clear();

        map.render(&mut canvas);

        canvas.set_draw_color((255, 255, 255, 255));
        canvas.draw_rect(Rect::new(0, 0, 1280, 720))?;

        canvas.present();

        game.framerate.delay();
    }

    Ok(())
}
