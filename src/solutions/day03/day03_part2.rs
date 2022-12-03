use std::collections::HashSet;

use super::day03_shared::get_priority;

fn main(input: &str) -> i32 {
    let mut line_index = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    while line_index < lines.len() {
        let mut groups = [HashSet::new(), HashSet::new(), HashSet::new()];

        for (current_line_index, _) in lines.iter().enumerate().skip(line_index).take(3) {
            for c in lines[current_line_index].chars() {
                let group_index = current_line_index - line_index;
                groups[group_index].insert(c);
            }
        }

        for c in &groups[0] {
            if groups.iter().all(|group| group.contains(c)) {
                sum += get_priority(*c);
            }
        }

        line_index += 3;
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
