fn main(input: &str) -> u32 {
    let nums: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let row_len = nums[0].len() as u32;
    let col_len = nums.len() as u32;
    let mut visible = row_len * 2;
    visible += (col_len - 2) * 2;

    for row in 0..nums.len() {
        if row == 0 || row == row_len as usize - 1 {
            continue;
        }

        'outer: for col in 0..nums[row].len() {
            if col == 0 || col == col_len as usize - 1 {
                continue;
            }

            let num = nums[row][col];

            // top
            for top in (0..row).rev() {
                let val = nums[top][col];
                if num > val && top != 0 {
                    continue;
                } else if num <= val {
                    break;
                } else {
                    visible += 1;
                    continue 'outer;
                }
            }

            // bottom
            for bottom in (row + 1)..row_len as usize {
                let val = nums[bottom][col];
                if num > val && bottom != (row_len - 1) as usize {
                    continue;
                } else if num <= val {
                    break;
                } else {
                    visible += 1;
                    continue 'outer;
                }
            }

            // left
            for left in (0..col).rev() {
                let val = nums[row][left];
                if num > val && left != 0 {
                    continue;
                } else if num <= val {
                    break;
                } else {
                    visible += 1;
                    continue 'outer;
                }
            }

            // right
            for right in (col + 1)..col_len as usize {
                let val = nums[row][right];
                if num > val && right != (col_len - 1) as usize {
                    continue;
                } else if num <= val {
                    break;
                } else {
                    visible += 1;
                }
            }
        }
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::main;
    use crate::read_file;

    #[test]
    fn example() {
        let input = read_file("examples", 8);
        let output = main(&input);
        let expected = 21;
        assert_eq!(output, expected);
    }

    #[test]
    fn input() {
        let input = read_file("input", 8);
        let output = main(&input);
        let expected = 1805;
        assert_eq!(output, expected);
    }
}
