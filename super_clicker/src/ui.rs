use iced::Element;
use iced::widget::{checkbox, column, pick_list, text, text_input, button};
use super::app::Message;
use iced::Theme;

pub fn view<'a>(
    status: &str,
    interval_input: &str,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
) -> Element<'a, Message, Theme> {
    let start_btn: Element<Message> = if is_running {
        button("Start").into()
    } else {
        button("Start").on_press(Message::Start).into()
    };

    let stop_btn: Element<Message> = if !is_running {
        button("Stop").into()
    } else {
        button("Stop").on_press(Message::Stop).into()
    };

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
        start_btn,
        stop_btn,
    ]
    .spacing(10)
    .into()
}
