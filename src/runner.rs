pub trait AocDay {
    fn new(content: String) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub trait Aoc {
    fn run(content: String);
    #[cfg(test)]
    fn test_1(content: String, result: String);
    #[cfg(test)]
    fn test_2(content: String, result: String);
}

static ANSI_GREEN_PLUS: &str = "[\x1b[32m+\x1b[0m]";

impl <T> Aoc for T where T: AocDay {
    fn run(content: String) {
        let day = T::new(content);
        let part1start = std::time::Instant::now();
        let part1 = day.part1();
        let part1end = std::time::Instant::now();
        println!("{} Part 1: {} ({}ms)", ANSI_GREEN_PLUS, part1, part1end.duration_since(part1start).as_millis());

        let part2start = std::time::Instant::now();
        let part2 = day.part2();
        let part2end = std::time::Instant::now();

        println!("{} Part 2: {} ({}ms)", ANSI_GREEN_PLUS, part2, part2end.duration_since(part2start).as_millis());
    }

    #[cfg(test)]
    fn test_1(content: String, result: String) {
        let day = T::new(content);
        let part1 = day.part1();
        assert_eq!(part1, result, "Expected: {}, Got: {}", result, part1);
    }

    #[cfg(test)]
    fn test_2(content: String, result: String) {
        let day = T::new(content);
        let part2 = day.part2();
        assert_eq!(part2, result, "Expected: {}, Got: {}", result, part2);
    }
}
