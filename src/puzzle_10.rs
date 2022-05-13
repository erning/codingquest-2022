use image::{io::Reader as ImageReader, GenericImageView};

fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|v| -> Vec<u8> {
            v.split_whitespace()
                .map(|v| u8::from_str_radix(v, 16).unwrap())
                .collect()
        })
        .collect()
}

fn parse_image(input: &str) -> Vec<u8> {
    let img = ImageReader::open(input).unwrap().decode().unwrap();
    img.pixels().map(|(_, _, rgba)| rgba[0]).collect()
}

pub fn solve(input: &str, is_image_file: bool) -> String {
    let bytes = if is_image_file {
        parse_image(input)
    } else {
        parse_input(input)
    };

    let mut i = 0;
    let mut chars = Vec::new();
    while i < bytes.len() {
        let mut v = 0;
        for j in 0..8 {
            v <<= 1;
            v |= bytes[i + j] & 1;
        }
        if v == 0 {
            break;
        }
        chars.push(v);
        i += 8;
    }
    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_10-example.txt").unwrap();
        let output = solve(&input, false);
        assert_eq!(output, "Hello, world!");
    }
}
