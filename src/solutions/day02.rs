#[derive(PartialEq, Clone, Copy)]
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

    fn from_outcome_needed(opponent_choice: &MatchChoice, outcome_needed: MatchOutcome) -> Self {
        if outcome_needed == MatchOutcome::Draw {
            return *opponent_choice;
        }

        match (opponent_choice, outcome_needed) {
            (MatchChoice::Scissors, MatchOutcome::Win) | (MatchChoice::Paper, MatchOutcome::Loss) => MatchChoice::Rock,
            (MatchChoice::Rock, MatchOutcome::Win) | (MatchChoice::Scissors, MatchOutcome::Loss) => MatchChoice::Paper,
            _ => MatchChoice::Scissors,
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

    fn from_response_letter(letter: char) -> Self {
        match letter {
            'X' => MatchOutcome::Loss,
            'Y' => MatchOutcome::Draw,
            'Z' => MatchOutcome::Win,
            _ => panic!("Expected 'X', 'Y', or 'Z', but found {}", letter),
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

pub fn part2(input: &str) -> i32 {
    input
    .lines()
    .map(|line| {
        let match_choices = line.chars().collect::<Vec<char>>();
        let opponent_choice = MatchChoice::from_opponent_letter(*match_choices.first().unwrap());
        let outcome_needed = MatchOutcome::from_response_letter(*match_choices.last().unwrap());
        let response_choice = MatchChoice::from_outcome_needed(&opponent_choice, outcome_needed);
        let match_outcome = MatchOutcome::from_choices(&opponent_choice, &response_choice);
        (response_choice as i32) + (match_outcome as i32)
    })
    .sum()
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
        let expected = 12;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = read_file("input", 2);
        let output = part2(&input);
        let expected = 9975;
        assert_eq!(output, expected);
    }
}
