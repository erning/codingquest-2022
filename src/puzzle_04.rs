fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|v| {
            v.as_bytes()
                .into_iter()
                .map(|v| (v - b'1') as usize)
                .collect()
        })
        .collect()
}

fn check_vertical(board: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let p = board[x][y];
    return y >= 3 && p == board[x][y - 1] && p == board[x][y - 2] && p == board[x][y - 3];
}

fn check_horizontal(board: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let p = board[x][y];
    let mut c = 1;
    let mut i = x;
    while i > 0 && p == board[i - 1][y] {
        c += 1;
        i -= 1;
    }
    i = x + 1;
    while i < 7 && p == board[i][y] {
        c += 1;
        i += 1;
    }
    c >= 4
}

fn check_diagonal_1(board: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let p = board[x][y];
    let mut c = 1;
    let mut i = x;
    let mut j = y;
    while i > 0 && j > 0 && p == board[i - 1][j - 1] {
        c += 1;
        i -= 1;
        j -= 1;
    }
    i = x + 1;
    j = y + 1;
    while i < 7 && j < 7 && p == board[i][j] {
        c += 1;
        i += 1;
        j += 1;
    }
    c >= 4
}

fn check_diagonal_2(board: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let p = board[x][y];
    let mut c = 1;
    let mut i = x;
    let mut j = y + 1;
    while i > 0 && j < 7 && p == board[i - 1][j] {
        c += 1;
        i -= 1;
        j += 1;
    }
    i = x + 1;
    j = y;
    while i < 7 && j > 0 && p == board[i][j - 1] {
        c += 1;
        i += 1;
        j -= 1;
    }
    c >= 4
}

fn play(board: &mut Vec<Vec<usize>>, column: usize, player: usize) -> bool {
    let x = column;
    let mut y = 0;
    for v in board[column].iter_mut() {
        if *v == 0 {
            *v = player;
            break;
        }
        y += 1;
    }
    check_vertical(board, x, y)
        || check_horizontal(board, x, y)
        || check_diagonal_1(board, x, y)
        || check_diagonal_2(board, x, y)
}

pub fn solve(input: &str) -> i32 {
    let mut wins = vec![0; 3];
    let games = parse_input(input);
    for game in games {
        let mut board: Vec<Vec<usize>> = vec![vec![0; 7]; 7];
        let mut winner: Option<usize> = None;
        for (i, m) in game.into_iter().enumerate() {
            let player = i % 3 + 1;
            if play(&mut board, m, player) {
                winner = Some(player);
                break;
            }
        }
        if let Some(winner) = winner {
            wins[winner - 1] += 1;
        }
    }
    wins[0] * wins[1] * wins[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_04-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 125);
    }
}
