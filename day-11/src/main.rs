use common::Puzzle;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.run();
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum FloorState {
    Empty,
    Seat,
    Person,
}

type Row = Vec<FloorState>;

#[derive(Default)]
struct Puzzle1 {
    rows: Vec<Row>,
    previous: Vec<Row>,
}

impl Puzzle1 {
    fn to_row(input: &str) -> Row {
        let mut line = Row::new();
        for c in input.chars() {
            let state = match c {
                'L' => FloorState::Seat,
                '.' => FloorState::Empty,
                _ => panic!()
            };
            line.push(state);
        }

        line
    }

    fn read(&self, x: i64, y: i64) -> FloorState {
        if y < 0 || y as usize >= self.rows.len() {
            return FloorState::Empty;
        }

        let row = &self.rows[y as usize];
        if x < 0 || x as usize >= row.len() {
            return FloorState::Empty;
        }

        row[x as usize]
    }

    fn sample(&self, x: i64, y: i64) -> Vec<FloorState> {
        let mut sample = Vec::new();
        for y_d in -1..=1 {
            for x_d in -1..=1 {
                if y_d == 0 && x_d == 0 {
                    continue;
                }
                let state = self.read(x + x_d, y + y_d);
                if state != FloorState::Empty {
                    sample.push(state);
                }
            }
        }
        sample
    }

    fn step(&self) -> Vec<Row> {
        let mut next_state = Vec::new();
        for y in 0..self.rows.len() as i64 {
            let row = &self.rows[y as usize];
            let mut new_row = Row::new();
            for x in 0..row.len() as i64 {
                let sample = self.sample(x, y);
                let next_seat_state = match self.read(x, y) {
                    FloorState::Empty => FloorState::Empty,
                    FloorState::Seat => {
                        if sample.iter().filter(|&f| *f == FloorState::Person).count() == 0 {
                            FloorState::Person
                        } else {
                            FloorState::Seat
                        }
                    },
                    FloorState::Person => {
                        if sample.iter().filter(|&f| *f == FloorState::Person).count() >= 4 {
                            FloorState::Seat
                        } else {
                            FloorState::Person
                        }
                    },
                };
                new_row.push(next_seat_state);
            }
            next_state.push(new_row);
        }

        next_state
    }

    fn step_and_swap(&mut self) {
        let new_state = self.step();
        self.previous = self.rows.clone();
        self.rows = new_state;
    }

    fn print(rows: &[Row]) {
        for row in rows {
            for place in row {
                let str = match place {
                    FloorState::Empty => ".",
                    FloorState::Seat => "L",
                    FloorState::Person => "#",
                };
                print!("{}", str);
            }
            println!();
        }
    }

    fn print_rows(&self) {
        Puzzle1::print(&self.rows);
    }

    fn run_to_stability(&mut self) {
        while self.rows != self.previous {
            self.step_and_swap();
        }
    }

    fn count_occupied(&self) -> i64 {
        let mut count = 0;
        for row in &self.rows {
            count += row.iter().filter(|&f| *f == FloorState::Person).count();
        }
        count as i64
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let line = Puzzle1::to_row(&item);
        self.rows.push(line);
    }

    fn final_result(&mut self) -> String {
        self.run_to_stability();
        self.count_occupied().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.run_with_input(input);

        let num_occupied = subject.count_occupied();

        assert_eq!(37, num_occupied);
    }

    // #[test]
    // fn example_2() {
    //     let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3".to_string();
    //     let mut subject: Puzzle1 = Default::default();
    //     subject.run_with_input(input);
    //
    //     assert_eq!(31, subject.adaptors.len());
    //     assert_eq!(22 * 10, subject.part_a());
    // }
}