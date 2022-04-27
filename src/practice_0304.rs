use std::collections::HashMap;

fn parse_input(input: &str) -> String {
    let mut s = String::new();
    for ch in input.trim().chars() {
        let bin = match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => unimplemented!(),
        };
        s.push_str(bin);
    }
    s
}
fn parse_table() -> HashMap<String, char> {
    let txt = std::fs::read_to_string("inputs/practice_0304-table.txt").unwrap();
    txt.lines()
        .map(|v| (v[2..].to_string(), v.chars().nth(0).unwrap()))
        .collect()
}

pub fn solve(input: &str) -> String {
    let table = parse_table();
    let input = &parse_input(input);
    let mut answer = String::new();
    let mut a = 0;
    let mut b = 1;
    while b <= input.len() {
        let k = &input[a..b];
        if let Some(ch) = table.get(k) {
            if *ch == '*' {
                break;
            }
            answer.push(*ch);
            a = b;
        } else {
            b += 1
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/practice_0304-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, "GOOD_DAY");
    }
}
