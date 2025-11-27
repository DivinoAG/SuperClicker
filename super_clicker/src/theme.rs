// super_clicker/src/theme.rs
// This file will contain our custom AppTheme and related style sheets.

use iced::{application, Border, Color};
use iced::widget::{button, checkbox, container, pick_list, text_input};

// Our custom theme enum which can wrap Iced's default Theme or
// define our own custom theme types. For now, we'll align it
// with Iced's default Light and Dark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
}

// Convert our AppTheme to Iced's default Theme.
impl From<AppTheme> for iced::Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => iced::Theme::Light,
            AppTheme::Dark => iced::Theme::Dark,
        }
    }
}

// --- Color Design Tokens ---
// These define the core colors for our application in both light and dark modes.
// They are based on the reference_ui.jpg and front-end-spec.md.

impl AppTheme {
    pub fn primary_background(self) -> Color {
        match self {
            AppTheme::Light => Color::WHITE, // Or a very light gray
            AppTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1), // Dark charcoal
        }
    }

    pub fn card_background(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.95, 0.95, 0.95), // Slightly darker than primary_background
            AppTheme::Dark => Color::from_rgb(0.15, 0.15, 0.15), // Slightly lighter than primary_background
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
            AppTheme::Light => Color::from_rgb(0.6, 0.6, 0.6), // Darker grey for light mode
            AppTheme::Dark => Color::from_rgb(0.4, 0.4, 0.4), // Lighter grey for dark mode
        }
    }

    pub fn accent_blue(self) -> Color {
        Color::from_rgb8(0, 120, 212) // #0078D4 - Windows accent blue
    }

    pub fn status_running_green(self) -> Color {
        Color::from_rgb8(76, 175, 80) // #4CAF50
    }

    pub fn status_stopped_grey(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb8(160, 160, 160), // #A0A0A0
            AppTheme::Dark => Color::from_rgb8(60, 60, 60), // Darker grey for consistency
        }
    }

    pub fn button_disabled_background(self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.8, 0.8, 0.8),
            AppTheme::Dark => Color::from_rgb(0.2, 0.2, 0.2),
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

// --- Spacing Design Tokens ---
pub const _SPACING_SM: f32 = 8.0;
pub const _SPACING_MD: f32 = 12.0;
pub const _SPACING_LG: f32 = 16.0;

// --- Custom Style Sheets for Widgets ---

// Container Styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)] // Derive Default
pub enum ContainerStyle {
    #[default]
    Card,
    StatusDisplay,
}

impl container::StyleSheet for AppTheme {
    type Style = ContainerStyle;

    fn appearance(&self, style: &Self::Style) -> container::Appearance { // Changed to &Self::Style
        match style {
            ContainerStyle::Card => container::Appearance {
                background: Some(self.card_background().into()),
                border: Border {
                    radius: 8.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
            ContainerStyle::StatusDisplay => container::Appearance {
                background: Some(self.status_stopped_grey().into()), // Default to stopped
                border: Border {
                    radius: 8.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
        }
    }
}

// Button Styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)] // Derive Default
pub enum ButtonStyle {
    #[default]
    Primary, // For Start/Stop buttons
    Status,  // For Status Display button
    Icon,    // For future icon-only buttons
}

impl button::StyleSheet for AppTheme {
    type Style = ButtonStyle;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.accent_blue().into()),
            border: Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: self.text_primary(),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.accent_blue().into()),
            text_color: self.text_primary(),
            ..self.active(style) // Pass reference
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(self.accent_blue().into()),
            text_color: self.text_primary(),
            ..self.active(style) // Pass reference
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance { // Changed to &Self::Style, _style
        button::Appearance {
            background: Some(self.button_disabled_background().into()),
            text_color: self.text_disabled(),
            ..Default::default()
        }
    }
}

// Text Input Styles
impl text_input::StyleSheet for AppTheme {
    type Style = (); // Default style

    fn active(&self, _style: &Self::Style) -> text_input::Appearance { // Changed to &Self::Style
        text_input::Appearance {
            background: self.input_background().into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.input_border(),
            },
            icon_color: Color::TRANSPARENT, // No icon for text input, use transparent color
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance { // Changed to &Self::Style
        text_input::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..self.active(&()).border // Pass reference
            },
            ..self.active(&()) // Pass reference
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance { // Changed to &Self::Style
        text_input::Appearance {
            background: self.button_disabled_background().into(), // Re-use button disabled background
            border: Border {
                color: self.text_disabled(),
                ..self.active(&()).border // Pass reference
            },
            ..self.active(&()) // Pass reference
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color { // Changed to &Self::Style
        self.text_disabled()
    }

    fn value_color(&self, _style: &Self::Style) -> Color { // Changed to &Self::Style
        self.text_primary()
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color { // Changed to &Self::Style
        self.text_disabled()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color { // Changed to &Self::Style
        self.accent_blue()
    }
}

// Pick List Styles
impl pick_list::StyleSheet for AppTheme {
    type Style = (); // Default style

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

    fn hovered(&self, _style: &Self::Style) -> pick_list::Appearance {
        let active_appearance = self.active(_style);
        pick_list::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..active_appearance.border
            },
            placeholder_color: active_appearance.placeholder_color,
            background: active_appearance.background,
            text_color: active_appearance.text_color,
            handle_color: active_appearance.handle_color,
        }
    }
}

// Checkbox Styles
impl checkbox::StyleSheet for AppTheme {
    type Style = (); // Default style

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance { // Changed to &Self::Style, _is_checked
        checkbox::Appearance {
            background: self.input_background().into(),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: self.input_border(),
            },
            icon_color: self.accent_blue(), // Icon color when checked
            text_color: Some(self.text_primary()),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance { // Added missing hovered
        checkbox::Appearance {
            border: Border {
                color: self.accent_blue(),
                ..self.active(&(), _is_checked).border
            },
            ..self.active(&(), _is_checked)
        }
    }

    fn disabled(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance { // Changed to &Self::Style, _is_checked
        checkbox::Appearance {
            background: self.button_disabled_background().into(),
            border: Border {
                color: self.text_disabled(),
                ..self.active(&(), _is_checked).border
            },
            icon_color: self.text_disabled(), // Disabled icon color
            text_color: Some(self.text_disabled()),
        }
    }
}

// Application theme for Iced.
impl application::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.primary_background(), // Use background_color
            text_color: self.text_primary(),
        }
    }
}

