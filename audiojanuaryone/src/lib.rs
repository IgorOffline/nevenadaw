use nih_plug::prelude::*;
use nih_plug_iced::*;
use std::sync::Arc;

#[derive(Params)]
struct AudioJanuaryOneParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[persist = "editor-state"]
    pub editor_state: Arc<IcedState>,
}

impl Default for AudioJanuaryOneParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", 1.0, FloatRange::Linear { min: 0.0, max: 2.0 }),
            editor_state: IcedState::from_size(640, 360),
        }
    }
}

pub struct AudioJanuaryOnePlugin {
    params: Arc<AudioJanuaryOneParams>,
}

impl Default for AudioJanuaryOnePlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(AudioJanuaryOneParams::default()),
        }
    }
}

impl Plugin for AudioJanuaryOnePlugin {
    const NAME: &'static str = "AudioJanuaryOneName";
    const VENDOR: &'static str = "AudioJanuaryOneVendor";
    const URL: &'static str = "https://url.example.com";
    const EMAIL: &'static str = "email@example.com";

    const VERSION: &'static str = "0.1.0";

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
        create_iced_editor::<AudioJanuaryOneEditor>(
            self.params.editor_state.clone(),
            AudioJanuaryOneEditorInitializationFlags {
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

struct AudioJanuaryOneEditor {
    params: Arc<AudioJanuaryOneParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: widgets::param_slider::State,
}

#[derive(Clone)]
struct AudioJanuaryOneEditorInitializationFlags {
    params: Arc<AudioJanuaryOneParams>,
}

impl IcedEditor for AudioJanuaryOneEditor {
    type Executor = executor::Default;
    type Message = ();
    type InitializationFlags = AudioJanuaryOneEditorInitializationFlags;

    fn new(
        flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (
            AudioJanuaryOneEditor {
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
            .push(Text::new("[ AudioJanuaryOne ]"))
            .push(
                widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(|_| ()),
            )
            .into()
    }
}

impl Vst3Plugin for AudioJanuaryOnePlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"IgorOffPlugin010";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_vst3!(AudioJanuaryOnePlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_multiplication() {
        let plugin = AudioJanuaryOnePlugin::default();

        assert_eq!(plugin.params.gain.value(), 1.0);
    }
}
