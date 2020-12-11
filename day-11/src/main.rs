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
    Hyperspace,
}

enum Perception {
    Close,
    Far,
}

type Row = Vec<FloorState>;

#[derive(Default)]
struct Puzzle1 {
    rows: Vec<Row>,
    previous: Vec<Row>,

    close_occupied: i64,
    far_occupied: i64,
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
            return FloorState::Hyperspace;
        }

        let row = &self.rows[y as usize];
        if x < 0 || x as usize >= row.len() {
            return FloorState::Hyperspace;
        }

        row[x as usize]
    }

    fn sample_close(&self, x: i64, y: i64) -> Vec<FloorState> {
        let mut sample = Vec::new();
        for y_d in -1..=1 {
            for x_d in -1..=1 {
                if y_d == 0 && x_d == 0 {
                    continue;
                }
                let state = self.read(x + x_d, y + y_d);
                sample.push(state);
            }
        }
        sample
    }

    fn sample_directed(&self, x: i64, y: i64, d_x: i64, d_y: i64) -> FloorState {
        let mut cur_x = x + d_x;
        let mut cur_y = y +  d_y;
        let mut sample = self.read(cur_x, cur_y);
        while sample == FloorState::Empty {
            cur_x += d_x;
            cur_y += d_y;
            sample = self.read(cur_x, cur_y);
        }
        sample
    }

    fn sample_far(&self, x: i64, y: i64) -> Vec<FloorState> {
        let mut sample = Vec::new();

        sample.push(self.sample_directed(x, y, 0, -1));
        sample.push(self.sample_directed(x, y, 0, 1));
        sample.push(self.sample_directed(x, y, -1, 0));
        sample.push(self.sample_directed(x, y, 1, 0));
        sample.push(self.sample_directed(x, y, 1, 1));
        sample.push(self.sample_directed(x, y, -1, -1));
        sample.push(self.sample_directed(x, y, 1, -1));
        sample.push(self.sample_directed(x, y, -1, 1));

        sample
    }

    fn step(&self, tolerance: usize, perception: &Perception) -> Vec<Row> {
        let mut next_state = Vec::new();
        for y in 0..self.rows.len() as i64 {
            let row = &self.rows[y as usize];
            let mut new_row = Row::new();
            for x in 0..row.len() as i64 {
                let sample = match perception {
                    Perception::Close => self.sample_close(x, y),
                    Perception::Far => self.sample_far(x, y),
                };
                let next_seat_state = match self.read(x, y) {
                    FloorState::Hyperspace => panic!(),
                    FloorState::Empty => FloorState::Empty,
                    FloorState::Seat => {
                        if sample.iter().filter(|&f| *f == FloorState::Person).count() == 0 {
                            FloorState::Person
                        } else {
                            FloorState::Seat
                        }
                    },
                    FloorState::Person => {
                        if sample.iter().filter(|&f| *f == FloorState::Person).count() >= tolerance {
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

    fn step_and_swap(&mut self, tolerance: usize, perception: &Perception) {
        let new_state = self.step(tolerance, perception);
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
                    FloorState::Hyperspace => panic!(),
                };
                print!("{}", str);
            }
            println!();
        }
    }

    fn print_rows(&self) {
        Puzzle1::print(&self.rows);
    }

    fn run_to_stability(&mut self, tolerance: usize, perception: Perception) {
        while self.rows != self.previous {
            self.step_and_swap(tolerance, &perception);
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
        let initial = self.rows.clone();

        self.run_to_stability(4, Perception::Close);
        self.close_occupied = self.count_occupied();

        self.rows = initial;
        self.previous = Vec::new();
        self.run_to_stability(5, Perception::Far);
        self.far_occupied = self.count_occupied();

        format!("close: {}; far: {}", self.close_occupied, self.far_occupied)
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

        assert_eq!(37, subject.close_occupied);
        assert_eq!(26, subject.far_occupied);
    }
}