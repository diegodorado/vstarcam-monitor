use iced::{button, container, Background, Color};

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

const BLACK: Color = Color::from_rgb(
    0x11 as f32 / 255.0,
    0x11 as f32 / 255.0,
    0x11 as f32 / 255.0,
);

const WHITE: Color = Color::from_rgb(
    0xfa as f32 / 255.0,
    0xfa as f32 / 255.0,
    0xfa as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BLACK)),
            text_color: Some(WHITE),
            ..container::Style::default()
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(BLACK)),
            text_color: WHITE,
            border_width: 1.0,
            border_color: WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(HOVERED)),
            text_color: Color::WHITE,
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style { ..self.hovered() }
    }
}
