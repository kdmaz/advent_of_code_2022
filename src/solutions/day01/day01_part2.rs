use super::day01_shared::sums;

fn main(input: &str) -> i32 {
    let mut sums = sums(input);
    sums.sort();
    sums.reverse();
    sums[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 1);
        let output = main(&input);
        let expected = 45000;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 1);
        let output = main(&input);
        let expected = 207410;
        assert_eq!(output, expected);
    }
}
