use iced::{
    Length::Fill,
    Task,
    widget::{self, vertical_space},
};
use pest::Parser;

use crate::{Expr, ExprParser, Rule, inorder_eval, parse_expr};

#[derive(Debug)]
pub struct Inputs {
    current_input: String,
    current_x_val: f32,
    inputs: Vec<(String, Expr, f32)>,
    is_graphing: bool,
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
            current_input: Default::default(),
            current_x_val: Default::default(),
            inputs: Default::default(),
            is_graphing: true,
            pixels,
            scale: 1,
            x_pan: Default::default(),
            y_pan: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Slider(f32),
    StoreX(f32),
    Submit,
    ToggleGraphing(bool),
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
                let x_origin = RESOLUTION as f32 / 2.0;
                let y_origin = RESOLUTION as f32 / 2.0;
                let x_domain = (x_coord as f32 - x_origin - x_pan) / scale as f32;
                let y_domain = (y_coord as f32 - y_origin - y_pan) / scale as f32;
                if predicate(x_domain, y_domain, scale) {
                    pixel[0] = 200;
                    pixel[1] = 0;
                    pixel[2] = 0;
                    pixel[3] = 255;
                } else if x_domain == 0.0 || y_domain == 0.0 {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                    pixel[3] = 100;
                } else {
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = 255;
                }
            }
        });
}

impl Inputs {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        let task = match message {
            Message::Update(text) => {
                self.current_input = text;
                return Task::none();
            }
            Message::Submit => {
                if let Ok(mut pairs) = ExprParser::parse(Rule::equation, &self.current_input) {
                    let is_x = pairs
                        .clone()
                        .flatten()
                        .any(|pair| pair.as_rule() == Rule::var_x);
                    let expr = parse_expr(pairs.next().unwrap().into_inner());
                    let value = inorder_eval(&expr, self.current_x_val);

                    let input = if is_x {
                        format!(
                            "{} at x = {} is {}",
                            self.current_input.clone(),
                            self.current_x_val,
                            value
                        )
                    } else {
                        format!("{} = {}", self.current_input.clone(), value)
                    };
                    self.inputs.push((input, expr, value));
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

            Message::StoreX(value) => {
                self.current_x_val = value;
                return Task::none();
            }
            Message::ToggleGraphing(is_graphing) => {
                self.is_graphing = is_graphing;
                Task::none()
            }
        };
        if self.is_graphing {
            self.render_update()
        };
        // widget::text_input::focus("Text Box")
        task
    }

    fn render_update(&mut self) {
        paint_pixel(
            &mut self.pixels,
            self.x_pan,
            self.y_pan,
            self.scale,
            |x, y, scale| {
                self.inputs.iter().any(|(_, expr, _)| {
                    let eval_x_0 = inorder_eval(expr, x);
                    let x_1 = x + 1.5 / scale as f32;
                    let eval_x_1 = inorder_eval(expr, x_1);
                    (eval_x_1 - eval_x_0).abs() >= (eval_x_1 - y).abs()
                })
            },
        )
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        let items_list = self.inputs.iter().map(|item| {
            widget::row![
                widget::text(&item.0).size(FONT_SIZE),
                widget::horizontal_space(),
                widget::button(widget::text!("Store into x").size(FONT_SIZE))
                    .on_press(Message::StoreX(item.2))
            ]
            .into()
        });
        // let items_list = [];

        let scrollable_items_list =
            widget::container(widget::scrollable(widget::column(items_list))).max_height(600);
        let text_input_row = widget::container(widget::row![
            // widget::Row::from_vec(items_list),
            widget::text_input("Type an equation...", &self.current_input)
                .id("Text Box")
                .size(FONT_SIZE)
                .on_input(Message::Update)
                .on_submit(Message::Submit),
            widget::button(
                widget::text!("x = {} (Click to reset)", self.current_x_val).size(FONT_SIZE)
            )
            .on_press(Message::StoreX(Default::default())),
            widget::button(widget::text!("Evaluate").size(FONT_SIZE)).on_press(Message::Submit),
        ]);
        // let columns = widget::column![];

        let columns = if self.is_graphing {
            widget::column![
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
                    widget::button(widget::text!("Zoom +").size(FONT_SIZE))
                        .on_press(Message::ZoomIn),
                    widget::button(widget::text!("Zoom -").size(FONT_SIZE))
                        .on_press(Message::ZoomOut)
                ]
            ]
        } else {
            widget::column![vertical_space()]
        }
        .push(
            widget::toggler(self.is_graphing)
                .label("Toggle Graphing")
                .text_size(FONT_SIZE)
                .on_toggle(Message::ToggleGraphing),
        )
        .height(Fill);
        widget::container(columns.push(scrollable_items_list).push(text_input_row)).into()
    }
}
