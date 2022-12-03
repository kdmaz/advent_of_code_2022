// calories summed for each elf
pub fn sums(input: &str) -> Vec<i32> {
    let mut sums = vec![];

    for calories_per_elf in input.split("\n\n") {
        let sum = calories_per_elf
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        sums.push(sum);
    }

    sums
}
