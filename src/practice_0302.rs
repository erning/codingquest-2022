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
    let mut grid = parse_input(input);
    let m = grid.len();
    let n = grid[0].len();

    fn discover(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
            return 0;
        }
        let v = &mut grid[y as usize][x as usize];
        if *v == 0 {
            return 0;
        }
        let mut mass = *v;
        *v = 0;
        const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for &(dx, dy) in DIRS.iter() {
            mass += discover(grid, x + dx, y + dy)
        }
        mass
    }

    let mut count = 0;
    let mut masses = 0;
    for y in 0..m {
        for x in 0..n {
            if grid[y][x] != 0 {
                count += 1;
                masses += discover(&mut grid, x as i32, y as i32);
            }
        }
    }

    // println!("{:?}", (count, masses));
    masses / count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/practice_0302-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 8);
    }
}
