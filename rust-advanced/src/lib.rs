use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Decrement,
    Increment,
}

pub trait Sides {}
pub trait Angles {}

pub struct Quad<T> {
    pub equalities: PhantomData<T>,
}

impl<T> Quad<T> {
    pub fn new() -> Quad<T> {
        Self {
            equalities: PhantomData,
        }
    }
}

impl<T> Default for Quad<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<Kite: Sides> Quad<Kite> {
    pub fn is_kite(&self) -> bool {
        true
    }
}
impl<Rectangle: Angles> Quad<Rectangle> {
    pub fn is_rectangle(&self) -> bool {
        true
    }
}
impl<Square: Sides + Angles> Quad<Square> {
    pub fn is_square(&self) -> bool {
        true
    }
}

#[derive(PartialEq, Debug)]
pub struct Test(u64);

pub fn create_test(num: u64) -> Test {
    if num > 10 { Test(10) } else { Test(num) }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn update(num: &mut u64, message: Message) {
    match message {
        Message::Increment => *num = checked_increment(*num),
        Message::Decrement => *num = num.saturating_sub(1),
    }
}

fn checked_increment(num: u64) -> u64 {
    if num < 10 { num + 1 } else { num }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_created_test() {
        assert_eq!(create_test(43), Test(43));
    }
}
