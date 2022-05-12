use std::collections::BinaryHeap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|v| v.as_bytes().into_iter().map(|&v| v).collect())
        .collect()
}

pub fn solve(input: &str) -> i32 {
    let map = parse_input(input);

    let n = map[0].len() as i32;
    let m = map.len() as i32;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

    let start = map[0].iter().enumerate().find(|v| *v.1 != b'#').unwrap().0 as i32;
    queue.push((-1, start, 0));

    while let Some((step, x, y)) = queue.pop() {
        if y == m - 1 {
            return -step;
        }
        visited.insert((x, y));
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nx = x + dx;
            if nx < 0 || nx >= n {
                continue;
            }
            let ny = y + dy;
            if ny < 0 || ny >= m {
                continue;
            }
            if map[ny as usize][nx as usize] == b'#' {
                continue;
            }
            if visited.contains(&(nx, ny)) {
                continue;
            }
            queue.push((step - 1, nx, ny));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_09-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 71);
    }
}
