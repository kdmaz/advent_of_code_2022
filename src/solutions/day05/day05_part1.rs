use std::str::FromStr;

struct Procedure {
    stacks: Vec<Vec<char>>,
    steps: Vec<Step>,
}

impl Procedure {
    fn run(&mut self) -> String {
        for step in &self.steps {
            let Step { qty, from, to } = step;
            for _ in 0..*qty {
                let cargo = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].push(cargo);
            }
        }

        let mut result = String::new();

        for stack in &mut self.stacks {
            let cargo = stack.pop().unwrap();
            result.push(cargo);
        }

        result
    }
}

#[derive(Default)]
struct Step {
    qty: u8,
    from: usize,
    to: usize,
}

impl Step {
    fn update(&mut self, code: Code, val: u8) {
        match code {
            Code::Move => self.qty = val,
            Code::From => self.from = val as usize,
            Code::To => self.to = val as usize,
            Code::Invalid => {}
        }
    }
}

#[derive(PartialEq, Eq)]
enum Code {
    Move,
    From,
    To,
    Invalid,
}

impl FromStr for Code {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "move " => Self::Move,
            "from " => Self::From,
            "to " => Self::To,
            _ => Self::Invalid,
        })
    }
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut step = Step::default();
        let mut code_buffer = String::new();
        let mut val_buffer = String::new();

        for char in s.chars() {
            let code = Code::from_str(&code_buffer).unwrap();
            if code == Code::Invalid {
                code_buffer.push(char);
                continue;
            }

            if char == ' ' {
                let val = val_buffer.parse::<u8>().unwrap();
                step.update(code, val);
                code_buffer.clear();
                val_buffer.clear();
                continue;
            }

            val_buffer.push(char);
        }

        Ok(step)
    }
}

fn main(input: &str) -> String {
    for line in input.lines() {
        let i = 0;
        while i < line.len() {}
    }

    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 5);
        let output = main(&input);
        let expected = "CMZ".to_owned();
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 5);
        let output = main(&input);
        let expected = "".to_owned();
        assert_eq!(output, expected);
    }
}
