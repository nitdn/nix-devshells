use iced::{
    Length::Fill,
    Task,
    widget::{self},
};
use pest::Parser;

use crate::{Expr, ExprParser, Rule, inorder_eval, parse_expr};

#[derive(Debug)]
pub struct Inputs {
    current_input: String,
    inputs: Vec<(String, Expr)>,
    pixels: Vec<u8>,
    scale: u64,
    x_pan: f32,
    y_pan: f32,
}

impl Default for Inputs {
    fn default() -> Self {
        let mut pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];
        paint_pixel(&mut pixels, 0.0, 0.0, 1, |_, _, _| false); // set up axes

        Self {
            inputs: Default::default(),
            current_input: Default::default(),
            x_pan: Default::default(),
            y_pan: Default::default(),
            scale: 1,
            pixels,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Slider(f32),
    Submit,
    Update(String),
    VerticalSlider(f32),
    ZoomIn,
    ZoomOut,
}
const RESOLUTION: usize = 1024;
const FONT_SIZE: u16 = 24;
fn paint_pixel<T: Fn(f32, f32, u64) -> bool>(
    pixels: &mut [u8],
    x_pan: f32,
    y_pan: f32,
    scale: u64,
    predicate: T,
) {
    pixels
        .rchunks_exact_mut(RESOLUTION * 4)
        .enumerate()
        .map(|(y_coord, line)| (y_coord, line.chunks_mut(4).enumerate()))
        .for_each(|(y_coord, line)| {
            for (x_coord, pixel) in line {
                let x_origin = RESOLUTION as i64 / 2;
                let y_origin = RESOLUTION as i64 / 2;
                let x_domain = (x_coord as f32 - x_origin as f32 - x_pan) / scale as f32;
                let y_domain = (y_coord as f32 - y_origin as f32 - y_pan) / scale as f32;
                if x_domain == 0.0 || y_domain == 0.0 {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                    pixel[3] = 100;
                    continue;
                }
                if predicate(x_domain, y_domain, scale) {
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
                return Task::none();
            }
            Message::Submit => {
                if let Ok(mut pairs) = ExprParser::parse(Rule::equation, &self.current_input) {
                    let expr = parse_expr(pairs.next().unwrap().into_inner());
                    // eprintln!("Parsed: {expr:#?}");
                    let value = inorder_eval(&expr, 0.0);

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
            Message::VerticalSlider(slider) => {
                self.y_pan = slider;
                Task::none()
            }
            Message::ZoomIn => {
                self.scale += 5;
                Task::none()
            }
            Message::ZoomOut => {
                self.scale = self.scale.saturating_sub(5).max(1);
                Task::none()
            }
        };
        self.pixels = vec![255u8; RESOLUTION * RESOLUTION * 4];

        paint_pixel(
            &mut self.pixels,
            self.x_pan,
            self.y_pan,
            self.scale,
            |_, _, _| false,
        ); // clean previous painting
        for (_, expr) in &self.inputs {
            paint_pixel(
                &mut self.pixels,
                self.x_pan,
                self.y_pan,
                self.scale,
                |x, y, scale| {
                    let eval_x_0 = inorder_eval(expr, x);
                    let x_1 = x + 1.0 / scale as f32;
                    let eval_x_1 = inorder_eval(expr, x_1);
                    (eval_x_1 - eval_x_0).abs() >= (eval_x_1 - y).abs()
                },
            )
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
            widget::row![
                widget::image(widget::image::Handle::from_rgba(
                    RESOLUTION as u32,
                    RESOLUTION as u32,
                    self.pixels.clone()
                ))
                .width(Fill)
                .height(Fill),
                widget::vertical_slider(
                    -(RESOLUTION as f32 / 2.0)..=RESOLUTION as f32 / 2.0,
                    self.y_pan,
                    Message::VerticalSlider
                )
            ],
            widget::slider(
                -(RESOLUTION as f32 / 2.0)..=RESOLUTION as f32 / 2.0,
                self.x_pan,
                Message::Slider
            ),
            widget::row![
                widget::button(widget::text!("Zoom +").size(FONT_SIZE)).on_press(Message::ZoomIn),
                widget::button(widget::text!("Zoom -").size(FONT_SIZE)).on_press(Message::ZoomOut)
            ]
        ];
        widget::container(columns.extend(items_list).push(row)).into()
    }
}
