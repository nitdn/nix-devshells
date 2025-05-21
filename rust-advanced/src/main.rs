use iced::{
    Element,
    Length::Fill,
    Theme,
    widget::{button, container, row, text},
};

use template::{Message, update};

fn view(value: &u64) -> Element<Message> {
    let decrement = button(container("-").center(Fill)).width(100).height(Fill);
    let increment = button(container("+").center(Fill)).width(100).height(Fill);
    container(
        row![
            if *value > 0 {
                decrement.on_press(Message::Decrement)
            } else {
                decrement
            },
            container(text(value).size(10 * 10)).center(Fill),
            if *value < 9 {
                increment.on_press(Message::Increment)
            } else {
                increment
            },
        ]
        .spacing(10)
        .padding(10)
        .width(400)
        .height(200),
    )
    .center(Fill)
    .into()
}
fn theme<T>(_: &T) -> Theme {
    Theme::TokyoNight
}

pub fn main() -> iced::Result {
    iced::application(|| 1u64, update, view)
        .theme(theme)
        .title("My cool program")
        .run()
}
