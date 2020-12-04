use common::Puzzle;
use common::FilteredInputLine;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let mut a = Puzzle1::new();
    a.run();

    // let mut b: Puzzle2 = Default::default();
    // b.run();
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
enum PassportFields {
    Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid,
}

struct Rule {
    field: PassportFields,
    matcher: Regex,
}

#[derive(Default)]
struct Puzzle1 {
    building: HashSet<PassportFields>,
    valid_count: usize,

    required_fields: Vec<Rule>
}

impl Puzzle1 {
    fn new() -> Puzzle1 {
        let mut rules = Vec::new();

        rules.push( Rule {
            field: PassportFields::Byr,
            matcher: Regex::new(r"byr:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Iyr,
            matcher: Regex::new(r"iyr:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Eyr,
            matcher: Regex::new(r"eyr:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Hgt,
            matcher: Regex::new(r"hgt:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Hcl,
            matcher: Regex::new(r"hcl:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Ecl,
            matcher: Regex::new(r"ecl:(.+?)\b").expect("Whoops")
        } );
        rules.push( Rule {
            field: PassportFields::Pid,
            matcher: Regex::new(r"pid:(.+?)\b").expect("Whoops")
        } );

        Puzzle1 { building: Default::default(), valid_count: Default::default(), required_fields: rules }
    }

    fn finalise_passport(&mut self) {
        if self.building.is_empty() {
            return;
        }

        let required_fields: HashSet<PassportFields> = self.required_fields.iter().map( |r| r.field ).collect();

        let found_fields = self.building.iter().copied().collect();
        let missing_required_fields: HashSet<&PassportFields> = required_fields.difference(&found_fields).collect();

        // Get ready for the next build
        self.building.clear();

        if missing_required_fields.is_empty() {
            self.valid_count += 1;
        }
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn filter_line(&mut self, line: &str) -> FilteredInputLine {
        let filtered = self.default_filter_line(line);
        if let FilteredInputLine::Skip = filtered  {
            self.finalise_passport();
        }
        filtered
    }


    fn process_item(&mut self, item: Self::ParsedLine) {
        for rule in &self.required_fields {
            if rule.matcher.captures(&item).is_some() {
                self.building.insert(rule.field);
            }
        }
    }

    fn final_result(&mut self) -> String {
        self.finalise_passport();
        self.valid_count.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n".to_string();
        let mut subject: Puzzle1 = Puzzle1::new();
        subject.run_with_input(input);
        assert_eq!(2, subject.valid_count);
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(true, Puzzle2::is_valid("1-3 a: abcde".to_string()));
    //     assert_eq!(false, Puzzle2::is_valid("1-3 b: cdefg".to_string()));
    //     assert_eq!(false, Puzzle2::is_valid("2-9 c: ccccccccc".to_string()));
    // }
}
