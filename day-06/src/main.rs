use common::Puzzle;
use common::FilteredInputLine;
use std::collections::HashSet;

fn main() {
    let mut a = Puzzle1::new();
    a.run();

    // let mut b = Puzzle1::new(RuleSet::Strict);
    // b.run();
}

// #[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
// enum PassportFields {
//     Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid,
// }
//
// struct Rule {
//     field: PassportFields,
//     matcher: Regex,
//     match_validation: Option<Box<dyn Fn(String) -> bool>>
// }
//
// enum RuleSet {
//     Strict,
//     Casual,
// }

#[derive(Default)]
struct Puzzle1 {
    building: HashSet<char>,
    sum: usize,
}

impl Puzzle1 {
    fn new() -> Puzzle1 {
        Puzzle1 { building: Default::default(), sum: Default::default() }
    }

    fn finalise(&mut self) {
        self.sum += self.building.len();
        self.building.clear();
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
        }
    }

    fn final_result(&mut self) -> String {
        self.finalise();
        self.sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string();
        let mut subject: Puzzle1 = Puzzle1::new();
        subject.run_with_input(input);
        assert_eq!(11, subject.sum);
    }
}
