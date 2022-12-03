#!/bin/bash

set -e;

if [ ! -n "$1" ]; then
    >&2 echo "Argument is required for day."
    exit 1
fi

day=$(echo $1 | sed 's/^0*//');
day_padded=`printf %02d $day`;
day_formatted="day$day_padded";

day_directory="src/solutions/$day_formatted";
day_part1="$day_directory/${day_formatted}_part1.rs";
day_part2="$day_directory/${day_formatted}_part2.rs";
day_mod="$day_directory/mod.rs"

mkdir $day_directory;
touch $day_part1;
touch $day_part2;
touch $day_mod;

example_path="examples/$day_formatted.txt";
input_path="input/$day_formatted.txt";

touch $example_path;
touch $input_path;

new_file=$(cat << EOF
fn main(input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", $day);
        let output = main(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", $day);
        let output = main(&input);
        let expected = 0;
        assert_eq!(output, expected);
    }
}
EOF
)

echo "$new_file" >> $day_part1;
echo "$new_file" >> $day_part2;

echo "pub mod $day_formatted;" >> "src/solutions/mod.rs";
echo "pub mod ${day_formatted}_part1;" >> "src/solutions/$day_formatted/mod.rs";
echo "pub mod ${day_formatted}_part2;" >> "src/solutions/$day_formatted/mod.rs";
