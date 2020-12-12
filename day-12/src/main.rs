use common::Puzzle;
use regex::Regex;

fn main() {
    let mut a: Puzzle1 = Default::default();
    a.direction = 90;
    a.run();

    let mut b: Puzzle2 = Default::default();
    b.w_x = 10;
    b.w_y = 1;
    b.run();
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Command {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    RotateLeft(i32),
    RotateRight(i32),
    Forward(i32),
}

#[derive(Default)]
struct Puzzle1 {
    direction: i32,
    x: i32,
    y: i32,
}

impl Puzzle1 {
    fn to_command(input: String) -> Command {
        let matcher = Regex::new(r"([A-Z])(\d+)").unwrap();
        let captures = matcher.captures(&input).unwrap();

        let command: &str = captures.get(1).unwrap().as_str();
        let quantity: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        match command {
            "N" => Command::North(quantity),
            "S" => Command::South(quantity),
            "E" => Command::East(quantity),
            "W" => Command::West(quantity),

            "L" => Command::RotateLeft(quantity),
            "R" => Command::RotateRight(quantity),

            "F" => Command::Forward(quantity),

            _ => panic!(),
        }
    }

    fn move_north(&mut self, quantity: i32) {
        self.y += quantity;
    }

    fn move_south(&mut self, quantity: i32) {
        self.y += -quantity;
    }

    fn move_east(&mut self, quantity: i32) {
        self.x += quantity;
    }

    fn move_west(&mut self, quantity: i32) {
        self.x += -quantity;
    }

    fn rotate_left(&mut self, quantity: i32) {
        self.direction += -quantity;
        self.direction = self.direction % 360;
    }

    fn rotate_right(&mut self, quantity: i32) {
        self.direction += quantity;
        self.direction = self.direction % 360;
    }

    fn move_forward(&mut self, quantity: i32) {
        match self.direction {
            0 => self.move_north(quantity),
            90 | -270 => self.move_east(quantity),
            180 | -180 => self.move_south(quantity),
            270 | -90 => self.move_west(quantity),

            // Our boat is like the Automan car
            _ => panic!(format!("Can't move with {} degrees!", self.direction)),
        };
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::North(n) => self.move_north(n),
            Command::South(n) => self.move_south(n),
            Command::East(n) => self.move_east(n),
            Command::West(n) => self.move_west(n),

            Command::RotateLeft(n) => self.rotate_left(n),
            Command::RotateRight(n) => self.rotate_right(n),

            Command::Forward(n) => self.move_forward(n),
        };
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Puzzle for Puzzle1 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let command = Puzzle1::to_command(item);
        self.execute(command);
    }

    fn final_result(&mut self) -> String {
        self.manhattan_distance().to_string()
    }
}


#[derive(Default)]
struct Puzzle2 {
    x: i32,
    y: i32,

    w_x: i32,
    w_y: i32,
}

impl Puzzle2 {
    fn move_north(&mut self, quantity: i32) {
        self.w_y += quantity;
    }

    fn move_south(&mut self, quantity: i32) {
        self.w_y += -quantity;
    }

    fn move_east(&mut self, quantity: i32) {
        self.w_x += quantity;
    }

    fn move_west(&mut self, quantity: i32) {
        self.w_x += -quantity;
    }

    fn rotate_ccw(&mut self, direction: i32) {
        match direction {
            // 0 => ,
            90 | -270 => {
                let tmp_y = self.w_y;
                self.w_y = self.w_x;
                self.w_x = -tmp_y;
            },
            180 | -180 => {
                self.w_y = -self.w_y;
                self.w_x = -self.w_x;
            },
            270 | -90 => {
                let tmp_y = self.w_y;
                self.w_y = -self.w_x;
                self.w_x = tmp_y;
            },

            // Our boat is like the Automan car
            _ => panic!(format!("Can't move with {} degrees!", direction)),
        };
    }

    fn rotate_cw(&mut self, direction: i32) {
        self.rotate_ccw(-direction);
    }

    fn move_forward(&mut self, quantity: i32) {
        self.x += self.w_x * quantity;
        self.y += self.w_y * quantity;
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::North(n) => self.move_north(n),
            Command::South(n) => self.move_south(n),
            Command::East(n) => self.move_east(n),
            Command::West(n) => self.move_west(n),

            Command::RotateLeft(n) => self.rotate_ccw(n),
            Command::RotateRight(n) => self.rotate_cw(n),

            Command::Forward(n) => self.move_forward(n),
        };
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Puzzle for Puzzle2 {
    type ParsedLine = String;

    fn process_item(&mut self, item: Self::ParsedLine) {
        let command = Puzzle1::to_command(item);
        self.execute(command);
    }

    fn final_result(&mut self) -> String {
        self.manhattan_distance().to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "F10\nN3\nF7\nR90\nF11".to_string();
        let mut subject: Puzzle1 = Default::default();
        subject.direction = 90;
        subject.run_with_input(input);

        assert_eq!(25, subject.manhattan_distance());
    }

    #[test]
    fn example_2() {
        let input = "F10\nN3\nF7\nR90\nF11".to_string();
        let mut subject: Puzzle2 = Default::default();
        subject.w_x = 10;
        subject.w_y = 1;
        subject.run_with_input(input);

        assert_eq!(286, subject.manhattan_distance());
    }

    #[test]
    fn example_3() {
        let input = "F10".to_string();
        let mut subject: Puzzle2 = Default::default();
        subject.w_x = 10;
        subject.w_y = 1;
        subject.run_with_input(input);

        assert_eq!(10, subject.w_x);
        assert_eq!(1, subject.w_y);
        assert_eq!(100, subject.x);
        assert_eq!(10, subject.y);
    }

    #[test]
    fn example_4() {
        let input = "R90\nF10".to_string();
        let mut subject: Puzzle2 = Default::default();
        subject.w_x = 10;
        subject.w_y = 1;
        subject.run_with_input(input);

        assert_eq!(1, subject.w_x);
        assert_eq!(-10, subject.w_y);
        assert_eq!(10, subject.x);
        assert_eq!(-100, subject.y);
    }

    #[test]
    fn example_5() {
        let input = "L90\nF10".to_string();
        let mut subject: Puzzle2 = Default::default();
        subject.w_x = 10;
        subject.w_y = 1;
        subject.run_with_input(input);

        assert_eq!(-1, subject.w_x);
        assert_eq!(10, subject.w_y);
        assert_eq!(-10, subject.x);
        assert_eq!(100, subject.y);
    }
}
