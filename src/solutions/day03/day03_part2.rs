use super::day03_shared::get_priority;
use std::collections::HashSet;

fn main(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    (0..lines.len())
        .step_by(3)
        .map(|i| {
            let mut backpacks = [HashSet::new(), HashSet::new(), HashSet::new()];

            for (j, line) in lines.iter().skip(i).enumerate().take(3) {
                for c in line.chars() {
                    backpacks[j].insert(c);
                }
            }

            for c in &backpacks[0] {
                if backpacks.iter().all(|backpack| backpack.contains(c)) {
                    return get_priority(*c);
                }
            }

            panic!("Could not find common char in backpacks");
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
        let expected = 70;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 3);
        let output = main(&input);
        let expected = 2683;
        assert_eq!(output, expected);
    }
}
