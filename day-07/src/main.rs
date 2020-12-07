use common::Puzzle;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let mut a = Puzzle1::new();
    a.run();

    // let mut b = Puzzle1::new(RuleSet::Strict);
    // b.run();
}

struct Rule {
    colour: String,
    quantity: usize,
}

struct Puzzle1 {
    bag_rules: HashMap<String, Vec<Rule>>,

    bag_colour_matcher: Regex,
    bag_rule_matcher: Regex,
}

impl Puzzle1 {
    fn new() -> Puzzle1 {
        Puzzle1 {
            bag_colour_matcher: Regex::new(r"^(.+?) bag").unwrap(),
            bag_rule_matcher: Regex::new(r"(\d+?) (.+?) bag").unwrap(),

            bag_rules: Default::default(),
        }
    }

    fn rules_from_line(&self, line: String) -> (String, Vec<Rule>) {
        let bag_colour = self.bag_colour_matcher
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str().to_string();
        let mut rules = Vec::new();
        for capture in self.bag_rule_matcher.captures_iter(&line) {
            let quantity = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let colour = capture.get(2).unwrap().as_str().to_string();

            rules.push( Rule { quantity, colour, } );
        }

        (bag_colour, rules)
    }

    fn count_containers_of(&self, bag: &str) -> usize {
        let mut hits: HashSet<String> = HashSet::new();
        self.find_all_containers(&bag, &mut hits);
        hits.len()
    }

    fn find_all_containers(&self, search_colour: &str, hits: &mut HashSet<String>) {
        for (colour, rules) in &self.bag_rules {
            for rule in rules {
                if rule.colour == search_colour && !hits.contains(colour){
                    hits.insert(colour.clone());
                    self.find_all_containers(&colour, hits);
                }
            }
        }
    }

    fn count_contained_by(&self, bag: &str) -> usize {
        let mut count = 0usize;
        for rule in &self.bag_rules[bag] {
            count += &rule.quantity;
            count += self.count_contained_by(&rule.colour) * rule.quantity;
        }
        count
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let (bag, rules) = self.rules_from_line(item);
        let existing_rules = self.bag_rules.entry(bag).or_insert(Vec::new());
        for rule in rules {
            existing_rules.push(rule);
        }
    }

    fn final_result(&mut self) -> String {
        let containers = self.count_containers_of("shiny gold");
        let contained = self.count_contained_by("shiny gold");

        format!("Containers of: {}; contained by: {}", containers, contained)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_line_parser() {
        let input = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string();
        let subject = Puzzle1::new();
        let (bag_colour, rules) = subject.rules_from_line(input);

        assert_eq!("dark orange", bag_colour);
        assert_eq!("bright white", rules[0].colour);
        assert_eq!(3, rules[0].quantity);
        assert_eq!("muted yellow", rules[1].colour);
        assert_eq!(4, rules[1].quantity);
    }

    #[test]
    fn example_line_parser_2() {
        let input = "faded blue bags contain no other bags.".to_string();
        let subject = Puzzle1::new();
        let (bag_colour, rules) = subject.rules_from_line(input);

        assert_eq!("faded blue", bag_colour);
        assert_eq!(0, rules.len());
    }

    #[test]
    fn example_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.".to_string();
        let mut subject = Puzzle1::new();
        subject.run_with_input(input);

        assert_eq!(9, subject.bag_rules.len());
        assert_eq!(4, subject.count_containers_of("shiny gold"));
    }

    #[test]
    fn example_2() {
        let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags".to_string();
        let mut subject = Puzzle1::new();
        subject.run_with_input(input);

        assert_eq!(7, subject.bag_rules.len());
        assert_eq!(126, subject.count_contained_by("shiny gold"));
    }
}
