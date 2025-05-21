#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
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
}
