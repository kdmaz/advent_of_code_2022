use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn change_position(direction: &str, head: &mut Position, tail: &mut Position) {
    match direction {
        "U" => {
            head.y += 1;

            if head.y.abs_diff(tail.y) == 2 {
                tail.y = head.y - 1;
                tail.x = head.x;
            }
        }
        "D" => {
            head.y -= 1;

            if head.y.abs_diff(tail.y) == 2 {
                tail.y = head.y + 1;
                tail.x = head.x;
            }
        }
        "R" => {
            head.x += 1;

            if head.x.abs_diff(tail.x) == 2 {
                tail.x = head.x - 1;
                tail.y = head.y;
            }
        }
        "L" => {
            head.x -= 1;

            if head.x.abs_diff(tail.x) == 2 {
                tail.x = head.x + 1;
                tail.y = head.y;
            }
        }
        _ => {}
    }
}

fn main(input: &str) -> i32 {
    let start = Position { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(start);

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0];
        let distance: i32 = parts[1].parse().unwrap();

        for _ in 0..distance {
            change_position(direction, &mut head, &mut tail);
            visited_positions.insert(tail);
        }
    }

    visited_positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 9);
        let output = main(&input);
        let expected = 13;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 9);
        let output = main(&input);
        let expected = 6181;
        assert_eq!(output, expected);
    }
}
