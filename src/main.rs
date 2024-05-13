use std::{collections::HashMap, path::Path, time::Duration};
mod files;
use files::{get_stem, load_data, NoiseTrack};
use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{button, container, row, slider, text, vertical_slider::HandleShape, Column},
    window::Settings,
    Application, Color, Command, Length, Theme,
};

use iced_aw::Wrap;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
        FromFileError, PlaybackState,
    },
    tween::{Easing, Tween},
    StartTime,
};
const LINEAR_TWEEN: Tween = Tween {
    duration: Duration::from_secs(1),
    easing: Easing::Linear,
    start_time: StartTime::Immediate,
};
pub fn main() -> iced::Result {
    Noise::run(iced::Settings {
        window: Settings {
            size: iced::Size {
                width: 600.0,
                height: 800.0,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Noise {
    manager: AudioManager,
    files: Vec<NoiseTrack>,
    currently_playing: HashMap<String, StreamingSoundHandle<FromFileError>>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Play(String),
    Theme,
    VolumeChanged((f32, String)),
}

impl Application for Noise {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    // type Renderer = Renderer;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                    .ok()
                    .unwrap(),
                files: load_data(),
                currently_playing: HashMap::new(),
                theme: Theme::TokyoNightStorm,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Noise")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Play(track) => {
                match self.currently_playing.get_mut(&get_stem(Path::new(&track))) {
                    Some(k) => match k.state() {
                        PlaybackState::Playing => {
                            k.pause(LINEAR_TWEEN).unwrap();
                        }
                        _ => {
                            k.resume(LINEAR_TWEEN).unwrap();
                        }
                    },
                    None => {
                        //removed tween when starting initially
                        let settings = StreamingSoundSettings::new().loop_region(0.0..);
                        let sound_data =
                            StreamingSoundData::from_file(Path::new(&track), settings).unwrap();
                        // println!("{:?}", sound_data.duration());

                        let handler = self.manager.play(sound_data).unwrap();

                        self.currently_playing
                            .insert(get_stem(Path::new(&track)), handler);
                    }
                }
                Command::none()
            }
            Message::Theme => {
                self.theme = Theme::Nord;
                Command::none()
            }
            Message::VolumeChanged(level) => {
                println!("{:?}", level);
                let (f, s) = level;

                match self.currently_playing.get_mut(&s) {
                    Some(t) => {
                        t.set_volume(f as f64, Tween::default()).unwrap();
                        let i = self.files.iter().position(|r| r.name == s).unwrap();
                        let aass = self.files.get_mut(i).unwrap();
                        aass.volume_level = f;
                    }
                    None => {
                        println!("asd");
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = self.files.iter().fold(Wrap::new(), |w, t| {
            w.push(
                button(iced::widget::column![
                    text(&t.name)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center),
                    slider(0.0..=1.0, t.volume_level, |x| Message::VolumeChanged((
                        x,
                        t.name.clone()
                    )))
                    .style(iced::theme::Slider::Custom(Box::new(CustomSlider::Active)))
                    .step(0.01)
                    .height(10.0)
                ])
                .width(150.0)
                .height(80.0)
                .on_press(Message::Play(t.path.clone())),
            )
            .spacing(5.0)
            .line_spacing(5.0)
        });
        let layout = Column::new();

        container(
            layout
                .push(
                    row!(button("Theme tbd").on_press(Message::Theme).height(40.0))
                        .width(Length::Fill),
                )
                .push(content)
                .spacing(5.0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    // fn style(&self, theme: &Self::Theme) -> Appearance {
    //     theme.default_style()
    // }
}

// Define your custom style here
pub enum CustomSlider {
    Active,
    Disabled,
}
const COLOR: Color = Color::from_rgba(1.0, 152.0 / 255.0, 0.0 / 255.0, 1.0);

impl slider::StyleSheet for CustomSlider {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (COLOR, COLOR),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: iced::widget::slider::Handle {
                shape: HandleShape::Circle { radius: 6.0 },
                color: COLOR,
                border_width: 2.,
                border_color: COLOR,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (COLOR, COLOR),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: iced::widget::slider::Handle {
                shape: HandleShape::Circle { radius: 6.0 },
                color: COLOR,
                border_width: 2.,
                border_color: COLOR,
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (COLOR, COLOR),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: iced::widget::slider::Handle {
                shape: HandleShape::Circle { radius: 6.0 },
                color: COLOR,
                border_width: 2.,
                border_color: COLOR,
            },
        }
    } // actually implement the trait
}
