use crate::runner::Aoc;
use clap::Parser;

mod runner;
pub mod days;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The day to run
    #[arg(short, long)]
    day: u8,
}

fn load_content(name: String) -> String {
    let path = format!("inputs/{}.txt", name);
    std::fs::read_to_string(path).unwrap()
}

macro_rules! matcher {
    ({ $($key:literal => $module:ty, $name:ident, $exp1:literal, $exp2:literal),* $(,)? }) => {
fn run_day(day: u8, content: String) {
        match day {
            $($key => {<$module as Aoc>::run(content)},)*
            _ => panic!("Day not implemented"),
        }
}

        #[cfg(test)]
        mod tests {
            pub use super::*;
            $(
                #[cfg(test)]
                mod $name {
                    use super::*;
                    #[test]
                    fn part1() {
                        let content = load_content(format!("{}_test1", $key));
                        <$module as Aoc>::test_1(content, $exp1.to_string());
                    }

                    #[test]
                    fn part2() {
                        let content = load_content(format!("{}_test2", $key));
                        <$module as Aoc>::test_2(content, $exp2.to_string());
                    }
                }
            )*
        }
    };
}


matcher!({
        1 => days::dec1::DecemberFirst, dec1, "11", "31",
        2 => days::dec2::DecemberSecond, dec2, "2", "4",
        3 => days::dec3::DecemberThird, dec3, "161", "48",
        4 => days::dec4::DecemberFourth, dec4, "18", "9",
});


fn main() {
    let args = Args::parse();
    let content = load_content(args.day.to_string());
    run_day(args.day, content);
}
