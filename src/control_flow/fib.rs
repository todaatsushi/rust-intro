pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn test_base_case() {
        assert_eq!(0, fibonacci(0));
        assert_eq!(1, fibonacci(1));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(55, fibonacci(10));
    }
}
