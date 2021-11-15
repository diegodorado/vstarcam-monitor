mod style;

use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};

use iced::{
    button, executor, keyboard, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text, VerticalAlignment,
};
use iced_native::{window, Event};
use iced_video_player::{VideoPlayer, VideoPlayerMessage};

const INCONSOLATA_BYTES: &'static [u8] = include_bytes!("../fonts/Inconsolata-Regular.ttf");

#[derive(Default)]
struct Flags {
    ip_octets: [u8; 4],
    port: u16,
}

impl Flags {
    fn init() -> Flags {
        let socket = UdpSocket::bind("0.0.0.0:8601").expect("couldn't bind to address");
        socket
            .set_broadcast(true)
            .expect("set_broadcast call failed");

        socket
            .send_to(&[0x44, 0x48, 0x01, 0x01], "255.255.255.255:8600")
            .expect("couldn't send message");

        let mut buf = [0; 524];
        let (_len, addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

        let ip_octets = match addr.ip() {
            IpAddr::V4(ipv4) => ipv4.octets(),
            _ => panic!("Invalid IP format."),
        };

        let port: u16 = ((buf[91] as u16) << 8) + (buf[90] as u16);

        Flags { ip_octets, port }
    }
}

pub fn main() -> iced::Result {
    let flags = Flags::init();

    App::run(Settings {
        exit_on_close_request: true,
        default_font: Some(INCONSOLATA_BYTES),
        ..Settings::with_flags(flags)
    })
}

fn keycap(key: char) -> Text {
    Text::new(key.to_string())
        .size(30)
        .width(Length::Units(50))
        .height(Length::Units(50))
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
}

#[derive(Clone, Debug)]
enum Message {
    Loaded(Result<String, AppError>),
    EventOccurred(iced_native::Event),
    VideoPlayerMessage(VideoPlayerMessage),
    UpPressed,
    DownPressed,
    LeftPressed,
    RightPressed,
    CmdSent(Result<String, AppError>),
}

struct App {
    video: VideoPlayer,
    socket: SocketAddrV4,
    up_btn: button::State,
    down_btn: button::State,
    left_btn: button::State,
    right_btn: button::State,
    should_exit: bool,
}

enum CameraCmd {
    PtzUp,
    PtzUpStop,
    PtzDown,
    PtzDownStop,
    PtzLeft,
    PtzLeftStop,
    PtzRight,
    PtzRightStop,
    PtzLeftUp,
    PtzRightUp,
    PtzLeftDown,
    PtzRightDown,
    PtzCenter,
    PtzVPatrol,
    PtzVPatrolStop,
    PtzHPatrol,
    PtzHPatrolStop,
    IrOn,
    IrOff,
}

impl CameraCmd {
    fn value(&self) -> u8 {
        match *self {
            Self::PtzUp => 0,
            Self::PtzUpStop => 1,
            Self::PtzDown => 2,
            Self::PtzDownStop => 3,
            Self::PtzLeft => 4,
            Self::PtzLeftStop => 5,
            Self::PtzRight => 6,
            Self::PtzRightStop => 7,
            Self::PtzLeftUp => 90,
            Self::PtzRightUp => 91,
            Self::PtzLeftDown => 92,
            Self::PtzRightDown => 93,
            Self::PtzCenter => 25,
            Self::PtzVPatrol => 26,
            Self::PtzVPatrolStop => 27,
            Self::PtzHPatrol => 28,
            Self::PtzHPatrolStop => 29,
            Self::IrOn => 94,
            Self::IrOff => 94,
        }
    }
}

#[derive(Debug, Clone)]
enum AppError {
    InitializationError(&'static str),
    APIError,
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> AppError {
        dbg!(error);
        AppError::APIError
    }
}

impl App {
    async fn initialize() -> Result<String, AppError> {
        Ok("Everythink Ok".to_string())
    }

    async fn send_cmd(socket: SocketAddrV4, cmd: CameraCmd) -> Result<String, AppError> {
        let username = "admin";
        let passsword = "qwerpoiu";
        let onestep = 0;
        let url = format!(
            "http://{}:{}/decoder_control.cgi?loginuse={}&loginpas={}&command={}&onestep={}",
            socket.ip(),
            socket.port(),
            username,
            passsword,
            cmd.value(),
            onestep,
        );
        println!("{:?}", url);
        let body = reqwest::get(url).await?.text().await?;

        println!("body = {:?}", body);
        Ok(body)
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Message>) {
        let o = flags.ip_octets;
        let ip = Ipv4Addr::new(o[0], o[1], o[2], o[3]);
        let socket = SocketAddrV4::new(ip, flags.port);
        let username = "admin";
        let passsword = "qwerpoiu";
        let rtsp_port = 10554;
        let url = format!(
            "rtsp://{}:{}@{}:{}/udp/av0_0",
            username,
            passsword,
            ip.to_string(),
            rtsp_port,
        );
        println!("{:?}", url);

        let video = VideoPlayer::new(&url::Url::parse(&url.to_string()).unwrap(), true).unwrap();

        (
            App {
                socket,
                video,
                up_btn: Default::default(),
                down_btn: Default::default(),
                left_btn: Default::default(),
                right_btn: Default::default(),
                should_exit: false,
            },
            Command::perform(App::initialize(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Message, _: &mut iced::Clipboard) -> Command<Message> {
        match message {
            Message::CmdSent(Ok(msg)) => {
                println!("Sent {:?}", msg);
                Command::none()
            }

            Message::CmdSent(Err(msg)) => {
                println!("Sent Err {:?}", msg);
                Command::none()
            }

            Message::Loaded(Ok(msg)) => {
                println!("LOaded {:?}", msg);
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

            Message::EventOccurred(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers: _,
                    key_code,
                }) => match key_code {
                    keyboard::KeyCode::W => Command::perform(
                        App::send_cmd(self.socket, CameraCmd::PtzUp),
                        Message::CmdSent,
                    ),
                    keyboard::KeyCode::S => Command::perform(
                        App::send_cmd(self.socket, CameraCmd::PtzDown),
                        Message::CmdSent,
                    ),
                    keyboard::KeyCode::Escape => {
                        println!("QUIT");
                        self.should_exit = true;
                        Command::none()
                    }
                    _ => Command::none(),
                },
                Event::Keyboard(keyboard::Event::KeyReleased {
                    modifiers: _,
                    key_code,
                }) => match key_code {
                    keyboard::KeyCode::W => Command::perform(
                        App::send_cmd(self.socket, CameraCmd::PtzUpStop),
                        Message::CmdSent,
                    ),
                    keyboard::KeyCode::S => Command::perform(
                        App::send_cmd(self.socket, CameraCmd::PtzDownStop),
                        Message::CmdSent,
                    ),
                    _ => Command::none(),
                },
                Event::Window(window::Event::CloseRequested) => {
                    self.should_exit = true;
                    Command::none()
                }
                _ => Command::none(),
            },
            Message::VideoPlayerMessage(msg) => {
                return self.video.update(msg).map(Message::VideoPlayerMessage);
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.video.subscription().map(Message::VideoPlayerMessage);
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(
                Text::new("Load")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .push(self.video.frame_view())
            .push(
                Row::new()
                    .align_items(Align::End)
                    .push(
                        Button::new(&mut self.left_btn, keycap('A'))
                            .style(style::Button)
                            .on_press(Message::LeftPressed),
                    )
                    .push(
                        Column::new()
                            .push(
                                Button::new(&mut self.up_btn, keycap('W'))
                                    .style(style::Button)
                                    .on_press(Message::UpPressed),
                            )
                            .push(
                                Button::new(&mut self.down_btn, keycap('S'))
                                    .style(style::Button)
                                    .on_press(Message::DownPressed),
                            ),
                    )
                    .push(
                        Button::new(&mut self.right_btn, keycap('D'))
                            .style(style::Button)
                            .on_press(Message::RightPressed),
                    ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}
