use iced::{Color, Theme, color};

mod button;
mod fonts;
mod picklist;

pub(crate) use button::{p_button, s_button};
#[allow(unused)]
pub(crate) use fonts::{
    CHEVRON_BACK, DOWNLOAD, FILE, GEAR, MINUS, PLUS, SHARE, STAR, WRENCH, icon, icon_text,
    load_fonts,
};
pub(crate) use picklist::s_pick_list;

pub(crate) const THEME_CORNER_RADIUS: f32 = 4.0;
pub(crate) const THEME_FONT_SIZE: f32 = 12.0;
pub(crate) const THEME_PADDING: f32 = 9.0;
pub(crate) const THEME_ICON_SIZE: f32 = 12.0;

pub(crate) fn p_font() -> iced::Font {
    iced::Font {
        family: iced::font::Family::Name("Hack".into()),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum PlumeTheme {
    PlumeDark,
    Mint,
    Blue,
    Red,
}

impl PlumeTheme {
    pub(crate) const ALL: &'static [PlumeTheme] = &[
        PlumeTheme::PlumeDark,
        PlumeTheme::Mint,
        PlumeTheme::Blue,
        PlumeTheme::Red,
    ];

    pub(crate) fn to_iced_theme(self) -> Theme {
        match self {
            Self::PlumeDark => Self::plume_dark(),
            Self::Mint => Self::mint(),
            Self::Blue => Self::blue(),
            Self::Red => Self::red(),
        }
    }

    fn plume_dark() -> Theme {
        Theme::custom(
            "Plume Dark".to_string(),
            iced::theme::Palette {
                background: color!(0x282021),
                text: color!(0xf2d5cf),
                primary: color!(0xd3869b),
                success: color!(0xd9a6b3),
                danger: color!(0xe78a8a),
                warning: color!(0xf4b8c4),
            },
        )
    }

    fn mint() -> Theme {
        Theme::custom(
            "Mint".to_string(),
            iced::theme::Palette {
                background: color!(0x1e2e28),
                text: color!(0xd4ece1),
                primary: color!(0x55b182),
                success: color!(0x7dcda3),
                danger: color!(0xe27a7a),
                warning: color!(0xe6c37c),
            },
        )
    }

    fn blue() -> Theme {
        Theme::custom(
            "Blue".to_string(),
            iced::theme::Palette {
                background: color!(0x1a2130),
                text: color!(0xd6e5fa),
                primary: color!(0x4d8bd9),
                success: color!(0x7bb1e6),
                danger: color!(0xe46c6c),
                warning: color!(0xe8c17b),
            },
        )
    }

    fn red() -> Theme {
        Theme::custom(
            "Red".to_string(),
            iced::theme::Palette {
                background: color!(0x2d1a1b),
                text: color!(0xf7d9da),
                primary: color!(0xd9464e),
                success: color!(0xdf7c7e),
                danger: color!(0xe84a4a),
                warning: color!(0xe3b772),
            },
        )
    }
}

impl std::fmt::Display for PlumeTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PlumeDark => "Plume Dark",
                Self::Mint => "Mint",
                Self::Blue => "Blue",
                Self::Red => "Red",
            }
        )
    }
}

impl Default for PlumeTheme {
    fn default() -> Self {
        Self::PlumeDark
    }
}

pub(crate) fn lighten(color: Color, amount: f32) -> Color {
    Color {
        r: (color.r + amount).min(1.0),
        g: (color.g + amount).min(1.0),
        b: (color.b + amount).min(1.0),
        a: color.a,
    }
}

pub(crate) fn darken(color: Color, amount: f32) -> Color {
    Color {
        r: (color.r - amount).max(0.0),
        g: (color.g - amount).max(0.0),
        b: (color.b - amount).max(0.0),
        a: color.a,
    }
}

pub(crate) fn p_progress_bar(theme: &Theme) -> iced::widget::progress_bar::Style {
    let palette = theme.palette();
    iced::widget::progress_bar::Style {
        background: iced::Background::Color(lighten(palette.background, 0.05)),
        bar: iced::Background::Color(palette.primary),
        border: iced::Border {
            radius: THEME_CORNER_RADIUS.into(),
            ..Default::default()
        },
    }
}

pub(crate) fn s_scrollable(
    _theme: &Theme,
    _status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),
        vertical_rail: iced::widget::scrollable::Rail {
            background: None,
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: iced::Background::Color(Color::TRANSPARENT),
                border: iced::Border::default(),
            },
        },
        horizontal_rail: iced::widget::scrollable::Rail {
            background: None,
            border: iced::Border::default(),
            scroller: iced::widget::scrollable::Scroller {
                background: iced::Background::Color(Color::TRANSPARENT),
                border: iced::Border::default(),
            },
        },
        gap: None,
        auto_scroll: None,
    }
}
