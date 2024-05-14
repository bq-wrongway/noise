use std::{collections::HashMap, path::Path, time::Duration};
mod files;
use files::{load_data, NoiseTrack};
use iced::{
    alignment::Vertical,
    executor,
    widget::{button, container, horizontal_space, mouse_area, row, slider, text, Column},
    window::Settings,
    Application, Border, Color, Command, Length, Padding, Theme,
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
const PADDING: f32 = 5.0;
const SPACING: f32 = 5.0;
const LINEAR_TWEEN: Tween = Tween {
    duration: Duration::from_secs(1),
    easing: Easing::Linear,
    start_time: StartTime::Immediate,
};
pub fn main() -> iced::Result {
    Noise::run(iced::Settings {
        window: Settings {
            size: iced::Size {
                width: 560.0,
                height: 600.0,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Noise {
    manager: AudioManager,
    files: Vec<NoiseTrack>,
    currently_playing: HashMap<usize, StreamingSoundHandle<FromFileError>>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Play(usize),
    Theme,
    VolumeChanged((f32, usize)),
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
            Message::Play(i) => {
                let mut is_playing = self.files.get_mut(i).unwrap().is_playing;
                match self.currently_playing.get_mut(&i) {
                    Some(h) => match h.state() {
                        PlaybackState::Playing => {
                            self.files.get_mut(i).unwrap().is_playing = false;
                            println!("is stop,should be false:{:?}", is_playing);
                            h.pause(LINEAR_TWEEN).unwrap()
                        }

                        _ => {
                            self.files.get_mut(i).unwrap().is_playing = true;
                            println!("iresumed,shouldbe true:{:?}", is_playing);

                            h.resume(Tween::default()).unwrap()
                        }
                    },
                    None => {
                        let settings = StreamingSoundSettings::new().loop_region(0.0..);
                        let sound_data =
                            StreamingSoundData::from_file(Path::new(&self.files[i].path), settings)
                                .unwrap();

                        let handler = self.manager.play(sound_data).unwrap();
                        self.currently_playing.insert(i, handler);
                        self.files.get_mut(i).unwrap().is_playing = true;
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
                        // let i = self.files.iter().position(|r| r.name == s).unwrap();
                        let aass = self.files.get_mut(s).unwrap();
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
        let content = self
            .files
            .iter()
            .enumerate()
            .fold(Wrap::new(), |w, (i, t)| {
                w.push(
                    mouse_area(
                        container(
                            get_component(&t, i)
                                .push_maybe(self.files[i].is_playing.then(|| text("wl"))),
                        )
                        .width(180)
                        .height(80)
                        .padding(PADDING)
                        .style(get_style(self.theme().palette().primary)),
                    )
                    .on_press(Message::Play(i)),
                )
                .spacing(SPACING)
                .line_spacing(SPACING)
            });
        let layout = Column::new();

        container(
            layout
                .push(
                    row!(button("Theme tbd").on_press(Message::Theme).height(40.0))
                        .width(Length::Fill),
                )
                .push(content)
                .spacing(SPACING),
        )
        .padding(Padding {
            top: PADDING,
            right: 0.,
            bottom: PADDING,
            left: PADDING,
        })
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

fn get_style(color: Color) -> container::Appearance {
    return container::Appearance {
        border: Border {
            color,
            width: 1.3,
            radius: 6.0.into(),
        },
        ..Default::default()
    };
}
fn uppercase_first(data: &str) -> String {
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
        }
    }
    result
}
pub fn get_component(t: &NoiseTrack, i: usize) -> iced::widget::Column<Message> {
    iced::widget::column![
        row![
            text(uppercase_first("[p]"))
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center),
            horizontal_space(),
            text(uppercase_first(&t.name))
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center),
            horizontal_space(),
            text("[ * ]")
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
        slider(0.0..=1.0, t.volume_level, move |x| Message::VolumeChanged(
            (x, i)
        ))
        // .style(iced::theme::Slider::Custom(Box::new(CustomSlider::Active)))
        .step(0.01)
        .height(10.0)
    ]
    .spacing(SPACING) //might not be necesserry, the size of whole container should be reduced, but will see based on icons
    .width(Length::Fill)
    .height(Length::Fill)
}
