use super::day05_shared::{Procedure, Step};

impl Procedure {
    fn run2(&mut self) {
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
}

fn main(input: &str) -> String {
    let mut procedure = Procedure::build(input);
    procedure.run2();
    procedure.get_result()
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 5);
        let output = main(&input);
        let expected = "MCD".to_owned();
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 5);
        let output = main(&input);
        let expected = "TCGLQSLPW".to_owned();
        assert_eq!(output, expected);
    }
}
