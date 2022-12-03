use super::day02_shared::{Choice, Outcome};
use std::str::FromStr;

impl Choice {
    fn from_outcome_needed(opponent_choice: &Choice, outcome_needed: Outcome) -> Self {
        if outcome_needed == Outcome::Draw {
            return *opponent_choice;
        }

        match (opponent_choice, outcome_needed) {
            (Choice::Scissors, Outcome::Win) | (Choice::Paper, Outcome::Loss) => Choice::Rock,
            (Choice::Rock, Outcome::Win) | (Choice::Scissors, Outcome::Loss) => Choice::Paper,
            _ => Choice::Scissors,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid choice".to_owned()),
        }
    }
}

pub fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let choices: Vec<&str> = line.split(' ').collect();
            let opponent_choice = Choice::from_str(choices[0]).unwrap();
            let outcome_needed = Outcome::from_str(choices[1]).unwrap();
            let response_choice = Choice::from_outcome_needed(&opponent_choice, outcome_needed);
            let match_outcome = Outcome::from_choices(&opponent_choice, &response_choice);
            (response_choice as i32) + (match_outcome as i32)
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
        let expected = 12;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 2);
        let output = main(&input);
        let expected = 9975;
        assert_eq!(output, expected);
    }
}
