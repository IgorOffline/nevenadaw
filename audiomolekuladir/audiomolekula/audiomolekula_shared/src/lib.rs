use bevy::prelude::Resource;
use libloading::Library;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Resource)]
pub struct AudioState {
    _stream: cpal::Stream,
    _library: Library,
    is_pressed: Arc<AtomicBool>,
}

pub struct PluginGuiRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

impl AudioState {
    pub fn new(library: Library, stream: cpal::Stream, is_pressed: Arc<AtomicBool>) -> Self {
        Self {
            _stream: stream,
            _library: library,
            is_pressed,
        }
    }

    pub fn set_pressed(&self, pressed: bool) {
        self.is_pressed.store(pressed, Ordering::Relaxed);
    }

    pub fn set_parent_window(&self, _hwnd: isize) {
        // Implementation will go here
    }

    pub fn update_gui_layout(&self, _rect: PluginGuiRect) {
        // Implementation will go here
    }
}
