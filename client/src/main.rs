extern crate iced;

use iced::{text_input, canvas, Application, Command, Element, Length, Color, HorizontalAlignment, VerticalAlignment, TextInput, Container, Column, Row, Settings, Scrollable, Canvas, Align};
use crate::Message::{InputChanged, InputSubmitted};
use std::borrow::BorrowMut;

#[derive(Debug, Default)]
struct Chat {
    input: text_input::State,
    input_value: String,
    text_messages: Vec<iced::Text>,
    needs_update: bool
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    InputSubmitted(String)
}

impl Application for Chat {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Chat {
            input: text_input::State::default(),
            input_value: String::from(""),
            text_messages: Vec::new(),
            needs_update: false
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Drocsid")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            InputChanged(value) => {
                self.input_value = value;
            },
            InputSubmitted(value) => {
                self.input_value = String::from("");
                self.text_messages.push(iced::Text::new(value));
                self.needs_update = true
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = iced::Text::new("Drocsid")
            .width(Length::Shrink)
            .size(32)
            .color(Color::from_rgb8(148, 0, 211))
            .horizontal_alignment(HorizontalAlignment::Left).vertical_alignment(VerticalAlignment::Top);

        let input = TextInput::new(
            self.input.borrow_mut(),
            "What needs to be done?",
            self.input_value.as_ref(),
            Message::InputChanged,
        )
            .padding(15)
            .size(30)
            .on_submit(Message::InputSubmitted(String::from(&self.input_value)));

        let chat_messages = self.text_messages.iter().fold(Column::new().spacing(10),
        |column, txt| {
            column.push(txt.clone())
        },);

        let content = Column::<Message>::new()
            .max_width(800)
            .spacing(20)
            .push(chat_messages);

        let con = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Start)
            .center_y();

        Column::new().align_items(Align::Start).push(title).push(con).push(input).into()
    }
}


fn main() {
    Chat::run(Settings::default())
}


