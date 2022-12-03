use std::collections::HashSet;

use super::day03_shared::get_priority;

fn main(input: &str) -> i32 {
    let mut i = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    while i < lines.len() {
        let mut groups = [HashSet::new(), HashSet::new(), HashSet::new()];

        for (j, line) in lines.iter().skip(i).enumerate().take(3) {
            for c in line.chars() {
                groups[j].insert(c);
            }
        }

        for c in &groups[0] {
            if groups.iter().all(|group| group.contains(c)) {
                sum += get_priority(*c);
            }
        }

        i += 3;
    }

    sum
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
