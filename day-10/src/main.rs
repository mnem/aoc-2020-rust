use common::Puzzle;
use std::collections::HashMap;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    adaptors: Vec<i64>,
}

impl Puzzle1 {
    fn part_a(&mut self) -> i64 {
        self.adaptors.sort();

        let mut gaps: HashMap<i64,i64> = HashMap::new();
        let mut previous_rating = 0;
        for rating in self.adaptors.iter() {
            let gap = rating - previous_rating;
            *gaps.entry(gap).or_insert(0) += 1;
            previous_rating = *rating;
        }

        // Device
        *gaps.entry(3).or_insert(0) += 1;

        let one_jolt_diffs = gaps[&1];
        let three_jolt_diffs = gaps[&3];

        one_jolt_diffs * three_jolt_diffs
    }

}

impl Puzzle for Puzzle1 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.adaptors.push(item);
    }

    fn final_result(&mut self) -> String {
        let part_a = self.part_a();

        part_a.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);

        assert_eq!(11, subject.adaptors.len());
        assert_eq!(7 * 5, subject.part_a());
    }

    #[test]
    fn example_2() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);

        assert_eq!(31, subject.adaptors.len());
        assert_eq!(22 * 10, subject.part_a());
    }
}
