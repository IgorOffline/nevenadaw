use audiomolekula_shared::{AudioState, PluginGuiRect};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, RawHandleWrapper};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use raw_window_handle::RawWindowHandle;

pub fn frontend_show_window() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AudioMolekula Host".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_resource::<PluginWindowState>()
        .add_systems(Startup, (setup_camera_system, setup_audio_system))
        .add_systems(
            Update,
            (capture_parent_window_handle_system, sync_plugin_gui_system),
        )
        .add_systems(Update, ui_main_layout_system)
        .run();
}

#[derive(Resource, Default)]
struct PluginWindowState {
    parent_hwnd: Option<isize>,
    last_known_rect: Option<egui::Rect>,
    is_visible: bool,
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
    mut state: ResMut<PluginWindowState>,
    audio_state: Option<Res<AudioState>>,
) {
    if state.parent_hwnd.is_some() {
        return;
    }

    if let Ok(handle_wrapper) = query.single() {
        if let RawWindowHandle::Win32(handle) = handle_wrapper.0 {
            let hwnd = handle.hwnd.get();
            state.parent_hwnd = Some(hwnd);

            if let Some(audio) = audio_state {
                audio.set_parent_window(hwnd);
            }
        }
    }
}

fn ui_main_layout_system(
    mut contexts: EguiContexts,
    audio_state: Option<Res<AudioState>>,
    mut state: ResMut<PluginWindowState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("controls").show(ctx, |ui| {
        ui.heading("Instrument");

        if let Some(audio) = &audio_state {
            let btn = ui.add_sized([100.0, 40.0], egui::Button::new("Play Note"));
            audio.set_pressed(btn.is_pointer_button_down_on());
        }

        ui.separator();
        ui.checkbox(&mut state.is_visible, "Show Vital Editor");
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Plugin Rack");

        let rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(rect, egui::Sense::hover());

        state.last_known_rect = Some(response.rect);

        ui.painter()
            .rect_stroke(response.rect, 0.0, (1.0, egui::Color32::DARK_GRAY));
    });
}

fn sync_plugin_gui_system(
    state: Res<PluginWindowState>,
    audio_state: Option<Res<AudioState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (Some(audio), Some(rect)) = (audio_state, state.last_known_rect) else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    let scale = window.scale_factor();

    let physical_rect = PluginGuiRect {
        x: (rect.min.x * scale) as i32,
        y: (rect.min.y * scale) as i32,
        width: (rect.width() * scale) as u32,
        height: (rect.height() * scale) as u32,
        visible: state.is_visible,
    };

    audio.update_gui_layout(physical_rect);
}
