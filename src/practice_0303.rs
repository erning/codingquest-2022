fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|v| i32::from_str_radix(v, 16).unwrap())
                .collect()
        })
        .collect()
}

pub fn solve(input: &str) -> i32 {
    let grid = parse_input(input);
    let n = grid[0].len() - 1;
    let m = grid.len() - 1;

    let mut err_y = m;
    let mut cs_y = 0;
    for y in 0..m {
        let mut cs = 0;
        for x in 0..n {
            cs += grid[y][x];
            cs %= 256;
        }
        if cs != grid[y][n] {
            err_y = y;
            cs_y = cs;
            break;
        }
    }
    let mut err_x = n;
    let mut cs_x = 0;
    for x in 0..n {
        let mut cs = 0;
        for y in 0..m {
            cs += grid[y][x];
            cs %= 256;
        }
        if cs != grid[m][x] {
            err_x = x;
            cs_x = cs;
            break;
        }
    }
    // println!("{:?}", (err_x, err_y));
    let a = (256 + cs_y - grid[err_y][n]) % 256;
    let b = (256 + cs_x - grid[m][err_x]) % 256;
    assert_eq!(a, b);

    let error = grid[err_y][err_x];
    let correct = (256 + error - a) % 256;
    error * correct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/practice_0303-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 3072);
    }
}
