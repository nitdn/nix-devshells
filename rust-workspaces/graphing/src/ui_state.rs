use iced::{
    Length::Fill,
    Task,
    widget::{self},
};
use pest::Parser;

use crate::{Expr, ExprParser, Rule, inorder_eval, parse_expr};

#[derive(Debug)]
pub struct Inputs {
    inputs: Vec<(String, Expr)>,
    current_input: String,
    x_pan: f32,
    y_pan: f32,
    pixels: Vec<u8>,
}

impl Default for Inputs {
    fn default() -> Self {
        let mut pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];
        paint_pixel(&mut pixels, 0, |_, _| false); // set up axes

        Self {
            inputs: Default::default(),
            current_input: Default::default(),
            x_pan: Default::default(),
            y_pan: Default::default(),
            pixels,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
    Submit,
    Slider(f32),
}
const RESOLUTION: usize = 1024;
const FONT_SIZE: u16 = 24;
fn paint_pixel<T: Fn(i64, i64) -> bool>(pixels: &mut [u8], x_pan: i64, predicate: T) {
    pixels
        .rchunks_exact_mut(RESOLUTION * 4)
        .enumerate()
        .map(|(y_coord, line)| (y_coord, line.chunks_mut(4).enumerate()))
        .for_each(|(y_coord, line)| {
            for (x_coord, pixel) in line {
                let x_origin = RESOLUTION as i64 / 2;
                let y_origin = RESOLUTION as i64 / 2;
                let x_domain = x_coord as i64 - x_origin - x_pan;
                let y_domain = y_coord as i64 - y_origin;
                if x_domain == 0 || y_domain == 0 {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                    pixel[3] = 100;
                }
                if predicate(x_domain, y_domain) {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                    pixel[3] = 255;
                }
            }
        });
}

impl Inputs {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        let task = match message {
            Message::Update(text) => {
                eprintln!("{text}");
                self.current_input = text;
                Task::none()
            }
            Message::Submit => {
                if let Ok(mut pairs) = ExprParser::parse(Rule::equation, &self.current_input) {
                    let expr = parse_expr(pairs.next().unwrap().into_inner());
                    // eprintln!("Parsed: {expr:#?}");
                    let value = inorder_eval(&expr, 0);

                    let input = if self.current_input.contains('x') {
                        format!("{} at x = 0 is {}", self.current_input.clone(), value)
                    } else {
                        format!("{} = {}", self.current_input.clone(), value)
                    };
                    self.inputs.push((input, expr));
                }
                widget::text_input::focus("Text Box")
            }
            Message::Slider(slider) => {
                self.x_pan = slider;
                Task::none()
            }
        };
        self.pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];

        paint_pixel(&mut self.pixels, self.x_pan as i64, |_, _| false); // clean previous painting
        for (_, expr) in &self.inputs {
            paint_pixel(&mut self.pixels, self.x_pan as i64, |x, y| {
                let eval_x_0 = inorder_eval(expr, x);
                let x_1 = x + 1;
                let eval_x_1 = inorder_eval(expr, x_1);
                (eval_x_1 - eval_x_0).abs() >= (eval_x_1 - y).abs()
            })
        }
        // widget::text_input::focus("Text Box")
        task
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let items_list = self
            .inputs
            .iter()
            .map(|item| widget::text(&item.0).size(FONT_SIZE).into());

        let row = widget::container(widget::row![
            // widget::Row::from_vec(items_list),
            widget::text_input("Type an equation...", &self.current_input)
                .id("Text Box")
                .size(FONT_SIZE)
                .on_input(Message::Update)
                .on_submit(Message::Submit),
            widget::button(widget::text!("Add").size(FONT_SIZE)).on_press(Message::Submit)
        ]);
        // let columns = widget::column![];

        let columns = widget::column![
            widget::scrollable(widget::image(widget::image::Handle::from_rgba(
                RESOLUTION as u32,
                RESOLUTION as u32,
                self.pixels.clone()
            )))
            .height(Fill),
            widget::slider(
                -(RESOLUTION as f32 / 2.0)..=RESOLUTION as f32 / 2.0,
                self.x_pan,
                Message::Slider
            )
        ];
        widget::container(columns.extend(items_list).push(row)).into()
    }
}
