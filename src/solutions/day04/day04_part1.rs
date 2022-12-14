use super::day04_shared::Pair;
use std::str::FromStr;

fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let pairs: Vec<&str> = line.split(',').collect();

            let p1 = Pair::from_str(pairs[0]).unwrap();
            let p2 = Pair::from_str(pairs[1]).unwrap();

            (p1.is_subset(&p2) || p2.is_subset(&p1)) as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 4);
        let output = main(&input);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 4);
        let output = main(&input);
        let expected = 509;
        assert_eq!(output, expected);
    }
}
