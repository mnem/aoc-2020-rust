use common::Puzzle;
use std::collections::HashSet;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    seating_ids: Vec<i32>,
}

impl Puzzle1 {
    fn id(pass: &String) -> i32 {
        let mut row = SearchRange { low: 0, high: 127 };
        let mut col = SearchRange { low: 0, high: 7 };
        for c in pass.chars() {
            match c {
                'F' => row.half_low(),
                'B' => row.half_high(),
                'R' => col.half_high(),
                'L' => col.half_low(),
                _ => panic!(),
            }
        }
        row.result().unwrap() * 8 + col.result().unwrap()
    }

    fn possible_ids() -> HashSet<i32> {
        let mut ids = HashSet::new();
        for r in 1..127i32 {
            for c in 1..7i32 {
                let id = r * 8 + c;
                ids.insert(id);
            }
        }
        ids
    }
}

struct SearchRange {
    low: i32,
    high: i32,
}

impl SearchRange {
    fn mid(&self) -> i32 {
        ((self.high - self.low) / 2) + self.low
    }

    fn half_high(&mut self) {
        self.low = self.mid() + 1;
    }

    fn half_low(&mut self) {
        self.high = self.mid();
    }

    fn result(&self) -> Option<i32> {
        if self.low != self.high {
            None
        } else {
            Some(self.low)
        }
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.seating_ids.push(Puzzle1::id(&item));
    }

    fn final_result(&mut self) -> String {
        self.seating_ids.sort();

        let (head, tail) = self.seating_ids.split_at(1);
        let mut last = head.first().unwrap();
        let mut my_seat: Option<i32> = None;
        for id in tail {
            if last + 1 != *id {
                my_seat = Some(last + 1);
                break;
            }
            last = id
        }

        let max_id = self.seating_ids.iter().max().unwrap();

        format!("max: {}, my seat: {}", max_id, my_seat.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_row() {
        let mut row = SearchRange { low: 0, high: 127 };

        row.half_low();
        assert_eq!(0, row.low);
        assert_eq!(63, row.high);

        row.half_high();
        assert_eq!(32, row.low);
        assert_eq!(63, row.high);

        row.half_low();
        assert_eq!(32, row.low);
        assert_eq!(47, row.high);

        row.half_high();
        assert_eq!(40, row.low);
        assert_eq!(47, row.high);

        row.half_high();
        assert_eq!(44, row.low);
        assert_eq!(47, row.high);

        row.half_low();
        assert_eq!(44, row.low);
        assert_eq!(45, row.high);

        row.half_low();
        assert_eq!(44, row.low);
        assert_eq!(44, row.high);

        assert_eq!(44, row.result().unwrap());
    }

    #[test]
    fn example_1_col() {
        let mut col = SearchRange { low: 0, high: 7 };

        col.half_high();
        assert_eq!(4, col.low);
        assert_eq!(7, col.high);

        col.half_low();
        assert_eq!(4, col.low);
        assert_eq!(5, col.high);

        col.half_high();
        assert_eq!(5, col.low);
        assert_eq!(5, col.high);

        assert_eq!(5, col.result().unwrap());
    }

    #[test]
    fn example_1_id() {
        let input_a = "FBFBBFFRLR".to_string();
        assert_eq!(357, Puzzle1::id(&input_a));
        let input_a = "BFFFBBFRRR".to_string();
        assert_eq!(567, Puzzle1::id(&input_a));
        let input_a = "FFFBBBFRRR".to_string();
        assert_eq!(119, Puzzle1::id(&input_a));
        let input_a = "BBFFBBFRLL".to_string();
        assert_eq!(820, Puzzle1::id(&input_a));
    }
}
