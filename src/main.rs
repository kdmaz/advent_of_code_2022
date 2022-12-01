use solutions::day01;
use advent_of_code_2022::read_file;
mod solutions;

fn main() {
    let input = read_file("input", 1);

    day01::part1(&input);
    day01::part2(&input);
}
