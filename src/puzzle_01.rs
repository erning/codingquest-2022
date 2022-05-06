fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .into_iter()
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn solve(input: &str) -> usize {
    // return parse_input(input)
    //     .windows(60)
    //     .map(|v| v.into_iter().sum::<i32>())
    //     .filter(|&v| v < 1500 & 60 || v > 1600 * 60)
    //     .count();

    let data = parse_input(input);
    let ws = 60;
    if data.len() < ws {
        return 0;
    }
    let lo = 1500 * ws;
    let hi = 1600 * ws;
    let mut sum = data[0..ws].into_iter().sum::<usize>();
    let mut count = if sum < lo || sum > hi { 1 } else { 0 };
    for i in ws..data.len() {
        sum -= data[i - ws];
        sum += data[i];
        if sum < lo || sum > hi {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_01-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 10);
    }
}
