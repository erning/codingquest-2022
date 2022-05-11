fn parse_input(input: &str) -> (&[u8], &[u8], &[u8], bool) {
    (
        input.lines().nth(0).unwrap().as_bytes(),
        input.lines().nth(1).unwrap().as_bytes(),
        input.lines().nth(2).unwrap().as_bytes(),
        input.lines().nth(3).unwrap() == "forward",
    )
}

pub fn solve(input: &str) -> String {
    let (secret, charset, message, forward) = parse_input(input);
    encode(secret, charset, message, forward)
}

fn encode(secret: &[u8], charset: &[u8], message: &[u8], forward: bool) -> String {
    let mut out: Vec<u8> = Vec::new();
    let mut si = 0;
    for m in message.into_iter() {
        match charset.iter().enumerate().find(|&(_, v)| v == m) {
            Some((i, _)) => {
                let s = secret[si];
                let j = match charset.iter().enumerate().find(|&(_, v)| *v == s) {
                    Some((j, _)) => j + 1,
                    _ => unimplemented!(),
                };
                let offset = if forward {
                    i + j
                } else {
                    charset.len() + i - j
                };
                out.push(charset[offset % charset.len()]);
            }
            _ => {
                out.push(*m);
            }
        }
        si = (si + 1) % secret.len();
    }
    String::from_utf8(out).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_08-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "PJ UTGX LF JXFFW");
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("inputs/puzzle_08-example2.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "dAyevvMbfHgENFsy:fDqnGddIzfMqm");
    }

    #[test]
    fn example3() {
        let input = std::fs::read_to_string("inputs/puzzle_08-example3.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "I could use this to pass secret notes in class!");
    }
}
