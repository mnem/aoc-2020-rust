use common::Puzzle;
use std::collections::VecDeque;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.preamble_length = 25;
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    buffer: VecDeque<i64>,
    list: Vec<i64>,
    weak_number: Option<i64>,
    preamble_length: usize,
}

impl Puzzle1 {
    fn is_valid(&self, n: i64) -> bool {
        for a_i in 0..(self.buffer.len() - 1) {
            let a = self.buffer.get(a_i).unwrap();
            for b_i in (a_i + 1)..self.buffer.len() {
                let b = self.buffer.get(b_i).unwrap();
                if (a + b) == n {
                    return true;
                }
            }
        }

        false
    }

    fn check_for_weakness(&mut self, n: i64) {
        if self.weak_number.is_some() {
            // Already have it
            return
        }

        if self.buffer.len() < self.preamble_length {
            // still reading preamble
            self.buffer.push_back(n);
        } else if self.is_valid(n) {
            self.buffer.pop_front();
            self.buffer.push_back(n);
        } else {
            // Found the odd one out
            self.weak_number = Some(n);
        }
    }

    fn find_weak_range(&self) -> &[i64] {
        let target = self.weak_number.unwrap();
        for a_i in 0..(self.list.len() - 1) {
            for b_i in (a_i + 1)..self.list.len() {
                let slice = &self.list[a_i..=b_i];
                if slice.iter().sum::<i64>() == target {
                    return slice;
                }
            }
        }

        panic!()
    }

    fn find_encryption_weakness(&self) -> i64 {
        let weakness = self.find_weak_range();
        let min = weakness.iter().min().unwrap();
        let max = weakness.iter().max().unwrap();

        min + max
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.list.push(item);
        self.check_for_weakness(item);
    }

    fn final_result(&mut self) -> String {
        let weak_num = self.weak_number.unwrap();
        let weak_enc = self.find_encryption_weakness();

        format!("weak number: {}; weak encryption: {}", weak_num, weak_enc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.preamble_length = 5;
        subject.run_with_input(input);

        assert!(subject.weak_number.is_some());
        assert_eq!(127, subject.weak_number.unwrap());
        assert_eq!(62, subject.find_encryption_weakness());
    }
}
