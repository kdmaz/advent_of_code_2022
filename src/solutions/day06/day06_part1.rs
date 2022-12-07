use std::collections::HashSet;

fn main(input: &str) -> i32 {
    let (mut l, mut r) = (0, 0);
    let mut set = HashSet::new();
    let input = input.as_bytes();

    loop {
        while set.contains(&input[r]) {
            set.remove(&input[l]);
            l += 1;
        }

        set.insert(input[r]);
        r += 1;

        if set.len() == 4 {
            return r as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 6);
        let output = main(&input);
        let expected = 7;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 6);
        let output = main(&input);
        let expected = 1625;
        assert_eq!(output, expected);
    }
}
