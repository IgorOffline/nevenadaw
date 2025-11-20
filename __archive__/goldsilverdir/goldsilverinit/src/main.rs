use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::Sdl;
use serde::Deserialize;
use std::cell::Cell;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const SCREEN_WIDTH: u32 = 426;
const SCREEN_HEIGHT: u32 = 240;
const SDL_DELAY: u32 = 32;

#[derive(Clone, Copy, Debug, Deserialize)]
struct Regina {
    size: u32,
    emperor: u32,
    general: u32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Config {
    regina: Regina,
}

fn main() -> Result<(), String> {
    let regina_default = Regina {
        size: 25,
        emperor: 2,
        general: 1,
    };
    let regina = Cell::new(regina_default.clone());

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Gold Silver Init", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(
        Path::new(".")
            .join("assets")
            .join("fonts")
            .join("IosevkaTerm-Regular.ttf"),
        8,
    )?;

    gold_silver_running_loop(&sdl_context, &mut canvas, &font, &regina_default, &regina)?;

    Ok(())
}

fn gold_silver_running_loop(
    sdl_context: &Sdl,
    canvas: &mut WindowCanvas,
    font: &Font,
    regina_default: &Regina,
    regina: &Cell<Regina>,
) -> Result<(), String> {
    'running: loop {
        if gold_silver_running_inner_loop(sdl_context, regina_default, regina)? {
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(0x21, 0x21, 0x21));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0x75, 0x75, 0x75));

        let current_size = regina.clone().get().size;
        canvas
            .fill_rect(Rect::new(50, 50, current_size, current_size))
            .map_err(|e| e.to_string())?;

        let text_to_render = get_text_to_render(regina);
        let surface = font
            .render(&text_to_render)
            .blended(Color::RGB(69, 90, 100))
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(15, 15, width, height);
        canvas.copy(&texture, None, Some(target))?;

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(SDL_DELAY as u64));
    }

    Ok(())
}

fn gold_silver_running_inner_loop(
    sdl_context: &Sdl,
    regina_default: &Regina,
    regina: &Cell<Regina>,
) -> Result<bool, String> {
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
                match gold_silver_load_toml() {
                    Ok(regina_loaded_from_toml) => {
                        regina.set(regina_loaded_from_toml.clone());
                    }
                    Err(e) => println!("Error loading config: {}", e),
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                println!("Resetting Regina...");
                regina.set(regina_default.clone());
            }
            _ => { /* ignore other events */ }
        }
    }

    Ok(false)
}

fn gold_silver_load_toml() -> Result<Regina, String> {
    let mut content_raw = String::new();
    File::open("sdl2.toml")
        .map_err(|e| format!("Failed to open config file: {}", e))?
        .read_to_string(&mut content_raw)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config =
        toml::from_str(&content_raw).map_err(|e| format!("Failed to parse TOML: {}", e))?;

    let return_value = Regina {
        size: config.regina.size,
        emperor: config.regina.emperor,
        general: config.regina.general,
    };

    Ok(return_value)
}

fn get_text_to_render(regina: &Cell<Regina>) -> String {
    let regina_clone = regina.clone().get();
    format!(
        "Regina[size={}, emperor={}, general={}]",
        regina_clone.size, regina_clone.emperor, regina_clone.general
    )
}
