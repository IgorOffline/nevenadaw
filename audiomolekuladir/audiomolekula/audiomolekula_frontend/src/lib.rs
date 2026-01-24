use audiomolekula_shared::{AudioState, PluginGuiRect};
use bevy::prelude::*;
use bevy::window::{ExitCondition, PrimaryWindow, RawHandleWrapper};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use raw_window_handle::RawWindowHandle;
use std::rc::Rc;

pub fn frontend_show_window() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AudioMolekula Host".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            exit_condition: ExitCondition::DontExit,
            close_when_requested: false,
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_non_send_resource(MainThreadToken::new())
        .init_resource::<PluginWindowState>()
        .add_systems(Startup, (setup_camera_system, setup_audio_system))
        .add_systems(
            Update,
            (capture_parent_window_handle_system, sync_plugin_gui_system),
        )
        .add_systems(EguiPrimaryContextPass, ui_main_layout_system)
        .run();
}

#[derive(Resource, Default)]
struct PluginWindowState {
    parent_hwnd: Option<isize>,
    last_known_rect: Option<egui::Rect>,
    is_visible: bool,
    requested_size_physical: Option<(u32, u32)>,
}

struct MainThreadToken(#[allow(dead_code)] Rc<()>);

impl MainThreadToken {
    fn new() -> Self {
        Self(Rc::new(()))
    }
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
    _main_thread: NonSend<MainThreadToken>,
) {
    if state.parent_hwnd.is_some() {
        return;
    }

    if let Ok(handle_wrapper) = query.single() {
        let handle = handle_wrapper.get_window_handle();
        if let RawWindowHandle::Win32(handle) = handle {
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
    window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    _main_thread: NonSend<MainThreadToken>,
) {
    let Ok((window_entity, window)) = window_query.single() else {
        return;
    };

    let ctx = match contexts.ctx_for_entity_mut(window_entity) {
        Ok(ctx) => ctx,
        Err(_) => return,
    };

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

        let available_rect = ui.available_rect_before_wrap();
        let mut desired_size = available_rect.size();
        if let Some((width, height)) = state.requested_size_physical {
            let scale = window.scale_factor() as f32;
            if scale > 0.0 {
                desired_size = egui::vec2(width as f32 / scale, height as f32 / scale);
            }
        }
        let clamped_size = egui::vec2(
            desired_size.x.min(available_rect.width()),
            desired_size.y.min(available_rect.height()),
        );
        let (_rect, response) = ui.allocate_exact_size(clamped_size, egui::Sense::hover());

        state.last_known_rect = Some(response.rect);

        ui.painter().rect_stroke(
            response.rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
            egui::StrokeKind::Middle,
        );
    });
}

fn sync_plugin_gui_system(
    mut state: ResMut<PluginWindowState>,
    audio_state: Option<Res<AudioState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _main_thread: NonSend<MainThreadToken>,
) {
    let Some(audio) = audio_state else {
        return;
    };
    let requests = audio.take_gui_requests();
    if let Some((width, height)) = requests.requested_resize {
        state.requested_size_physical = Some((width, height));
    }
    if requests.requested_show {
        state.is_visible = true;
    }
    if requests.requested_hide || requests.closed {
        state.is_visible = false;
    }

    let Some(rect) = state.last_known_rect else {
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
        scale: scale as f64,
    };

    audio.update_gui_layout(physical_rect);
}
