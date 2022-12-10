fn calculate_added_signal_strength(cycle: i32, x: i32) -> i32 {
    match cycle {
        20 => 20 * x,
        _ if cycle >= 60 && ((cycle - 20) % 40) == 0 => cycle * x,
        _ => 0,
    }
}

fn main(input: &str) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strength = 0;

    for line in input.lines() {
        if line.starts_with("noop") {
            cycle += 1;
            signal_strength += calculate_added_signal_strength(cycle, x);
            continue;
        }

        let parts: Vec<&str> = line.split(' ').collect();
        let val: i32 = parts[1].trim().parse().unwrap();

        for _ in 0..2 {
            cycle += 1;
            signal_strength += calculate_added_signal_strength(cycle, x);
        }

        x += val;
    }

    signal_strength
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 10);
        let output = main(&input);
        let expected = 13140;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 10);
        let output = main(&input);
        let expected = 13720;
        assert_eq!(output, expected);
    }
}
