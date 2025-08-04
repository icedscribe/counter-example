use iced::{
    Element,
    Length::Fill,
    Size, Task,
    widget::{button, center, column, text},
};

#[derive(Debug, Default)]
struct App {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(app: &mut Self, message: Message) {
        match message {
            Message::Increment => {
                app.value += 1;
            }
            Message::Decrement => {
                app.value -= 1;
            }
        }
    }

    fn view(app: &Self) -> Element<Message> {
        let increment = button(center(text("+")))
            .width(Fill)
            .height(Fill)
            .on_press(Message::Increment);
        let decrement = button(center(text("-")))
            .width(Fill)
            .height(Fill)
            .on_press(Message::Decrement);

        let text = column!(center(text(app.value).size(30)));

        column![increment, text, decrement].spacing(20).into()
    }
}

fn main() -> iced::Result {
    iced::application("Counter", App::update, App::view)
        .window_size(Size::new(300.0, 600.0))
        .run_with(App::new)
}
