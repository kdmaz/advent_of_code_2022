// calories summed for each elf
pub fn sums(input: &str) -> Vec<i32> {
    let mut sums = vec![];

    for calories_per_elf in input.split("\n\n") {
        let sum = calories_per_elf
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        sums.push(sum);
    }

    sums
}

pub fn part1(input: &str) -> i32 {
    *sums(input).iter().max().unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut sums = sums(input);
    sums.sort();
    sums.reverse();
    sums[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::read_file;

    #[test]
    fn part1_example() {
        let input = read_file("examples", 1);
        let output = part1(&input);
        let expected = 24000;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = read_file("input", 1);
        let output = part1(&input);
        let expected = 72602;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_example() {
        let input = read_file("examples", 1);
        let output = part2(&input);
        let expected = 45000;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = read_file("input", 1);
        let output = part2(&input);
        let expected = 207410;
        assert_eq!(output, expected);
    }
}
