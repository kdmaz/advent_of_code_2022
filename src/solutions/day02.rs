pub fn part1(_input: &str) -> i32 {
    -1
}

pub fn part2(_input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::read_file;

    #[test]
    fn part1_example() {
        let input = read_file("examples", 2);
        let output = part1(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = read_file("input", 2);
        let output = part1(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_example() {
        let input = read_file("examples", 2);
        let output = part2(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = read_file("input", 2);
        let output = part2(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }
}
