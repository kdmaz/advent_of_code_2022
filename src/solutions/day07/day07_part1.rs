use std::{str::FromStr, cell::Ref};
use super::day07_shared::{FileSystem, Directory};

fn get_sizes_under_100_000(directory: &Ref<Directory>, vec: &mut Vec<i32>) {
    for (_, sub_directory) in &directory.sub_directories {
        get_sizes_under_100_000(&sub_directory.clone().borrow(), vec);
    }

    if directory.size < 100_000 {
        vec.push(directory.size);
    }
}

fn sum_of_directories_under_100_000(file_system: FileSystem) -> i32 {
    let mut sizes = vec![];

    get_sizes_under_100_000(&file_system.root.borrow(), &mut sizes);

    sizes.iter().sum()
}

fn main(input: &str) -> i32 {
    let file_system = FileSystem::from_str(input).unwrap();
    sum_of_directories_under_100_000(file_system)
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 7);
        let output = main(&input);
        let expected = 95437;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 7);
        let output = main(&input);
        let expected = 1427048;
        assert_eq!(output, expected);
    }
}
