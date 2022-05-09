use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

struct Processor<'a> {
    vars: HashMap<&'a str, i64>,
    program: Vec<&'a str>,
    pc: usize,
    flag: bool,
    output: Vec<i64>,
}

impl<'a> Processor<'a> {
    fn new(program: Vec<&'a str>) -> Self {
        Processor {
            vars: HashMap::new(),
            program,
            pc: 0,
            flag: false,
            output: Vec::new(),
        }
    }

    fn get(&self, oprand: &str) -> i64 {
        oprand
            .parse()
            .unwrap_or(*self.vars.get(oprand).unwrap_or(&0))
    }

    fn set(&mut self, name: &'a str, value: i64) {
        if let Some(t) = self.vars.get_mut(name) {
            *t = value;
        } else {
            self.vars.insert(name, value);
        }
    }

    fn exec(&mut self) {
        let v: Vec<&str> = self.program[self.pc].split_whitespace().collect();
        match v[0] {
            "ADD" => {
                let t = self.get(&v[1]);
                let s = self.get(&v[2]);
                self.set(&v[1], t + s);
                self.pc += 1;
            }
            "MOD" => {
                let t = self.get(&v[1]);
                let s = self.get(&v[2]);
                self.set(&v[1], t % s);
                self.pc += 1;
            }
            "DIV" => {
                let t = self.get(&v[1]);
                let s = self.get(&v[2]);
                self.set(&v[1], t / s);
                self.pc += 1;
            }
            "MOV" => {
                let s = self.get(&v[2]);
                self.set(&v[1], s);
                self.pc += 1;
            }
            "JMP" => {
                let s = self.get(&v[1]);
                self.pc = (self.pc as i64 + s) as usize;
            }
            "JIF" => {
                if self.flag {
                    let s = self.get(&v[1]);
                    self.pc = (self.pc as i64 + s) as usize;
                } else {
                    self.pc += 1;
                }
            }
            "CEQ" => {
                let s1 = self.get(&v[1]);
                let s2 = self.get(&v[2]);
                self.flag = s1 == s2;
                self.pc += 1;
            }
            "CGE" => {
                let s1 = self.get(&v[1]);
                let s2 = self.get(&v[2]);
                self.flag = s1 >= s2;
                self.pc += 1;
            }
            "OUT" => {
                let s = self.get(&v[1]);
                self.output.push(s);
                self.pc += 1;
            }
            "END" => {
                self.pc = usize::MAX;
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.exec();
        }
    }
}

pub fn solve(input: &str) -> Vec<i64> {
    let program = parse_input(input);
    let mut processor = Processor::new(program);
    processor.run();
    processor.output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = std::fs::read_to_string("inputs/puzzle_06-example.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, &[111]);
    }

    #[test]
    fn example2() {
        let input = std::fs::read_to_string("inputs/puzzle_06-example2.txt").unwrap();
        let output = solve(&input);
        assert_eq!(output, &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}
