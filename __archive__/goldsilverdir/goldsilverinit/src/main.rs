use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

const SCREEN_WIDTH: u32 = 426;
const SCREEN_HEIGHT: u32 = 240;
const SDL_DELAY: u32 = 32;

#[derive(Deserialize, Debug)]
struct Root {
    speed: f32,
}

#[derive(Deserialize, Debug)]
struct Config {
    root: Root,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Gold Silver Init", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let running_loop = gold_silver_running_loop(&sdl_context, &mut canvas).unwrap_or_else(|_| {
        println!("ERR::100100");
        false
    });
    println!("{}", running_loop);

    Ok(())
}

fn gold_silver_running_loop(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<bool, String> {
    let mut running = true;

    'running: loop {
        gold_silver_running_inner_loop(&sdl_context).unwrap_or_else(|_| {
            running = false;
            running
        });

        if !running {
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(0x21, 0x21, 0x21));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xBD, 0xBD, 0xBD));

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(SDL_DELAY as u64));
    }

    Ok(true)
}

fn gold_silver_running_inner_loop(sdl_context: &Sdl) -> Result<bool, String> {
    let mut event_pump = sdl_context.event_pump()?;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Ok(true);
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                println!("Loading toml...");
                let new_speed = gold_silver_load_toml().unwrap_or_else(|_| {
                    println!("ERR::100200");
                    0.0
                });
                println!("{}", new_speed);
            }
            _ => { /* empty */ }
        }
    }

    Ok(false)
}

fn gold_silver_load_toml() -> Result<f32, String> {
    let mut content_raw = String::new();
    File::open("sdl2.toml")
        .unwrap()
        .read_to_string(&mut content_raw)
        .expect("I'm learning, give me a break!");
    let content = content_raw.clone();

    let config: Config = toml::from_str(&content).unwrap();
    Ok(config.root.speed)
}
