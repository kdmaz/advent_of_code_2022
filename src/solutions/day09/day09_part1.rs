use std::{cmp, str::FromStr};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction::U,
            "D" => Direction::D,
            "R" => Direction::R,
            "L" => Direction::L,
            _ => panic!("Invalid direction"),
        })
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
struct Bridge {
    grid: Vec<Vec<String>>,
    start: Position,
    motions: Vec<Motion>,
}

impl Bridge {
    fn simulate_rope(&mut self) {
        let Bridge {
            grid,
            start,
            motions,
        } = self;
        let mut head = Position {
            x: start.x,
            y: start.y,
        };
        let mut tail = Position {
            x: start.x,
            y: start.y,
        };

        for Motion {
            direction,
            distance,
        } in motions
        {
            for _ in 0..*distance {
                match *direction {
                    Direction::U => head.y += 1,
                    Direction::D => head.y -= 1,
                    Direction::R => head.x += 1,
                    Direction::L => head.x -= 1,
                }

                grid[tail.x as usize][tail.y as usize] = "#".to_owned();
            }
        }
    }

    fn count_visited(&self) -> i32 {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| if col.as_str() == "#" { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum()
    }
}

impl FromStr for Bridge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut min = Position { x: 0, y: 0 };
        let mut max = Position { x: 0, y: 0 };
        let mut current = Position { x: 0, y: 0 };
        let mut start = Position { x: 0, y: 0 };
        let mut motions = vec![];

        for line in s.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            let direction = Direction::from_str(parts[0]).unwrap();
            let distance = parts[1].parse().unwrap();
            let motion = Motion {
                direction,
                distance,
            };
            motions.push(motion);

            match direction {
                Direction::R => current.x += distance,
                Direction::L => current.x -= distance,
                Direction::U => current.y += distance,
                Direction::D => current.y -= distance,
            };

            min.x = cmp::min(min.x, current.x);
            max.x = cmp::max(max.x, current.x);

            min.y = cmp::min(min.y, current.y);
            max.y = cmp::max(max.y, current.y);

            start.x = cmp::min(min.x, start.x).abs();
            start.y = cmp::min(min.y, start.y).abs();
        }

        let length = (max.x - min.x) + 1;
        let height = (max.y - min.y) + 1;

        let grid = vec![vec![".".to_owned(); length as usize]; height as usize];

        Ok(Bridge {
            grid,
            start: Position {
                x: start.x,
                y: start.y,
            },
            motions,
        })
    }
}

fn main(input: &str) -> i32 {
    let mut grid = Bridge::from_str(input).unwrap();
    grid.simulate_rope();
    grid.count_visited()
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
        let expected = 0;
        assert_eq!(output, expected);
    }
}
