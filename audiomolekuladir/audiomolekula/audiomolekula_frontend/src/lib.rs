use audiomolekula_shared::AudioState;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, RawHandleWrapper};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use raw_window_handle::RawWindowHandle;

pub fn frontend_show_window() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640, 360).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .init_resource::<ParentWindowHandle>()
        .add_systems(Startup, (setup_camera_system, setup_audio_system))
        .add_systems(Update, capture_parent_window_handle_system)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

#[derive(Resource, Default)]
struct ParentWindowHandle {
    hwnd: Option<isize>,
}

fn setup_audio_system(mut commands: Commands) {
    if let Some(audio_state) = audiomolekula_audio::setup_audio_system() {
        commands.insert_resource(audio_state);
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn capture_parent_window_handle_system(
    query: Query<&RawHandleWrapper, With<PrimaryWindow>>,
    mut parent_handle: ResMut<ParentWindowHandle>,
) {
    if parent_handle.hwnd.is_some() {
        return;
    }

    let handle_wrapper = match query.single() {
        Ok(handle_wrapper) => handle_wrapper,
        Err(_) => return,
    };

    match handle_wrapper.0 {
        RawWindowHandle::Win32(handle) => {
            let hwnd = handle.hwnd.get();
            parent_handle.hwnd = Some(hwnd);
            println!("Primary window HWND: 0x{:x}", hwnd);
        }
        _ => {
            println!("Primary window handle is not Win32.");
            parent_handle.hwnd = Some(0);
        }
    }
}

fn ui_example_system(mut contexts: EguiContexts, audio_state: Option<Res<AudioState>>) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("(Soundhold)");
        let button_response = ui.button("Play Sound");
        if let Some(audio_state) = audio_state {
            if button_response.is_pointer_button_down_on() {
                audio_state.set_pressed(true);
            } else {
                audio_state.set_pressed(false);
            }
        }
    });

    Ok(())
}
