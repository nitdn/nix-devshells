pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn fib(n: u64) -> u128 {
    (0..n).fold((0, 1), |(a, b), _| (a + b, a)).0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 42);
    }
}
