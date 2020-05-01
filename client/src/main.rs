use orbtk::{Window, Rect, TextBox, Place, Text, Enter};
use std::borrow::Borrow;

fn main() {

    let mut window = Window::new(Rect::new(100, 100, 1280, 720), "Drocsid");

    let x = 0u32;
    let mut y = 10u32;

    let chat_box = TextBox::new();
    chat_box.position(x as i32, y as i32)
        .size(400, 400)
        .text_offset(1,1);
    window.add(&chat_box);

    y += chat_box.rect.get().height + 15;

    let text_input = TextBox::new();
    text_input.position(x as i32, y as i32)
        .size(100, 30)
        .text_offset(6,6)
        .on_enter(move |tb: &TextBox| {
            chat_box.text(&tb.text.get());
        });
    window.add(&text_input);

    window.exec();
}