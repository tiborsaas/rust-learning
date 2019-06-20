pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sanity() {
        if 1 != 1 {
            panic!("Make this test fail");
        }
    }

    #[test]
    fn test_adder() {
        assert_eq!(add_two(10), 12);
    }
}
