pub fn part1(input: &str) -> u32 {
    println!("{}", input);
    0
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let input = read_file("examples", 1);
        let output = part1(&input);
        let expected = 24000;
        assert_eq!(output, expected);
    }
    
    #[test]
    fn part2_test() {
        let input = read_file("examples", 2);
        let output = part2(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }
}