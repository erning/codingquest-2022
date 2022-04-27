use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|v| v.split_whitespace().collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .collect()
}

pub fn solve(input: &str) -> String {
    let guesses = parse_input(input);
    // println!("guesses: {:?}", guesses);

    let mut green: Vec<(char, usize)> = Vec::new();
    let mut yellow: Vec<(char, usize)> = Vec::new();
    let mut black: Vec<char> = Vec::new();
    let mut blackset: HashSet<char> = HashSet::new();
    for guess in guesses.iter() {
        for (i, color) in guess.1.as_bytes().iter().enumerate() {
            let ch = guess.0.chars().nth(i).unwrap();
            match color {
                b'G' => green.push((ch, i)),
                b'Y' => yellow.push((ch, i)),
                b'B' => {
                    if blackset.insert(ch) {
                        black.push(ch)
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
    // println!("G: {:?}", &green);
    // println!("Y: {:?}", &yellow);
    // println!("B: {:?}", &black);

    let words = std::fs::read_to_string("inputs/practice_0301-word.txt").unwrap();
    let words: Vec<&str> = words.lines().collect();
    // println!("{:?}", words.len());

    let words: Vec<&str> = words
        .into_iter()
        .filter(|&v| {
            let mut found = true;
            for &ch in green.iter() {
                if v.chars().nth(ch.1).unwrap() != ch.0 {
                    found = false;
                    break;
                }
            }
            found
        })
        .collect();
    // println!("{:?}", words.len());

    let words: Vec<&str> = words
        .into_iter()
        .filter(|&v| {
            let mut found = true;
            for &ch in yellow.iter() {
                if v.chars().find(|&c| ch.0 == c).is_none() {
                    found = false;
                    break;
                }
            }
            found
        })
        .collect();
    // println!("{:?}", words.len());

    let words: Vec<&str> = words
        .into_iter()
        .filter(|&v| {
            let mut found = false;
            for &ch in black.iter() {
                if v.chars().find(|&c| ch == c).is_some() {
                    found = true;
                    break;
                }
            }
            !found
        })
        .collect();

    // println!("{:?}", words.len());

    words.first().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/practice_0301-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "helpful");
    }
}
