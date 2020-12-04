use common::Puzzle;
use common::FilteredInputLine;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let mut a = Puzzle1::new(RuleSet::Casual);
    a.run();

    let mut b = Puzzle1::new(RuleSet::Strict);
    b.run();
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
enum PassportFields {
    Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid,
}

struct Rule {
    field: PassportFields,
    matcher: Regex,
    match_validation: Option<Box<dyn Fn(String) -> bool>>
}

enum RuleSet {
    Strict,
    Casual,
}

#[derive(Default)]
struct Puzzle1 {
    building: HashSet<PassportFields>,
    valid_count: usize,

    required_fields: Vec<Rule>
}

impl Puzzle1 {
    fn new(strict: RuleSet) -> Puzzle1 {
        let rules = match strict {
            RuleSet::Strict => Puzzle1::make_strict_rules(),
            RuleSet::Casual => Puzzle1::make_casual_rules(),
        };

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

    fn make_strict_rules() -> Vec<Rule> {
        let mut rules = Vec::new();
        rules.push( Rule {
            field: PassportFields::Byr,
            matcher: Regex::new(r"byr:(\d{4})\b").expect("Whoops"),
            match_validation: Some(Box::new(|s: String| -> bool {
                (1920..=2002).contains(&s.parse().unwrap())
            })),
        } );
        rules.push( Rule {
            field: PassportFields::Iyr,
            matcher: Regex::new(r"iyr:(\d{4})\b").expect("Whoops"),
            match_validation: Some(Box::new(|s: String| -> bool {
                (2010..=2020).contains(&s.parse().unwrap())
            })),
        } );
        rules.push( Rule {
            field: PassportFields::Eyr,
            matcher: Regex::new(r"eyr:(\d{4})\b").expect("Whoops"),
            match_validation: Some(Box::new(|s: String| -> bool {
                (2020..=2030).contains(&s.parse().unwrap())
            })),
        } );
        rules.push( Rule {
            field: PassportFields::Hgt,
            matcher: Regex::new(r"hgt:(\d+?(cm|in))\b").expect("Whoops"),
            match_validation: Some(Box::new(|s: String| -> bool {
                let c = Regex::new(r"(\d+)(.+)").unwrap().captures(&s).unwrap();
                let n: i32 = c.get(1).unwrap().as_str().to_string().parse().unwrap();
                match c.get(2).unwrap().as_str() {
                    "cm" => (150..=193).contains(&n),
                    "in" => (59..=76).contains(&n),
                    _ => panic!()
                }
            })),
        } );
        rules.push( Rule {
            field: PassportFields::Hcl,
            matcher: Regex::new(r"hcl:#([0-9a-f]{6})\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Ecl,
            matcher: Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Pid,
            matcher: Regex::new(r"pid:(\d{9})\b").expect("Whoops"),
            match_validation: None,
        } );

        rules
    }

    fn make_casual_rules() -> Vec<Rule> {
        let mut rules = Vec::new();
        rules.push( Rule {
            field: PassportFields::Byr,
            matcher: Regex::new(r"byr:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Iyr,
            matcher: Regex::new(r"iyr:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Eyr,
            matcher: Regex::new(r"eyr:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Hgt,
            matcher: Regex::new(r"hgt:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Hcl,
            matcher: Regex::new(r"hcl:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Ecl,
            matcher: Regex::new(r"ecl:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );
        rules.push( Rule {
            field: PassportFields::Pid,
            matcher: Regex::new(r"pid:(.+?)\b").expect("Whoops"),
            match_validation: None,
        } );

        rules
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
            if let Some(c) = rule.matcher.captures(&item) {
                match &rule.match_validation {
                    None => {
                        self.building.insert(rule.field);
                    },
                    Some(v) => {
                        if v(c.get(1).unwrap().as_str().to_string()) {
                            self.building.insert(rule.field);
                        }
                    }
                };
            }
        }
    }

    fn final_result(&mut self) -> String {
        self.finalise_passport();
        self.valid_count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n".to_string();
        let mut subject: Puzzle1 = Puzzle1::new(RuleSet::Casual);
        subject.run_with_input(input);
        assert_eq!(2, subject.valid_count);
    }
}
