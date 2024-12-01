use crate::runner::AocDay;

pub struct DecemberFirst(String);

impl AocDay for DecemberFirst {
    fn new(content: String) -> Self {
        DecemberFirst(content)
    }

    fn part1(&self) -> String {
        "11".into()
    }

    fn part2(&self) -> String {
        "11".into()
    }
}