fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve(input: &str) -> i64 {
    let data = parse_input(input);
    let mut sum = 0;
    for v in data.windows(2) {
        let x = v[0][0] - v[1][0];
        let y = v[0][1] - v[1][1];
        let z = v[0][2] - v[1][2];
        let s = (x * x + y * y + z * z) as f64;
        sum += s.sqrt() as i64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_03-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 85);
    }
}
