use std::str::FromStr;

pub struct Procedure {
    stacks: Vec<Vec<char>>,
    steps: Vec<Step>,
}

impl Procedure {
    pub fn run_with_9000(&mut self) {
        for step in &self.steps {
            let Step { qty, from, to } = step;
            for _ in 0..*qty {
                let c = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].push(c);
            }
        }
    }

    pub fn run_with_9001(&mut self) {
        for step in &self.steps {
            let Step { qty, from, to } = step;
            let mut stack = vec![];
            for _ in 0..*qty {
                let c = self.stacks[from - 1].pop().unwrap();
                stack.push(c);
            }

            stack.reverse();

            for c in stack {
                self.stacks[to - 1].push(c);
            }
        }
    }

    pub fn get_result(&mut self) -> String {
        let mut result = String::new();
        for stack in &mut self.stacks {
            let c = stack.pop().unwrap();
            result.push(c);
        }
        result
    }
}

impl FromStr for Procedure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut procedure = Procedure {
            stacks: vec![],
            steps: vec![],
        };

        for line in s.lines() {
            if line.starts_with('m') {
                let step = Step::from_str(line).unwrap();
                procedure.steps.push(step);
                continue;
            }

            let mut is_crate = false;
            for (i, c) in line.chars().enumerate() {
                let stack_i = i / 4;

                if procedure.stacks.len() == stack_i {
                    procedure.stacks.push(vec![]);
                }

                if is_crate {
                    procedure.stacks[stack_i].push(c);
                    is_crate = false;
                }

                if c == '[' {
                    is_crate = true;
                }
            }
        }

        for stack in &mut procedure.stacks {
            stack.reverse();
        }

        Ok(procedure)
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

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut step = Step::default();
        let mut code_buffer = String::new();
        let mut val_buffer = String::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        for (i, char) in chars.into_iter().enumerate() {
            let code = Code::from_str(&code_buffer).unwrap();
            if code == Code::Invalid {
                code_buffer.push(char);
                continue;
            }

            val_buffer.push(char);

            if char == ' ' || i + 1 == len {
                let val = val_buffer.trim().parse::<u8>().unwrap();
                step.update(code, val);
                code_buffer.clear();
                val_buffer.clear();
                continue;
            }
        }

        Ok(step)
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "move " => Self::Move,
            "from " => Self::From,
            "to " => Self::To,
            _ => Self::Invalid,
        })
    }
}
