pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn test_random_spaces() -> u64 {
    if true {
        todo!()
    }
    todo!()
}

pub fn test_pattern_matching((1 | 2 | 3 | _): u64) {
    // why did i do this
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
