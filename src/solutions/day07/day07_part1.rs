use std::{str::FromStr, rc::Rc, cell::RefCell, collections::HashMap};

type RefDirectory = Rc<RefCell<Directory>>;

#[derive(Debug)]
struct FileSystem {
    root: RefDirectory
}

impl FromStr for FileSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Rc::new(RefCell::new(Directory::new("/".to_owned())));
        let mut cwd = vec![root.clone()];
        let file_system = FileSystem {
            root: root.clone()
        };

        for line in s.lines() {
            if line.starts_with("$ cd /") || line.starts_with("$ ls") {
                continue;
            } else if line.starts_with("$ cd ..") {
                cwd.pop();
            } else if line.starts_with("$ cd") {
                let name = line.split("$ cd ").collect::<Vec<&str>>()[1].to_owned();
                let next_directory = cwd.last().unwrap().borrow().sub_directories.get(&name).unwrap().clone();
                cwd.push(next_directory);
            } else {
                println!("line {}", line);
                cwd.last().unwrap().borrow_mut().populate_from_str(line);
            }
        }

        Ok(file_system)
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: i32,
    files: Vec<File>,
    sub_directories: HashMap<String, RefDirectory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            size: i32::default(),
            files: vec![],
            sub_directories: HashMap::new(),
        }
    }

    fn populate_from_str(&mut self, s: &str) {
        if s.starts_with("dir") {
            let sub_directory = Self::from_str(s).unwrap();
            self.sub_directories.insert(sub_directory.name.clone(), Rc::new(RefCell::new(sub_directory)));
        } else {
            let file = File::from_str(s).unwrap();
            self.files.push(file);
        }
    }
}

impl FromStr for Directory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.split(' ').collect::<Vec<&str>>()[1];
        Ok(Self::new(name.to_owned()))
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        let size: i32 = split[0].parse().unwrap();
        let name = split[1].to_owned();
        Ok(Self { name, size })
    }
}

fn main(input: &str) -> i32 {
    let _filesystem = FileSystem::from_str(input).unwrap();
    println!("{:#?}", _filesystem);
    -1
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
        let expected = 0;
        assert_eq!(output, expected);
    }
}