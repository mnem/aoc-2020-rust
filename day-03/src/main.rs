use common::Puzzle;
use std::cmp::max;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();

    // let mut b: Puzzle2 = Default::default();
    // b.run();
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum GroundState {
    Open,
    Tree,
}

type Line = Vec<GroundState>;

#[derive(Default)]
struct Puzzle1 {
    slope: Vec<Line>,
    max_line: usize
}

impl Puzzle1 {
    fn to_line(input: String) -> Line {
        let mut line = Line::new();
        for c in input.chars() {
            let state = match c {
                '#' => GroundState::Tree,
                '.' => GroundState::Open,
                _ => panic!()
            };
            line.push(state);
        }

        line
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let line = Puzzle1::to_line(item);
        self.max_line = max(self.max_line, line.len());
        self.slope.push(line);
    }

    fn final_result(&mut self) -> String {
        let mut trees = 0;
        let mut y = 3;
        for x in 1..self.slope.len() {
            if self.slope[x][y % self.max_line] == GroundState::Tree {
                trees += 1;
            }
            y += 3;
        }
        trees.to_string()
    }
}

// #[derive(Default)]
// struct Puzzle2 {
//     valid_passwords: i64,
//     invalid_passwords: i64,
// }
//
// impl Puzzle2 {
//     fn is_valid(rule: String) -> bool {
//         let pos_a: usize;
//         let pos_b: usize;
//         let letter: char;
//         let password: String;
//         let mut chars = rule.into_bytes().into_iter();
//         scan!(chars => "{}-{} {}: {}", pos_a, pos_b, letter, password);
//
//
//         let pos_a_char = password.chars().nth(pos_a - 1).expect("Pos A OOB");
//         let pos_b_char = password.chars().nth(pos_b - 1).expect("Pos B OOB");
//
//         let in_pos_a = letter == pos_a_char;
//         let in_pos_b = letter == pos_b_char;
//
//         in_pos_a ^ in_pos_b
//     }
// }

// impl Puzzle for Puzzle2 {
//     type ParsedLine = String;
//
//     fn process_item(&mut self, item: Self::ParsedLine) {
//         if Puzzle2::is_valid(item) {
//             self.valid_passwords += 1;
//         } else {
//             self.invalid_passwords += 1;
//         }
//     }
//
//     fn final_result(&mut self) -> String {
//         self.valid_passwords.to_string()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example_1() {
//         assert_eq!(true, Puzzle1::is_valid("1-3 a: abcde".to_string()));
//         assert_eq!(false, Puzzle1::is_valid("1-3 b: cdefg".to_string()));
//         assert_eq!(true, Puzzle1::is_valid("2-9 c: ccccccccc".to_string()));
//     }
//
//     #[test]
//     fn example_2() {
//         assert_eq!(true, Puzzle2::is_valid("1-3 a: abcde".to_string()));
//         assert_eq!(false, Puzzle2::is_valid("1-3 b: cdefg".to_string()));
//         assert_eq!(false, Puzzle2::is_valid("2-9 c: ccccccccc".to_string()));
//     }
// }
