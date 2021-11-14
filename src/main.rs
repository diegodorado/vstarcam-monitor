mod style;

use std::net::{IpAddr, UdpSocket};

use iced::{
    button, executor, keyboard, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text, VerticalAlignment,
};
use iced_native::{window, Event};
use iced_video_player::{VideoPlayer, VideoPlayerMessage};

const INCONSOLATA_BYTES: &'static [u8] = include_bytes!("../fonts/Inconsolata-Regular.ttf");

#[derive(Default)]
struct Flags {
 ip: [u8; 4],
  port: u16,
}

fn      get_flags() -> Flags {
    let socket = UdpSocket::bind("0.0.0.0:8601").expect("couldn't bind to address");
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");

    socket
        .send_to(&[0x44, 0x48, 0x01, 0x01], "255.255.255.255:8600")
        .expect("couldn't send message");

    let mut buf = [0; 524];
    let (_len, addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    //println!("{:?}", buf);

    let ip = match addr.ip() {
        IpAddr::V4(ipv4) => ipv4.octets(),
        _ => [0, 0, 0, 0],
    };

    let port: u16 = ((buf[91] as u16) << 8) + (buf[90] as u16);

    Flags { ip, port }
}

pub fn main() -> iced::Result {
    let flags = get_flags();

    App::run(Settings {
        exit_on_close_request: true,
        default_font: Some(INCONSOLATA_BYTES),
        ..Settings::with_flags(flags)
    })
}

fn keycap(key: char) -> Text {
    Text::new(key.to_string())
        .width(Length::Units(50))
        .height(Length::Units(50))
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
}

#[derive(Clone, Debug)]
enum Message {
    Loaded(Result<(), CameraError>),
    EventOccurred(iced_native::Event),
    VideoPlayerMessage(VideoPlayerMessage),
    UpPressed,
    DownPressed,
    LeftPressed,
    RightPressed,
}

struct State {
    video: VideoPlayer,
    up_btn: button::State,
    down_btn: button::State,
    left_btn: button::State,
    right_btn: button::State,
    should_exit: bool,
}

enum App {
    Loading,
    Loaded(State),
}

struct Camera {
    port: u16,
}

#[derive(Debug, Clone)]
enum CameraError {
    RotateError,
    APIError,
}

impl From<reqwest::Error> for CameraError {
    fn from(error: reqwest::Error) -> CameraError {
        dbg!(error);
        CameraError::APIError
    }
}

impl App {
    async fn rotate() -> Result<(), CameraError> {
        let body = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;

        println!("body = {:?}", body);
        Ok(())
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Message>) {
        println!("{:?}", flags.port);
        println!("{:?}", flags.ip);
        let ip = format!(
            "{}.{}.{}.{}",
            flags.ip[0], flags.ip[1], flags.ip[2], flags.ip[3]
        );
        let username = "admin";
        let passsword = "qwerpoiu";
        let rtsp_port = 10554;
        let url = format!(
            "rtsp://{}:{}@{}:{}/udp/av0_0",
            username, passsword, ip, rtsp_port,
        );

        let video = VideoPlayer::new(&url::Url::parse(&url.to_string()).unwrap(), true).unwrap();

        /*
        (
            App {
                video,
                up_btn: Default::default(),
                down_btn: Default::default(),
                left_btn: Default::default(),
                right_btn: Default::default(),
                should_exit: false,
            },
            Command::none(),
        )
        */

        (
            App::Loading,
            Command::perform(App::rotate(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Message, _: &mut iced::Clipboard) -> Command<Message> {
        match message {
            Message::Loaded(Ok(())) => {
                println!("LOaded");
                Command::none()
            }

            Message::Loaded(Err(error)) => {
                println!("ERROR {:?}", error);
                Command::none()
            }

            Message::UpPressed => {
                println!("UP");
                Command::none()
            }

            Message::DownPressed => {
                println!("DOWN");
                Command::none()
            }

            Message::LeftPressed => {
                println!("LEFT");
                Command::none()
            }

            Message::RightPressed => {
                println!("RIGHT");
                Command::none()
            }

            Message::EventOccurred(event) => match self {
                  App::Loading => Command::none(),

                App::Loaded(state) => match event {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        modifiers: _,
                        key_code,
                    }) => {
                        if key_code == keyboard::KeyCode::Escape {
                            println!("QUIT");
                            state.should_exit = true;
                        } else {
                            println!("{:?}", key_code);
                        }
                        Command::none()
                    }
                    Event::Window(window::Event::CloseRequested) => {
                        state.should_exit = true;
                        Command::none()
                    }
                    _ => Command::none(),
                },
            },
            Message::VideoPlayerMessage(msg) => {
                //return self.video.update(msg).map(Message::VideoPlayerMessage);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        //self.video.subscription().map(Message::VideoPlayerMessage);
     iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn should_exit(&self) -> bool {
        match self {
            App::Loading => false,
            App::Loaded(state) => state.should_exit,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self {
            App::Loading => Column::new().push(
                Text::new("Loading")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            ),
            App::Loaded(state) => Column::new().push(state.video.frame_view()).push(
                Row::new()
                    .align_items(Align::End)
                    .push(
                        Button::new(&mut state.left_btn, keycap('A'))
                            .style(style::Button)
                            .on_press(Message::LeftPressed),
                    )
                    .push(
                        Column::new()
                            .push(
                                Button::new(&mut state.up_btn, keycap('W'))
                                    .style(style::Button)
                                    .on_press(Message::UpPressed),
                            )
                            .push(
                                Button::new(&mut state.down_btn, keycap('S'))
                                    .style(style::Button)
                                    .on_press(Message::DownPressed),
                            ),
                    )
                    .push(
                        Button::new(&mut state.right_btn, keycap('D'))
                            .style(style::Button)
                            .on_press(Message::RightPressed),
                    ),
            ),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}

