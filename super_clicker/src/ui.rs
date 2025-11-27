use iced::Element;
use iced::widget::{checkbox, column, pick_list, text, text_input};
use super::app::{Message, SuperClicker}; // Import Message and SuperClicker from app.rs
use iced::Theme; // Import Theme

pub fn view<'a>(
    status: &str,
    interval_input: &str,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
) -> Element<'a, Message, Theme> { // Corrected Renderer type
    column![
        text(format!("Status: {}", status)),
        text_input("Click Interval (ms):", interval_input)
            .on_input(Message::IntervalInputChanged),
        pick_list(
            vec!["Left".to_string(), "Middle".to_string(), "Right".to_string()],
            Some(mouse_button_selected),
            Message::MouseButtonSelected
        ),
            checkbox(
                "Enable Dynamic Interval Adjustment",
                enable_dynamic_adjustment,
            ).on_toggle(Message::DynamicAdjustmentToggled),
            iced::widget::button("Start").on_press(Message::Start),
            iced::widget::button("Stop").on_press(Message::Stop),
        ]
        .spacing(10)
        .into()
    }
