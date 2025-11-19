use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::f32::consts::PI;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;
const SDL_DELAY: u32 = 32;

fn main() -> Result<(), String> {
    let rust_flag = true;
    let circle_speed_modifier = if rust_flag { 4.4 } else { 2.5 };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Gold Silver Init", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut circle_x = SCREEN_WIDTH as f32 / 2.0;
    let mut circle_y = SCREEN_HEIGHT as f32 / 2.0;
    let mut circle_speed_x = 3.0 * circle_speed_modifier;
    let mut circle_speed_y = 2.0 * circle_speed_modifier;
    let circle_radius = 50.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        circle_x += circle_speed_x;
        circle_y += circle_speed_y;

        if circle_x - circle_radius < 0.0 || circle_x + circle_radius > SCREEN_WIDTH as f32 {
            circle_speed_x = -circle_speed_x;
        }
        if circle_y - circle_radius < 0.0 || circle_y + circle_radius > SCREEN_HEIGHT as f32 {
            circle_speed_y = -circle_speed_y;
        }

        canvas.set_draw_color(Color::RGB(0x21, 0x21, 0x21));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xBD, 0xBD, 0xBD));

        for angle in 0..360 {
            let rad = (angle as f32) * PI / 180.0;
            let px = circle_x + circle_radius * rad.cos();
            let py = circle_y + circle_radius * rad.sin();
            canvas.draw_point((px as i32, py as i32))?;
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(SDL_DELAY as u64));
    }

    Ok(())
}
