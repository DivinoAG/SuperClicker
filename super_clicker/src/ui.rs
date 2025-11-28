use iced::widget::{button, checkbox, column, container, pick_list, text, text_input, Row, Space};
use iced::{Alignment, Element, Length};

use super::app::Message;
use super::theme::{self, AppTheme, TextStyle};

pub fn view(
    status: &str,
    interval_input: &str,
    mouse_button_selected: String,
    enable_dynamic_adjustment: bool,
    is_running: bool,
) -> Element<'static, Message, AppTheme> {
    // --- Status ---
    let status_display = container(text(format!("Status: {}", status)).size(20))
        .padding(10)
        .center_x()
        .width(Length::Fill)
        .style(if is_running {
            theme::ContainerStyle::StatusDisplayRunning
        } else {
            theme::ContainerStyle::StatusDisplay
        });

    // --- Primary Controls ---
    let primary_controls_title = text("Primary Controls").size(18);

    let interval_control: Element<_, _, _> = if is_running {
        Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text("Click Interval (ms):").style(TextStyle::Disabled))
            .push(Space::with_width(Length::Fill))
            .push(
                container(text(interval_input))
                    .style(theme::ContainerStyle::ReadOnly)
                    .padding(10)
                    .width(Length::Fixed(80.0))
            )
            .into()
    } else {
        Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text("Click Interval (ms):"))
            .push(Space::with_width(Length::Fill))
            .push(text_input("ms", interval_input)
                .on_input(Message::IntervalInputChanged)
                .padding(10)
                .width(Length::Fixed(80.0)))
            .into()
    };

    let mouse_button_control: Element<_, _, _> = if is_running {
        Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text("Mouse Button:").style(TextStyle::Disabled))
            .push(Space::with_width(Length::Fill))
            .push(
                container(text(mouse_button_selected))
                    .style(theme::ContainerStyle::ReadOnly)
                    .padding(10)
                    .width(Length::Fixed(120.0))
            )
            .into()
    } else {
        Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(text("Mouse Button:"))
            .push(Space::with_width(Length::Fill))
            .push(pick_list(
                vec!["Left".to_string(), "Middle".to_string(), "Right".to_string()],
                Some(mouse_button_selected),
                Message::MouseButtonSelected,
            ).padding(10).width(Length::Fixed(120.0)))
            .into()
    };
    
    let dynamic_adj_checkbox = checkbox(
        "Enable Dynamic Interval Adjustment",
        enable_dynamic_adjustment,
    );
    
    let dynamic_adj_checkbox_styled = if is_running {
        dynamic_adj_checkbox
    } else {
        dynamic_adj_checkbox.on_toggle(Message::DynamicAdjustmentToggled)
    };

    let (start_btn_content, stop_btn_content) = if is_running {
        (
            text("Start").style(TextStyle::Disabled),
            text("Stop").style(TextStyle::Accent),
        )
    } else {
        (
            text("Start").style(TextStyle::Accent),
            text("Stop").style(TextStyle::Disabled),
        )
    };

    let start_btn = button(start_btn_content.width(Length::Fill))
        .style(theme::ButtonStyle::Primary)
        .padding([10, 20]);
    let stop_btn = button(stop_btn_content.width(Length::Fill))
        .style(theme::ButtonStyle::Primary)
        .padding([10, 20]);
    
    let (start_btn, stop_btn) = if is_running {
        (start_btn, stop_btn.on_press(Message::Stop))
    } else {
        (start_btn.on_press(Message::Start), stop_btn)
    };

    let action_buttons = Row::new()
        .spacing(10)
        .push(start_btn)
        .push(stop_btn);

    // --- Assembly ---
    let main_controls_card = container(
        column![
            primary_controls_title,
            interval_control,
            mouse_button_control,
            dynamic_adj_checkbox_styled,
        ]
        .spacing(15),
    )
    .style(theme::ContainerStyle::Card)
    .padding(15);
    
    // Final Column
    container(
        column![status_display, main_controls_card, action_buttons]
            .spacing(20)
            .padding(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .into()
}
