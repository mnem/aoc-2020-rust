use common::Puzzle;
use std::cmp::max;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
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

    fn count_trees(&self, x_step: usize, y_step: usize) -> i64 {
        let mut trees = 0;
        let mut x = x_step;
        let mut y = y_step;
        while y < self.slope.len() {
            if self.slope[y][x % self.max_line] == GroundState::Tree {
                trees += 1;
            }
            x += x_step;
            y += y_step;
        }

        trees
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
        let trees_a = self.count_trees(1, 1);
        let trees_b = self.count_trees(3, 1);
        let trees_c = self.count_trees(5, 1);
        let trees_d = self.count_trees(7, 1);
        let trees_e = self.count_trees(1, 2);

        format!("{}, {}", trees_b, (trees_a * trees_b * trees_c * trees_d * trees_e))
    }
}
