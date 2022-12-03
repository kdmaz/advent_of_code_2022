use super::day02_shared::{Choice, Outcome};
use std::str::FromStr;

fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let choices: Vec<&str> = line.split(' ').collect();
            let opponent_choice = Choice::from_str(choices[0]).unwrap();
            let response_choice = Choice::from_str(choices[1]).unwrap();
            let outcome = Outcome::from_choices(&opponent_choice, &response_choice);
            (response_choice as i32) + (outcome as i32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 2);
        let output = main(&input);
        let expected = 15;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 2);
        let output = main(&input);
        let expected = 12276;
        assert_eq!(output, expected);
    }
}
