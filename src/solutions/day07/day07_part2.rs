use super::day07_shared::Directory;
use crate::solutions::day07::day07_shared::FileSystem;
use std::cmp::min;
use std::{cell::Ref, str::FromStr};

fn get_smallest(directory: &Ref<Directory>, min_to_del: i32) -> i32 {
    let mut smallest = directory.size;
    for sub_directory in directory.sub_directories.values() {
        let result = get_smallest(&sub_directory.clone().borrow(), min_to_del);
        smallest = if result > min_to_del {
            min(smallest, result)
        } else {
            smallest
        }
    }

    smallest
}

fn smallest_directory_size_above_min(file_system: &FileSystem) -> i32 {
    // disk space
    let available = 70_000_000;
    let used = file_system.root.borrow().size;
    let unused = available - used;
    let needed = 30_000_000;
    let minimum_to_delete = needed - unused;

    get_smallest(&file_system.root.borrow(), minimum_to_delete)
}

fn main(input: &str) -> i32 {
    let file_system = FileSystem::from_str(input).unwrap();
    smallest_directory_size_above_min(&file_system)
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 7);
        let output = main(&input);
        let expected = 24933642;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 7);
        let output = main(&input);
        let expected = 2940614;
        assert_eq!(output, expected);
    }
}
