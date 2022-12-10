use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn change_position(direction: &str, head: &mut Position, tail: &mut Position) {
    match direction {
        "U" => head.y += 1,
        "D" => head.y -= 1,
        "R" => head.x += 1,
        "L" => head.x -= 1,
        _ => panic!("Invalid direction"),
    }

    if head.y.abs_diff(tail.y) > 1 {
        tail.y = head.y - if direction == "U" { 1 } else { -1 };
        tail.x = head.x;
    }

    if head.x.abs_diff(tail.x) > 1 {
        tail.x = head.x - if direction == "R" { 1 } else { -1 };
        tail.y = head.y;
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
