use common::Puzzle;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();

    let mut b: Puzzle2 = Default::default();
    b.run();
}

#[derive(Default)]
struct Puzzle1 {
    values: Vec<i64>,
}

impl Puzzle for Puzzle1 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.values.push(item);
    }

    fn final_result(&mut self) -> String {
        for i in 0..(self.values.len() - 1) {
            let entry_a = self.values[i];
            for j in (i + 1)..self.values.len() {
                let entry_b = self.values[j];
                if entry_a + entry_b == 2020 {
                    return (entry_a * entry_b).to_string();
                }
            }
        }
        "Not found".to_string()
    }
}

#[derive(Default)]
struct Puzzle2 {
    values: Vec<i64>,
}

impl Puzzle for Puzzle2 {
    type ParsedLine = i64;

    fn process_item(&mut self, item: Self::ParsedLine) {
        self.values.push(item);
    }

    fn final_result(&mut self) -> String {
        for i in 0..(self.values.len() - 2) {
            let entry_a = self.values[i];
            for j in (i + 1)..(self.values.len() - 1) {
                let entry_b = self.values[j];
                for k in (j + 1)..self.values.len() {
                    let entry_c = self.values[k];
                    if entry_a + entry_b + entry_c == 2020 {
                        return (entry_a * entry_b * entry_c).to_string();
                    }
                }
            }
        }
        "Not found".to_string()
    }
}
