use nih_plug::prelude::*;
use nih_plug_iced::*;
use std::sync::Arc;

pub struct Audiocubes {
    params: Arc<AudiocubesParams>,
}

#[derive(Params)]
pub struct AudiocubesParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Audiocubes {
    fn default() -> Self {
        Self {
            params: Arc::new(AudiocubesParams::default()),
        }
    }
}

impl Default for AudiocubesParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2.0,
                    factor: FloatRange::gain_skew_factor(0.0, 2.0),
                },
            )
            .with_unit(" gainfloat"),
        }
    }
}

impl Plugin for Audiocubes {
    const NAME: &'static str = "Audiocubes";
    const VENDOR: &'static str = "Igor Offline";
    const URL: &'static str = "https://igordurbek.com";
    const EMAIL: &'static str = "igor@igordurbek.com";

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
        create(self.params.clone(), default_state())
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel in buffer.as_slice() {
            for sample in channel.iter_mut() {
                *sample *= self.params.gain.value();
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Audiocubes {
    const CLAP_ID: &'static str = "com.igordurbek.audiocubes";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Audiocubes");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(300, 200)
}

pub(crate) fn create(
    params: Arc<AudiocubesParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<AudiocubesEditor>(editor_state, params)
}

struct AudiocubesEditor {
    params: Arc<AudiocubesParams>,
    context: Arc<dyn GuiContext>,

    audiocubes_slider_state: widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(widgets::ParamMessage),
}

impl IcedEditor for AudiocubesEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<AudiocubesParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = AudiocubesEditor {
            params,
            context,

            audiocubes_slider_state: Default::default(),
        };

        (editor, Command::none())
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
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .padding(20)
            .spacing(10)
            .push(
                Text::new("Audiocubes")
                    .size(24)
                    .height(30.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                widgets::ParamSlider::new(&mut self.audiocubes_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .into()
    }
}

nih_export_clap!(Audiocubes);
