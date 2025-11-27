use iced::Element;
use iced::widget::{checkbox, column, pick_list, text, text_input, Button};
use super::app::Message;
use iced::Theme;
use iced::Color;

pub fn view<'a>(
    status: &str,
    interval_input: &str,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
) -> Element<'a, Message, Theme> {
    let start_btn: Element<Message> = if is_running {
        Button::new(text("Start")).into()
    } else {
        Button::new(text("Start")).on_press(Message::Start).into()
    };

    let stop_btn: Element<Message> = if !is_running {
        Button::new(text("Stop")).into()
    } else {
        Button::new(text("Stop")).on_press(Message::Stop).into()
    };

    let status_color = if is_running {
        Color::from_rgb(0.0, 0.8, 0.0)
    } else {
        Color::from_rgb(0.5, 0.5, 0.5)
    };

    column![
        text(format!("Status: {}", status))
            .size(20)
            .style(status_color),
        text_input("Click Interval (ms):", interval_input)
            .on_input(Message::IntervalInputChanged)
            .padding(5),
        pick_list(
            vec!["Left".to_string(), "Middle".to_string(), "Right".to_string()],
            Some(mouse_button_selected),
            Message::MouseButtonSelected
        ),
        checkbox(
            "Enable Dynamic Interval Adjustment",
            enable_dynamic_adjustment,
        ).on_toggle(Message::DynamicAdjustmentToggled),
        start_btn,
        stop_btn,
    ]
    .spacing(15)
    .padding(20)
    .into()
}
