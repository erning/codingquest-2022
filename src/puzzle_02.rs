fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve(input: &str) -> i32 {
    let winning = vec![12, 48, 30, 95, 15, 55, 97];
    let mut winning_set = vec![false; 100];
    winning.iter().for_each(|&v| winning_set[v as usize] = true);
    let rewards = [0, 0, 0, 1, 10, 100, 1000];

    let mut sum = 0;
    let tickets = parse_input(input);
    for ticket in tickets.into_iter() {
        let mut count = 0;
        for n in ticket.into_iter() {
            if winning_set[n as usize] {
                count += 1;
            }
        }
        sum += rewards[count];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_02-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 110);
    }
}
