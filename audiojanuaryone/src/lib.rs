use nih_plug::prelude::*;
use nih_plug_iced::*;
use std::sync::Arc;

#[derive(Params)]
struct MyParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[persist = "editor-state"]
    pub editor_state: Arc<IcedState>,
}

impl Default for MyParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", 1.0, FloatRange::Linear { min: 0.0, max: 2.0 }),
            editor_state: IcedState::from_size(200, 150),
        }
    }
}

struct MyPlugin {
    params: Arc<MyParams>,
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(MyParams::default()),
        }
    }
}

pub fn library_output() {
    println!("<library_output>");
}

impl Plugin for MyPlugin {
    const NAME: &'static str = "Hello Rust Plugin";
    const VENDOR: &'static str = "My Name";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = "0.0.1";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_iced_editor::<MyEditor>(
            self.params.editor_state.clone(),
            MyEditorInitializationFlags {
                params: self.params.clone(),
            },
        )
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.value();
            for sample in channel_samples {
                *sample *= gain;
            }
        }
        ProcessStatus::Normal
    }
}

struct MyEditor {
    params: Arc<MyParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: widgets::param_slider::State,
}

#[derive(Clone)]
struct MyEditorInitializationFlags {
    params: Arc<MyParams>,
}

impl IcedEditor for MyEditor {
    type Executor = executor::Default;
    type Message = ();
    type InitializationFlags = MyEditorInitializationFlags;

    fn new(
        flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (
            MyEditor {
                params: flags.params,
                context,
                gain_slider_state: Default::default(),
            },
            Command::none(),
        )
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .push(Text::new("Hello from Rust!"))
            .push(
                widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(|_| ()),
            )
            .into()
    }
}

impl Vst3Plugin for MyPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"HelloRustPlugin1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_vst3!(MyPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_multiplication() {
        let plugin = MyPlugin::default();

        //assert_eq!(plugin.params.gain.value(), 0.999);
        assert_eq!(plugin.params.gain.value(), 1.0);
    }
}
