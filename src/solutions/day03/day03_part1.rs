use std::collections::HashSet;
use super::day03_shared::get_priority;


fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line_len = line.len();
            let mut first_half = HashSet::new();
            let mut second_half = HashSet::new();

            for (i, c) in line.chars().enumerate() {
                if i < line_len / 2 {
                    first_half.insert(c);
                } else {
                    second_half.insert(c);
                }
            }

            let intersection: Vec<_> = first_half.intersection(&second_half).collect();
            let c = **intersection.first().unwrap();
            get_priority(c)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 3);
        let output = main(&input);
        let expected = 157;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 3);
        let output = main(&input);
        let expected = 7831;
        assert_eq!(output, expected);
    }
}
