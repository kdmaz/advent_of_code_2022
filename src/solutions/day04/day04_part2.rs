fn main(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 4);
        let output = main(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 4);
        let output = main(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }
}
