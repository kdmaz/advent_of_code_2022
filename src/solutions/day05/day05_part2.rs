use super::day05_shared::Procedure;

fn main(input: &str) -> String {
    let mut procedure = Procedure::build(input);
    procedure.run_with_9001();
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
