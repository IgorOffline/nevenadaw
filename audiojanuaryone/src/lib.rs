use nih_plug::prelude::*;
use nih_plug_iced::widgets::ParamMessage;
use nih_plug_iced::*;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

#[derive(Params)]
struct AudioJanuaryOneParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "pan"]
    pub pan: FloatParam,

    #[persist = "editor-state"]
    pub editor_state: Arc<IcedState>,
}

impl Default for AudioJanuaryOneParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", 1.0, FloatRange::Linear { min: 0.0, max: 2.0 }),
            pan: FloatParam::new(
                "Pan",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
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
        for mut channel_samples in buffer.iter_samples() {
            let pan_value = self.params.pan.value();
            let gain_value = self.params.gain.value();

            let n_pan = (pan_value + 1.0) / 2.0;
            let left_gain = (1.0 - n_pan).sqrt() * gain_value;
            let right_gain = n_pan.sqrt() * gain_value;

            if let Some(left_sample) = channel_samples.get_mut(0) {
                *left_sample *= left_gain;
            }
            if let Some(right_sample) = channel_samples.get_mut(1) {
                *right_sample *= right_gain;
            }
        }

        ProcessStatus::Normal
    }
}

struct AudioJanuaryOneEditor {
    params: Arc<AudioJanuaryOneParams>,
    context: Arc<dyn GuiContext>,

    minus_button_state: button::State,
    plus_button_state: button::State,
}

#[derive(Clone)]
struct AudioJanuaryOneEditorInitializationFlags {
    params: Arc<AudioJanuaryOneParams>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncreaseGain,
    DecreaseGain,
}

impl From<ParamMessage> for Message {
    fn from(_value: ParamMessage) -> Self {
        Message::IncreaseGain
    }
}

impl IcedEditor for AudioJanuaryOneEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = AudioJanuaryOneEditorInitializationFlags;

    fn new(
        flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        (
            AudioJanuaryOneEditor {
                params: flags.params,
                context,
                minus_button_state: Default::default(),
                plus_button_state: Default::default(),
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
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::IncreaseGain => {
                let current_gain = self.params.gain.value();
                let new_gain = (current_gain + 0.1).min(2.0);
                let normalized_gain = self.params.gain.preview_normalized(new_gain);

                let result = std::panic::catch_unwind(AssertUnwindSafe(|| unsafe {
                    self.context
                        .raw_begin_set_parameter(self.params.gain.as_ptr());

                    self.context
                        .raw_set_parameter_normalized(self.params.gain.as_ptr(), normalized_gain);

                    self.context
                        .raw_end_set_parameter(self.params.gain.as_ptr());
                }));

                if let Err(_) = result {
                    println!("Error: Panic occurred during IncreaseGain parameter update.");
                }
            }
            Message::DecreaseGain => {
                let current_gain = self.params.gain.value();
                let new_gain = (current_gain - 0.1).max(0.0);
                let normalized_gain = self.params.gain.preview_normalized(new_gain);

                let result = std::panic::catch_unwind(AssertUnwindSafe(|| unsafe {
                    self.context
                        .raw_begin_set_parameter(self.params.gain.as_ptr());

                    self.context
                        .raw_set_parameter_normalized(self.params.gain.as_ptr(), normalized_gain);

                    self.context
                        .raw_end_set_parameter(self.params.gain.as_ptr());
                }));

                if let Err(_) = result {
                    println!("Error: Panic occurred during DecreaseGain parameter update.");
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let gain = self.params.gain.value();
        Column::new()
            .push(Text::new(format!("[ {:.2} ]", gain)))
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Button::new(&mut self.minus_button_state, Text::new("-"))
                            .on_press(Message::DecreaseGain),
                    )
                    .push(
                        Button::new(&mut self.plus_button_state, Text::new("+"))
                            .on_press(Message::IncreaseGain),
                    ),
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

    #[test]
    fn test_default_pan() {
        let plugin = AudioJanuaryOnePlugin::default();

        assert_eq!(plugin.params.pan.value(), 0.0);
    }
}
