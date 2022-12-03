use super::day01_shared::sums;

fn main(input: &str) -> i32 {
    *sums(input).iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 1);
        let output = main(&input);
        let expected = 24000;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 1);
        let output = main(&input);
        let expected = 72602;
        assert_eq!(output, expected);
    }
}
