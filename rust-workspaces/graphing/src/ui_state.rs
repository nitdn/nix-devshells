use std::sync::Arc;

use iced::{
    Length::{self, Fill, FillPortion},
    widget,
};
use pest::{Parser, iterators::Pairs};

use crate::{ExprParser, Rule, inorder_eval, parse_expr};

#[derive(Debug, Default)]
pub struct Inputs {
    inputs: Vec<String>,
    current_input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
    Submit,
}
const RESOLUTION: usize = 1024;

fn evaluate(pairs: &mut Pairs<'_, Rule>) -> i64 {
    let expr = parse_expr(pairs.next().unwrap().into_inner());
    // eprintln!("Parsed: {expr:#?}");
    inorder_eval(&expr, 19)
}
fn paint<T: Fn(usize) -> usize>(pixels: &mut [u8], predicate: T) {
    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let pixel_index = (x + (RESOLUTION - y - 1) * RESOLUTION) * 4;
            if y == predicate(x) {
                pixels[pixel_index] = 0;
                pixels[pixel_index + 1] = 0;
                pixels[pixel_index + 2] = 0;
                pixels[pixel_index + 3] = 0;
            }
        }
    }
}

impl Inputs {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Update(text) => {
                eprintln!("{text}");
                self.current_input = text;
            }
            Message::Submit => {
                if let Ok(pairs) = ExprParser::parse(Rule::equation, &self.current_input) {
                    let input = format!(
                        "{} = {}",
                        self.current_input.clone(),
                        evaluate(&mut pairs.clone())
                    );
                    self.inputs.push(input);
                }
            }
        }
        widget::text_input::focus("Text Box")
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let items_list = self.inputs.iter().map(|item| widget::text(item).into());

        let row = widget::row![
            // widget::Row::from_vec(items_list),
            widget::text_input("Type an equation...", &self.current_input)
                .id("Text Box")
                .on_input(Message::Update)
                .on_submit(Message::Submit)
                .line_height(25.0),
            widget::button("Add").on_press(Message::Submit)
        ];
        let mut pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];
        paint(&mut pixels, |x| x);
        let columns = widget::column![];

        // let columns = widget::column![widget::image(widget::image::Handle::from_rgba(
        //     RESOLUTION as u32,
        //     RESOLUTION as u32,
        //     pixels
        // ))];
        widget::container(columns.extend(items_list).push(row)).into()
    }
}
