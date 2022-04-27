fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let n = input.lines().next().unwrap().split_whitespace().count();

    let mut board: Vec<Vec<i32>> = input
        .lines()
        .take(n)
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut path: Vec<i32> = Vec::with_capacity(n * n);
    let mut reverse = false;
    while let Some(mut v) = board.pop() {
        if reverse {
            v.reverse();
        }
        reverse = !reverse;
        path.append(&mut v);
    }

    let rounds: Vec<i32> = input
        .lines()
        .skip(n)
        .flat_map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            // .flatten()
        })
        .collect();

    (path, rounds)
}

pub fn solve(input: &str) -> usize {
    let (path, rounds) = parse_input(input);
    let mut pos = [0; 2];
    for (i, &v) in rounds.iter().enumerate() {
        let p = &mut pos[i % 2];
        *p += v;
        while (*p as usize) < path.len() && path[*p as usize] != 0 {
            *p += path[*p as usize];
        }
        if (*p as usize) >= path.len() {
            // println!("{:?}", (i / 2 + 1, i % 2 + 1));
            return (i / 2 + 1) * (i % 2 + 1);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/practice_0228-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 13);
    }
}
