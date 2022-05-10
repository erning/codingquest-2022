fn parse_input(input: &str) -> Vec<Rect> {
    input
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| Rect::new(v[0], v[1], v[0] + v[2], v[1] + v[3]))
        .collect()
}

pub fn solve(input: &str) -> i32 {
    let rects = parse_input(input);
    let mut added: Vec<Rect> = Vec::new();
    let mut removed: Vec<Rect> = Vec::new();

    for rect in rects.into_iter() {
        let len = removed.len();
        for other in added.iter() {
            if let Some(overlap) = rect.overlap(&other) {
                removed.push(overlap);
            }
        }
        for other in removed.iter().take(len) {
            if let Some(overlap) = rect.overlap(&other) {
                added.push(overlap);
            }
        }
        added.push(rect);
    }

    let mut sum = 0;
    for i in 0..usize::max(added.len(), removed.len()) {
        if i < added.len() {
            sum += added[i].size();
        }
        if i < removed.len() {
            sum -= removed[i].size();
        }
    }
    sum

    // let mut sum = 0;
    // let mut iter_a = added.into_iter();
    // let mut iter_b = removed.into_iter();
    // let mut opt_a = iter_a.next();
    // let mut opt_b = iter_b.next();
    // while opt_a.is_some() || opt_b.is_some() {
    //     if let Some(v) = opt_a {
    //         sum += v.size();
    //         opt_a = iter_a.next();
    //     }
    //     if let Some(v) = opt_b {
    //         sum -= v.size();
    //         opt_b = iter_b.next();
    //     }
    // }
    // sum

    // let mut sum = 0i64;
    // sum += added.into_iter().fold(0i64, |a, i| a + i.size() as i64);
    // sum -= removed.into_iter().fold(0i64, |a, i| a + i.size() as i64);
    // sum as i32
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
        Rect { x1, y1, x2, y2 }
    }
    fn size(&self) -> i32 {
        i32::abs(self.x2 - self.x1) * i32::abs(self.y2 - self.y1)
    }

    fn overlap(&self, other: &Rect) -> Option<Rect> {
        if let Some(x) = segment(self.x1, self.x2, other.x1, other.x2) {
            if let Some(y) = segment(self.y1, self.y2, other.y1, other.y2) {
                return Some(Rect::new(x.0, y.0, x.1, y.1));
            }
        }
        None
    }
}

fn segment(mut a1: i32, mut a2: i32, mut b1: i32, mut b2: i32) -> Option<(i32, i32)> {
    if a1 == a2 || b1 == b2 {
        return None;
    }
    if a1 > a2 {
        std::mem::swap(&mut a1, &mut a2);
    }
    if b1 > b2 {
        std::mem::swap(&mut b1, &mut b2);
    }
    if b2 <= a1 {
        None // b1, b2, a1, a2
    } else if b1 <= a1 && b2 >= a1 && b2 <= a2 {
        Some((b2, a1)) // b1, a1, b2, a2
    } else if b1 <= a1 && b2 >= a2 {
        Some((a2, a1)) // b1, a1, a2, b2
    } else if b1 >= a1 && b2 <= a2 {
        Some((b2, b1)) // a1, b1, b2, a2
    } else if b1 >= a1 && b1 <= a2 && b2 >= a2 {
        Some((a2, b1)) // a1, b1, a2, b2
    } else {
        None // a1, a2, b1, b2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_07-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 10 * 10 - 12);
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("inputs/puzzle_07-example2.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, 100 * 100 - 2061);
    }
}
