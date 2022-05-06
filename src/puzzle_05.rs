use sha2::{Digest, Sha256};

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    //  description|mined number|previous hash
    input
        .lines()
        .map(|v| v.split('|').into_iter().collect())
        .collect()
}

fn mine(s: &str, prev: &str) -> (String, String) {
    let mut n = 1;
    let mut digest = Sha256::new();
    loop {
        let m = n.to_string();
        digest.update(s);
        digest.update(b"|");
        digest.update(&m);
        digest.update(b"|");
        digest.update(prev);
        let hash = digest.finalize_reset();
        if &hash[..3] == &[0, 0, 0] {
            let hash = format!("{:x}", hash);
            return (m, hash);
        }
        n += 1;
    }
}

fn verify(s: &str, m: &str, prev: &str, hash: &str) -> bool {
    let mut digest = Sha256::new();
    digest.update(s);
    digest.update(b"|");
    digest.update(m);
    digest.update(b"|");
    digest.update(prev);
    let h = digest.finalize();
    let h = format!("{:x}", h);
    h == hash
}

pub fn solve(input: &str) -> String {
    let records = parse_input(input);
    let mut ok = true;
    let mut prev = "".to_string();
    for record in records.into_iter() {
        let (s, m, p, h) = (record[0], record[1], record[2], record[3]);
        if ok {
            ok = verify(s, m, p, h);
            if ok {
                continue;
            }
            prev = p.to_string();
        }
        let mined = mine(s, &prev);
        println!("{}", record[0]);
        println!("    m: {}", mined.0);
        println!("    p: {}", prev);
        println!("    h: {}", mined.1);
        prev = mined.1;
    }
    prev.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mine_works() {
        let prev = "0000000000000000000000000000000000000000000000000000000000000000";
        let (n, s) = mine("Original iPhone still in box", &prev);
        assert_eq!(n, "3595421");
        assert_eq!(
            s,
            "00000078f97879b26be6baf2adb520b126f84ed10464ed4e9a77b8ed87e07468"
        );
    }

    #[test]
    fn verify_works() {
        assert_eq!(
            verify(
                "Apollo 11 moon rock",
                "27703084",
                "00000078f97879b26be6baf2adb520b126f84ed10464ed4e9a77b8ed87e07468",
                "00000068a1e823c97e72ff22b0450dc4cfa66495b6ac56266edb0389c2d9a045",
            ),
            true
        );
        assert_eq!(
            verify(
                "Apollo 11 moon rock",
                "27703083",
                "00000078f97879b26be6baf2adb520b126f84ed10464ed4e9a77b8ed87e07468",
                "00000068a1e823c97e72ff22b0450dc4cfa66495b6ac56266edb0389c2d9a045",
            ),
            false
        );
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_05-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "");
    }
}
