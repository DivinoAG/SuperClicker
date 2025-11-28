// super_clicker/src/theme.rs

use iced::widget::{
    button, checkbox, container, pick_list, scrollable, text, text_input,
};
use iced::{application, overlay, Border, Color};

// Our custom theme enum.
// We derive Default and set Light as the default theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Copy, Default)]
pub enum TextStyle {
    #[default]
    Default,
    Disabled,
    Accent,
}


// --- Color & Spacing Design Tokens ---
impl AppTheme {
    pub fn primary_background(self) -> Color {
        match self {
            AppTheme::Light => Color::WHITE,
            AppTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1),
        }
    }

    pub fn card_background(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.95, 0.95, 0.95),
            AppTheme::Dark => Color::from_rgb(0.15, 0.15, 0.15),
        }
    }

    pub fn text_primary(self) -> Color {
        match self {
            AppTheme::Light => Color::BLACK,
            AppTheme::Dark => Color::WHITE,
        }
    }

    pub fn text_disabled(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb8(89, 89, 89),      // #595959
            AppTheme::Dark => Color::from_rgb8(142, 142, 142),   // #8E8E8E
        }
    }

    pub fn accent_blue(self) -> Color {
        Color::from_rgb8(0, 90, 158) // #005A9E - WCAG AA+ compliant with white text
    }

    pub fn status_running_green(self) -> Color {
        Color::from_rgb8(46, 125, 50) // #2E7D32
    }

    pub fn status_stopped_grey(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb8(224, 224, 224),
            AppTheme::Dark => Color::from_rgb8(60, 60, 60),
        }
    }

    pub fn button_disabled_background(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb8(224, 224, 224), // #E0E0E0
            AppTheme::Dark => Color::from_rgb(0.2, 0.2, 0.2),    // #333333
        }
    }

    pub fn input_background(self) -> Color {
        match self {
            AppTheme::Light => Color::WHITE,
            AppTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1),
        }
    }

    pub fn input_border(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.7, 0.7, 0.7),
            AppTheme::Dark => Color::from_rgb(0.3, 0.3, 0.3),
        }
    }
}

// --- Custom Style Sheet Implementations ---

// A. Application Level
impl application::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.primary_background(),
            text_color: self.text_primary(),
        }
    }
}

// B. Widget-Specific Styles

// Container Styles
#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerStyle {
    #[default]
    Default,
    Card,
    StatusDisplay,
    StatusDisplayRunning,
    ReadOnly,
}

impl container::StyleSheet for AppTheme {
    type Style = ContainerStyle;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            ContainerStyle::Card => container::Appearance {
                background: Some(self.card_background().into()),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ContainerStyle::StatusDisplay => container::Appearance {
                background: Some(self.status_stopped_grey().into()),
                text_color: Some(match self {
                    AppTheme::Light => Color::from_rgb8(51, 51, 51), // #333333
                    AppTheme::Dark => Color::WHITE,
                }),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ContainerStyle::StatusDisplayRunning => container::Appearance {
                background: Some(self.status_running_green().into()),
                text_color: Some(Color::WHITE),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ContainerStyle::ReadOnly => container::Appearance {
                background: None,
                border: Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: self.input_border(),
                },
                ..Default::default()
            },
            ContainerStyle::Default => Default::default(),
        }
    }
}

// Button Styles
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
}

impl button::StyleSheet for AppTheme {
    type Style = ButtonStyle;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.accent_blue().into()),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            text_color: Color::WHITE, // This is a suggestion, the Text widget style will override
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        let active_appearance = self.active(_style);
        button::Appearance {
            background: Some(self.button_disabled_background().into()),
            text_color: self.text_disabled(),
            border: active_appearance.border,
            ..Default::default()
        }
    }
}

// Text Input Styles
impl text_input::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.input_background().into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.input_border(),
            },
            icon_color: self.text_primary(),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..self.active(style).border
            },
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.button_disabled_background().into(),
            border: Border {
                color: self.text_disabled(),
                ..self.active(style).border
            },
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.text_disabled()
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.text_primary()
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.text_disabled()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.accent_blue()
    }
}

// Pick List Styles
impl pick_list::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            background: self.input_background().into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.input_border(),
            },
            text_color: self.text_primary(),
            handle_color: self.text_primary(),
            placeholder_color: self.text_disabled(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..self.active(style).border
            },
            ..self.active(style)
        }
    }
}

// Checkbox Styles
impl checkbox::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: self.input_background().into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.input_border(),
            },
            icon_color: self.accent_blue(),
            text_color: Some(self.text_primary()),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..self.active(style, is_checked).border
            },
            ..self.active(style, is_checked)
        }
    }

    fn disabled(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let active = self.active(style, is_checked);

        checkbox::Appearance {
            background: self.button_disabled_background().into(),
            icon_color: self.text_disabled(),
            border: Border {
                color: self.text_disabled(),
                ..active.border
            },
            text_color: Some(self.text_disabled()),
        }
    }
}

// Text Styles
impl text::StyleSheet for AppTheme {
    type Style = TextStyle;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let color = match style {
            TextStyle::Default => self.text_primary(),
            TextStyle::Disabled => self.text_disabled(),
            TextStyle::Accent => Color::WHITE,
        };
        text::Appearance { color: Some(color) }
    }
}


// Scrollable Styles
impl scrollable::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        let default_theme: iced::Theme = (*self).into();
        scrollable::StyleSheet::active(&default_theme, &Default::default())
    }

    fn hovered(&self, _style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Appearance {
        let default_theme: iced::Theme = (*self).into();
        scrollable::StyleSheet::hovered(&default_theme, &Default::default(), is_mouse_over_scrollbar)
    }
}

// Menu Styles (for PickList)
impl overlay::menu::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        let default_theme: iced::Theme = (*self).into();
        overlay::menu::StyleSheet::appearance(&default_theme, &Default::default())
    }
}

// --- Helper conversion for delegation ---
impl From<AppTheme> for iced::Theme {
    fn from(theme: AppTheme) -> Self {
        match theme {
            AppTheme::Light => iced::Theme::Light,
            AppTheme::Dark => iced::Theme::Dark,
        }
    }
}