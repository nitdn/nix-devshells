use iced::widget::{self};
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

fn evaluate(pairs: &mut Pairs<'_, Rule>) -> i64 {
    let expr = parse_expr(pairs.next().unwrap().into_inner());
    // eprintln!("Parsed: {expr:#?}");
    inorder_eval(&expr, 19)
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
        let items_list = self
            .inputs
            .iter()
            .map(|item| widget::text(item).into())
            .collect();
        let row = widget::row![
            // widget::Row::from_vec(items_list),
            widget::text_input("Type an equation...", &self.current_input)
                .id("Text Box")
                .on_input(Message::Update)
                .on_submit(Message::Submit),
            widget::button("Add").on_press(Message::Submit)
        ];
        let columns = widget::Column::from_vec(items_list);
        widget::container(columns.push(row)).into()
    }
}
