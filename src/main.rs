use iced::{
    advanced::Application,
    executor,
    program::{Appearance, DefaultStyle},
    widget::{button, container, Column, Row},
    window::Settings,
    Command, Renderer, Theme,
};
use std::error::Error;

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    sound::streaming::{StreamingSoundData, StreamingSoundSettings},
    sound::FromFileError,
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
    sound: Option<StaticSoundData>,
    manager: AudioManager,
    // stream: Option<StreamingSoundData<dyn Error>>,
}

#[derive(Debug, Clone)]
enum Message {
    File,
    Play(Option<StaticSoundData>),
    Stream,
    // StreamStart(Option<StreamingSoundData<dyn Error>>),
}

impl Application for Noise {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Renderer = Renderer;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                sound: None,
                manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                    .ok()
                    .unwrap(),
                // stream: None,
            },
            Command::perform(get_file_test(), Message::Play),
        )
    }

    fn title(&self) -> String {
        String::from("Noise")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::File => Command::perform(get_file(), Message::Play),
            Message::Play(s) => {
                println!("file will play...");
                self.sound = s;
                self.manager.play(self.sound.clone().unwrap()).unwrap();
                Command::none()
            }
            Message::Stream => {
                // self.manager.play(self.stream.clone().unwrap()).unwrap();

                Command::none()
            } // Message::StreamStart(s) => {
              //     self.stream = s;
              //     Command::none()
              // }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        let layout = Row::new();
        let con = Column::new()
            .push(button("Add").on_press(Message::Stream))
            .push(button("Fetch").on_press(Message::File));
        container(layout.push(con)).into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self, theme: &Self::Theme) -> Appearance {
        theme.default_style()
    }
}

async fn get_file() -> Option<StaticSoundData> {
    let settings = StaticSoundSettings::new().loop_region(0.0..);
    println!("getting the file..");
    let sound_data = StaticSoundData::from_file("rain.mp3", settings);
    match sound_data {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}
async fn get_file_test() -> Option<StaticSoundData> {
    let settings = StaticSoundSettings::new().loop_region(0.0..);
    println!("reach..");
    let sound_data = StaticSoundData::from_file("rain.mp3", settings);
    match sound_data {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}
//this function returns Result<StreamingSoundData<FromFileError>,FromFileError> i dont know how to deal with this exactly
// async fn play_stream() -> Option<StreamingSoundData<dyn Error>> {
//     const NOISE: &str = "rain.mp3";
//     println!("reach..");
//     let sound_data = StreamingSoundData::from_file(NOISE, StreamingSoundSettings::default()).ok();
//     sound_data
// }
