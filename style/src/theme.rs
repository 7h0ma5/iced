mod palette;

pub use self::palette::Palette;

use crate::application;
use crate::button;
use crate::radio;
use crate::slider;

use iced_core::{Background, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn palette(self) -> Palette {
        match self {
            Self::Light => Palette::LIGHT,
            Self::Dark => Palette::DARK,
        }
    }

    fn extended_palette(&self) -> &palette::Extended {
        match self {
            Self::Light => &palette::EXTENDED_LIGHT,
            Self::Dark => &palette::EXTENDED_DARK,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

impl application::StyleSheet for Theme {
    fn background_color(&self) -> Color {
        let palette = self.extended_palette();

        palette.background.base.color
    }

    fn text_color(&self) -> Color {
        let palette = self.extended_palette();

        palette.background.base.text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Primary,
    Secondary,
    Positive,
    Destructive,
    Text,
}

impl Default for Button {
    fn default() -> Self {
        Self::Primary
    }
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: Self::Style) -> button::Appearance {
        let palette = self.extended_palette();

        let appearance = button::Appearance {
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        let from_pair = |pair: palette::Pair| button::Appearance {
            background: Some(pair.color.into()),
            text_color: pair.text,
            ..appearance
        };

        match style {
            Button::Primary => from_pair(palette.primary.strong),
            Button::Secondary => from_pair(palette.secondary.base),
            Button::Positive => from_pair(palette.success.base),
            Button::Destructive => from_pair(palette.danger.base),
            Button::Text => button::Appearance {
                text_color: palette.background.base.text,
                ..appearance
            },
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let palette = self.extended_palette();

        let background = match style {
            Button::Primary => Some(palette.primary.base.color),
            Button::Secondary => Some(palette.background.strong.color),
            Button::Positive => Some(palette.success.strong.color),
            Button::Destructive => Some(palette.danger.strong.color),
            Button::Text => None,
        };

        button::Appearance {
            background: background.map(Background::from),
            ..active
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> slider::Appearance {
        let palette = self.extended_palette();

        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 4.0,
            },
            color: Color::WHITE,
            border_color: Color::WHITE,
            border_width: 1.0,
        };

        slider::Appearance {
            rail_colors: (palette.primary.base.color, Color::TRANSPARENT),
            handle: slider::Handle {
                color: palette.background.base.color,
                border_color: palette.primary.base.color,
                ..handle
            },
        }
    }

    fn hovered(&self, style: Self::Style) -> slider::Appearance {
        let active = self.active(style);
        let palette = self.extended_palette();

        slider::Appearance {
            handle: slider::Handle {
                color: palette.primary.weak.color,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self, style: Self::Style) -> slider::Appearance {
        let active = self.active(style);
        let palette = self.extended_palette();

        slider::Appearance {
            handle: slider::Handle {
                color: palette.primary.base.color,
                ..active.handle
            },
            ..active
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> radio::Appearance {
        let palette = self.extended_palette();

        radio::Appearance {
            background: Color::TRANSPARENT.into(),
            dot_color: palette.primary.strong.color.into(),
            border_width: 1.0,
            border_color: palette.primary.strong.color,
            text_color: None,
        }
    }

    fn hovered(&self, style: Self::Style) -> radio::Appearance {
        let active = self.active(style);
        let palette = self.extended_palette();

        radio::Appearance {
            dot_color: palette.primary.weak.text.into(),
            background: palette.primary.weak.color.into(),
            ..active
        }
    }
}
