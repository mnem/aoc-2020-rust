use common::Puzzle;
use common::FilteredInputLine;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    building: HashSet<char>,
    building_count: HashMap<char, usize>,
    num_people: usize,

    sum: usize,
    better_sum: usize,
}

impl Puzzle1 {
    fn finalise(&mut self) {
        self.sum += self.building.len();
        self.building.clear();

        for (_c, hits) in &self.building_count {
            if *hits == self.num_people {
                self.better_sum += 1;
            }
        }
        self.building_count.clear();
        self.num_people = 0;
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn filter_line(&mut self, line: &str) -> FilteredInputLine {
        let filtered = self.default_filter_line(line);
        if let FilteredInputLine::Skip = filtered  {
            self.finalise();
        }
        filtered
    }


    fn process_item(&mut self, item: Self::ParsedLine) {
        for answer in item.chars() {
            self.building.insert(answer);
            *self.building_count.entry(answer).or_insert(0) += 1;
        }
        self.num_people += 1;
    }

    fn final_result(&mut self) -> String {
        self.finalise();

        format!("sum: {}, better_sum: {}", self.sum, self.better_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);
        assert_eq!(11, subject.sum);
        assert_eq!(6, subject.better_sum);
    }
}
