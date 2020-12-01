use common::Puzzle;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(Default)]
struct Puzzle1 {
    values: Vec<i64>
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
        return "Not found".to_string();
    }
}
