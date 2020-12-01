use std::fs;
use std::time::{Instant, Duration};

pub trait Puzzle {
    type ParsedLine: std::str::FromStr;

    fn process_item(&mut self, item: Self::ParsedLine);
    fn final_result(&mut self) -> String;

    fn parse_line(&mut self, line: &str) -> Self::ParsedLine {
        match line.parse() {
            Ok(i) => i,
            Err(_) => panic!()
        }
    }

    fn input(&self) -> String {
        let input_filename = String::from("input.txt");
        fs::read_to_string(input_filename)
            .expect("Failed to read file")
    }

    fn run(&mut self) {
        let run_start = Instant::now();
        let mut process_durations = Vec::new();

        let input = self.input();
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let item = self.parse_line(line);

            let processing_start = Instant::now();
            self.process_item(item);
            process_durations.push(processing_start.elapsed());
        }

        let final_result_start = Instant::now();
        let result = self.final_result();
        let final_result_duration = final_result_start.elapsed();

        let run_duration = run_start.elapsed();

        let avg_process = process_durations.iter().sum::<Duration>() / process_durations.len() as u32;
        let min_process = process_durations.iter().min().expect("No min!");
        let max_process = process_durations.iter().max().expect("No max!");

        println!("Result: {} (run: {}, process: {} (min: {}, max: {}), final: {})\n",
                 result,
                 fmt_dur(run_duration),
                 fmt_dur(avg_process), fmt_dur(*min_process), fmt_dur(*max_process),
                 fmt_dur(final_result_duration));
    }
}

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{}ms ", whole_ms) + &fmt_time(rem_ms);
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;

    format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0)
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct IntegerAdder {
        total: i64
    }

    impl Puzzle for IntegerAdder {
        type ParsedLine = i64;

        fn process_item(&mut self, item: Self::ParsedLine) {
            self.total += item;
        }

        fn final_result(&mut self) -> String {
            self.total.to_string()
        }

        fn input(&self) -> String {
            String::from("1\n2\n3\n")
        }
    }

    struct StringCatter {
        result: String
    }

    impl Puzzle for StringCatter {
        type ParsedLine = String;

        fn process_item(&mut self, item: Self::ParsedLine) {
            self.result.push_str(&item);
        }

        fn final_result(&mut self) -> String {
            self.result.clone()
        }
    }

    #[test]
    fn test_input_override() {
        let mut subject: IntegerAdder = Default::default();
        subject.run();
        assert_eq!("6", subject.final_result());
    }

    #[test]
    fn test_input() {
        let mut subject = StringCatter { result: String::new() };
        subject.run();
        assert_eq!("abcde", subject.final_result());
    }
}
