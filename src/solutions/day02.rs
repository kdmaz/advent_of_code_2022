#[derive(PartialEq)]
enum MatchChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl MatchChoice {
    fn from_opponent_letter(letter: char) -> Self {
        match letter {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("Expected 'A', 'B', or 'C', but found {}", letter),
        }
    }

    fn from_response_letter(letter: char) -> Self {
        match letter {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Expected 'X', 'Y', or 'Z', but found {}", letter),
        }
    }
}

enum MatchOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl MatchOutcome {
    fn from_choices(opponent_choice: &MatchChoice, response_choice: &MatchChoice) -> Self {
        if opponent_choice == response_choice {
            return MatchOutcome::Draw;
        }

        match (opponent_choice, response_choice) {
            (MatchChoice::Rock, MatchChoice::Paper)
            | (MatchChoice::Scissors, MatchChoice::Rock)
            | (MatchChoice::Paper, MatchChoice::Scissors) => MatchOutcome::Win,
            (_, _) => MatchOutcome::Loss,
        }
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let match_choices = line.chars().collect::<Vec<char>>();
            let opponent_choice = MatchChoice::from_opponent_letter(*match_choices.first().unwrap());
            let response_choice = MatchChoice::from_response_letter(*match_choices.last().unwrap());
            let match_outcome = MatchOutcome::from_choices(&opponent_choice, &response_choice);
            (response_choice as i32) + (match_outcome as i32)
        })
        .sum()
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
        let expected = 15;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = read_file("input", 2);
        let output = part1(&input);
        let expected = 12276;
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
