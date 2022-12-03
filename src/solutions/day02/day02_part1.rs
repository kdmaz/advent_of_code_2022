use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Invalid choice".to_owned()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum MatchOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl MatchOutcome {
    fn from_choices(opponent_choice: &Choice, response_choice: &Choice) -> Self {
        if opponent_choice == response_choice {
            return MatchOutcome::Draw;
        }

        match (opponent_choice, response_choice) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Scissors, Choice::Rock)
            | (Choice::Paper, Choice::Scissors) => MatchOutcome::Win,
            (_, _) => MatchOutcome::Loss,
        }
    }

    fn from_response_letter(s: &str) -> Self {
        match s {
            "X" => MatchOutcome::Loss,
            "Y" => MatchOutcome::Draw,
            "Z" => MatchOutcome::Win,
            _ => panic!("Expected 'X', 'Y', or 'Z', but found {}", s),
        }
    }
}

fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let choices: Vec<&str> = line.split(" ").collect();
            let opponent_choice = Choice::from_str(choices[0]).unwrap();
            let response_choice = Choice::from_str(choices[1]).unwrap();
            let match_outcome = MatchOutcome::from_choices(&opponent_choice, &response_choice);
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
