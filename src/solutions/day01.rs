use std::cmp;

pub fn part1(input: &str) -> i32 {
    let mut max = 0;
    let mut total = 0;

    for line in input.lines() {
        if let Ok(num) = line.parse::<i32>() {
            total += num;
        } else {
            max = cmp::max(total, max);
            total = 0;
        }
    }

    max = cmp::max(total, max);
    
    max
}

pub fn part2(input: &str) -> i32 {
    let mut sums = vec![];
    let mut total = 0;

    for line in input.lines() {
        if let Ok(num) = line.parse::<i32>() {
            total += num;
        } else {
            sums.push(total);
            total = 0;
        }
    }

    sums.push(total);
    
    sums.sort();
    sums.reverse();

    let mut sum = 0;

    for i in 0..3 {
        sum += sums[i];
    }


    sum
}

#[cfg(test)]
mod tests {
    use crate::read_file;
    use super::{part1, part2};

    #[test]
    fn part1_example() {
        let input = read_file("examples", 1);
        let output = part1(&input);
        let expected = 24000;
        assert_eq!(output, expected);
    }

    #[test]
    fn part1_input() {
        let input = read_file("input", 1);
        let output = part1(&input);
        let expected = 72602;
        assert_eq!(output, expected);
    }
    
    #[test]
    fn part2_example() {
        let input = read_file("examples", 1);
        let output = part2(&input);
        let expected = 45000;
        assert_eq!(output, expected);
    }

    #[test]
    fn part2_input() {
        let input = read_file("input", 1);
        let output = part2(&input);
        let expected = 207410;
        assert_eq!(output, expected);
    }
}