use std::sync::Arc;

use iced::{
    Length::{self, Fill, FillPortion},
    advanced::graphics::image::image_rs::math,
    widget::{
        self,
        scrollable::{Scrollbar, Viewport},
        text::LineHeight,
    },
};
use pest::{Parser, iterators::Pairs};

use crate::{ExprParser, Rule, inorder_eval, parse_expr};

#[derive(Debug, Default)]
pub struct Inputs {
    inputs: Vec<String>,
    current_input: String,
    x_pan: f32,
    y_pan: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
    Submit,
    Slider(f32),
}
const RESOLUTION: usize = 1024;
const FONT_SIZE: u16 = 24;

fn evaluate(pairs: &mut Pairs<'_, Rule>) -> i64 {
    let expr = parse_expr(pairs.next().unwrap().into_inner());
    // eprintln!("Parsed: {expr:#?}");
    inorder_eval(&expr, 19)
}
fn paint_pixel<T: Fn(usize, usize) -> bool>(pixels: &mut [u8], predicate: T) {
    pixels
        .rchunks_exact_mut(RESOLUTION * 4)
        .enumerate()
        .filter_map(|(y_coord, line)| {
            for (x_coord, pixel) in line.chunks_exact_mut(4).enumerate() {
                if predicate(x_coord, y_coord) {
                    return Some(pixel);
                }
            }
            None
        })
        .for_each(|pixel| {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 0;
        });
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
            Message::Slider(slider) => {
                self.x_pan = slider;
            }
        }
        widget::text_input::focus("Text Box")
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let items_list = self
            .inputs
            .iter()
            .map(|item| widget::text(item).size(FONT_SIZE).into());

        let row = widget::container(widget::row![
            // widget::Row::from_vec(items_list),
            widget::text_input("Type an equation...", &self.current_input)
                .id("Text Box")
                .size(FONT_SIZE)
                .on_input(Message::Update)
                .on_submit(Message::Submit),
            widget::button(widget::text!("Add").size(FONT_SIZE)).on_press(Message::Submit)
        ]);
        let mut pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];
        paint_pixel(&mut pixels, |x, y| {
            // let Some(equation) = self.inputs.first() else {
            //     return false;
            // };
            let x_origin = RESOLUTION / 2;
            let y_origin = RESOLUTION / 2;
            let domain_x = (x as f32 - x_origin as f32) - self.x_pan;
            let domain_y = y as f32 - y_origin as f32;
            let eval_x = domain_x.powi(2);
            (domain_y - eval_x).abs() < 512.0
        });
        // let columns = widget::column![];

        let columns = widget::column![
            widget::image(widget::image::Handle::from_rgba(
                RESOLUTION as u32,
                RESOLUTION as u32,
                pixels
            ))
            .height(Fill),
            widget::slider(0.0..=100.0, self.x_pan, Message::Slider)
        ];
        widget::container(columns.extend(items_list).push(row)).into()
    }
}
