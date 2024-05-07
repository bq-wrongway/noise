use std::collections::HashMap;

use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{button, container, row, text, Column, Row},
    window::Settings,
    Application, Command, Length, Theme,
};

use iced_aw::Wrap;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
        FromFileError, PlaybackState,
    },
    tween::Tween,
};

pub fn main() -> iced::Result {
    Noise::run(iced::Settings {
        window: Settings {
            size: iced::Size {
                width: 300.0,
                height: 400.0,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Noise {
    manager: AudioManager,
    files: Vec<NoiseTrack>,
    maper: HashMap<String, StreamingSoundHandle<FromFileError>>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Stream,
    Print(NoiseTrack),
    Theme,
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
                files: vec![],
                maper: HashMap::new(),
                theme: Theme::TokyoNight,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Noise")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Stream => {
                for entry in walkdir::WalkDir::new("assets") {
                    let entry = entry.unwrap();
                    println!("{:?}", entry.file_name());
                    if entry.path().is_file() {
                        self.files.push(NoiseTrack {
                            name: entry.file_name().to_str().unwrap().to_string(),
                            path: entry.path().to_str().unwrap().to_string(),
                            handle: false,
                        });
                    }
                }

                Command::none()
            }
            Message::Print(track) => {
                match self.maper.get_mut(&track.name) {
                    Some(k) => match k.state() {
                        PlaybackState::Playing => {
                            k.pause(Tween::default());
                        }
                        _ => {
                            k.resume(Tween::default());
                        }
                    },
                    None => {
                        let sound_data = StreamingSoundData::from_file(
                            track.path,
                            StreamingSoundSettings::default(),
                        )
                        .unwrap();
                        let handler = self.manager.play(sound_data).unwrap();
                        self.maper.insert(track.name, handler);
                    }
                }
                Command::none()
            }
            Message::Theme => {
                self.theme = Theme::Nord;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = self.files.iter().fold(Wrap::new(), |rowe, str| {
            rowe.push(
                button(row![text(str.name.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center),])
                .width(150.0)
                .height(50.0)
                .on_press(Message::Print(str.clone())),
            )
            .spacing(5.0)
            .line_spacing(5.0)
        });
        let layout = Column::new();

        container(
            layout
                .push(
                    row!(
                        button("Add").on_press(Message::Stream).height(40.0),
                        button("theme").on_press(Message::Theme).height(40.0)
                    )
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

#[derive(Debug, Clone)]
struct NoiseTrack {
    name: String,
    path: String,
    handle: bool,
}
