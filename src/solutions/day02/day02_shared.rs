use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Choice {
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    pub fn from_choices(opponent_choice: &Choice, response_choice: &Choice) -> Self {
        if opponent_choice == response_choice {
            return Self::Draw;
        }

        match (opponent_choice, response_choice) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Scissors, Choice::Rock)
            | (Choice::Paper, Choice::Scissors) => Self::Win,
            (_, _) => Self::Loss,
        }
    }
}
