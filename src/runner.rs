pub trait AocDay {
    fn new(content: String) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub trait Aoc {
    fn run(content: String);
    fn test(content: String);

}

static ANSI_GREEN_PLUS: &str = "[\x1b[32m+\x1b[0m]";
static ANSI_RED_MINUS: &str = "[\x1b[31m-\x1b[0m]";

impl <T> Aoc for T where T: AocDay {
    fn run(content: String) {
        let day = T::new(content);
        let part1start = std::time::Instant::now();
        let part1 = day.part1();
        let part1end = std::time::Instant::now();
        let part2start = std::time::Instant::now();
        let part2 = day.part2();
        let part2end = std::time::Instant::now();

        println!("{} Part 1: {} ({}ms)", ANSI_GREEN_PLUS, part1, part1end.duration_since(part1start).as_millis());
        println!("{} Part 2: {} ({}ms)", ANSI_GREEN_PLUS, part2, part2end.duration_since(part2start).as_millis());
    }

    fn test(content: String) {
        let test_input = TestInput::new(content);
        let day = T::new(test_input.input);
        let part1start = std::time::Instant::now();
        let part1 = day.part1();
        let part1end = std::time::Instant::now();
        let part2start = std::time::Instant::now();
        let part2 = day.part2();
        let part2end = std::time::Instant::now();

        if part1 != test_input.expected1 {
            println!("{} FAIL 1: Expected: {}, got: {}", ANSI_RED_MINUS, test_input.expected1, part1);
        } else {
            println!("{} PASS 1: {} ({}ms)", ANSI_GREEN_PLUS, part1, part1end.duration_since(part1start).as_millis());
        }

        if part2 != test_input.expected2 {
            println!("{} FAIL 2: Expected: {}, got: {}", ANSI_RED_MINUS, test_input.expected2, part2);
        } else {
            println!("{} PASS 2: {} ({}ms)", ANSI_GREEN_PLUS, part2, part2end.duration_since(part2start).as_millis());
        }
    }
}

pub struct TestInput {
    pub input: String,
    pub expected1: String,
    pub expected2: String,
}

impl TestInput {
    pub fn new(source: String) -> Self {
        // last two lines are expected1 and expected2
        let mut input = "".to_string();
        let mut expected1 = "".to_string();
        let mut expected2 = "".to_string();
        for line in source.lines() {
            if !expected2.is_empty() {
                input += &*(expected2.as_str().to_owned() + "\n");
            }
            expected2 = expected1.to_string();
            expected1 = line.to_string();
        }

        TestInput {
            input: input.trim().to_string(),
            expected1: expected2,
            expected2: expected1,
        }
    }
}