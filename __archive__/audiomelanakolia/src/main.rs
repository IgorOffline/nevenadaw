use bevy::prelude::*;
use nih_plug::prelude::{Plugin as AudioPlugin, *};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 200;

const GAIN_MIN: f32 = 0.0;
const GAIN_MAX: f32 = 2.0;
const GAIN_DEFAULT: f32 = 1.0;
const GAIN_STEP: f32 = 0.1;
const GAIN_SMOOTHING_MS: f32 = 20.0;

#[derive(Resource)]
struct BevyArc<T: ?Sized + Send + Sync + 'static>(Arc<T>);

impl<T: ?Sized + Send + Sync + 'static> std::ops::Deref for BevyArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BevyState {
    pub size: (u32, u32),
}

impl BevyState {
    pub fn from_size(width: u32, height: u32) -> Arc<Self> {
        Arc::new(Self {
            size: (width, height),
        })
    }
}

pub fn create_bevy_editor(
    state: Arc<BevyState>,
    _data: (),
    initialize: impl Fn(&mut App, Arc<dyn GuiContext>) + Send + Sync + 'static,
) -> Option<Box<dyn Editor>> {
    Some(Box::new(BevyEditor {
        state,
        initialize: Arc::new(initialize),
    }))
}

struct BevyEditor {
    state: Arc<BevyState>,
    initialize: Arc<dyn Fn(&mut App, Arc<dyn GuiContext>) + Send + Sync>,
}

impl Editor for BevyEditor {
    fn spawn(
        &self,
        _parent: ParentWindowHandle,
        context: Arc<dyn GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let initialize = self.initialize.clone();
        std::thread::spawn(move || {
            let mut app = App::new();
            (initialize)(&mut app, context);
            app.run();
        });

        Box::new(())
    }

    fn size(&self) -> (u32, u32) {
        self.state.size
    }

    fn set_scale_factor(&self, _factor: f32) -> bool {
        false
    }

    fn param_value_changed(&self, _id: &str, _value: f32) {}
    fn param_modulation_changed(&self, _id: &str, _modulation_offset: f32) {}
    fn param_values_changed(&self) {}
}

struct Foo {
    params: Arc<FooParams>,
    editor_state: Arc<BevyState>,
}

#[derive(Params)]
struct FooParams {
    #[id = "foo"]
    pub foo: FloatParam,
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            params: Arc::new(FooParams::default()),
            editor_state: BevyState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT),
        }
    }
}

impl Default for FooParams {
    fn default() -> Self {
        Self {
            foo: FloatParam::new(
                "Gain",
                GAIN_DEFAULT,
                FloatRange::Skewed {
                    min: GAIN_MIN,
                    max: GAIN_MAX,
                    factor: FloatRange::gain_skew_factor(GAIN_MIN, GAIN_MAX),
                },
            )
            .with_smoother(SmoothingStyle::Linear(GAIN_SMOOTHING_MS))
            .with_unit(" x"),
        }
    }
}

impl AudioPlugin for Foo {
    const NAME: &'static str = "Gain Plugin";
    const VENDOR: &'static str = "Foo Vendor";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = "0.1.0";
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        create_bevy_editor(self.editor_state.clone(), (), move |app, context| {
            app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    title: "Audiomelanakolia".to_string(),
                    ..default()
                }),
                ..default()
            }))
            .insert_resource(BevyArc(params.clone()))
            .insert_resource(BevyArc(context.clone()))
            .add_systems(Startup, setup)
            .add_systems(Update, (button_system, update_ui_system));
        })
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.foo.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }
}

#[derive(Component)]
enum Action {
    Increase,
    Decrease,
}

#[derive(Component)]
struct ValueText;

fn main() {
    nih_export_standalone::<Foo>();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Iosevka-Italic.ttf");
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Value: 1.00"),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                ValueText,
            ));

            parent
                .spawn(Node {
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|inner_parent| {
                    spawn_button(inner_parent, "Decrease", Action::Decrease, font.clone());
                    spawn_button(inner_parent, "Increase", Action::Increase, font.clone());
                });
        });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: Action,
    font: Handle<Font>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Action, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    params: Res<BevyArc<FooParams>>,
    context: Res<BevyArc<dyn GuiContext>>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let current = params.foo.value();
                let new_val = match action {
                    Action::Increase => (current + GAIN_STEP).min(GAIN_MAX),
                    Action::Decrease => (current - GAIN_STEP).max(GAIN_MIN),
                };
                if (new_val - current).abs() > f32::EPSILON {
                    let normalized = params.foo.preview_normalized(new_val);
                    unsafe {
                        context
                            .0
                            .raw_set_parameter_normalized(params.foo.as_ptr(), normalized);
                    }
                }
                color.0 = Color::srgb(0.35, 0.75, 0.35);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.15, 0.15, 0.15);
            }
        }
    }
}

fn update_ui_system(
    params: Res<BevyArc<FooParams>>,
    mut query: Query<&mut Text, With<ValueText>>,
    mut last_value: Local<Option<f32>>,
) {
    let value = params.foo.value();
    let should_update = last_value.map_or(true, |previous| (previous - value).abs() > f32::EPSILON);
    if !should_update {
        return;
    }

    for mut text in &mut query {
        text.0 = format!("Value: {:.2}", value);
    }
    *last_value = Some(value);
}
