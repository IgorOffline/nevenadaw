use nih_plug::prelude::*;
use nih_plug_iced::*;
use std::sync::Arc;

pub struct Rainhope {
    params: Arc<RainhopeParams>,
}

#[derive(Params)]
pub struct RainhopeParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Rainhope {
    fn default() -> Self {
        Self {
            params: Arc::new(RainhopeParams::default()),
        }
    }
}

impl Default for RainhopeParams {
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

impl Plugin for Rainhope {
    const NAME: &'static str = "Rainhope";
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

impl ClapPlugin for Rainhope {
    const CLAP_ID: &'static str = "com.igordurbek.rainhope";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Rainhope");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(300, 200)
}

pub(crate) fn create(
    params: Arc<RainhopeParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FooEditor>(editor_state, params)
}

struct FooEditor {
    params: Arc<RainhopeParams>,
    context: Arc<dyn GuiContext>,

    foo_slider_state: widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(widgets::ParamMessage),
}

impl IcedEditor for FooEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<RainhopeParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FooEditor {
            params,
            context,

            foo_slider_state: Default::default(),
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
                Text::new("Rainhope")
                    .size(24)
                    .height(30.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                widgets::ParamSlider::new(&mut self.foo_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .into()
    }
}

nih_export_clap!(Rainhope);
