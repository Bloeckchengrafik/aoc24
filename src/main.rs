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

    /// Whether to test the solution or run it
    #[arg(short, long, default_value = "false")]
    test: bool,
}

fn load_content(name: String) -> String {
    let path = format!("inputs/{}.txt", name);
    std::fs::read_to_string(path).unwrap()
}

macro_rules! matcher {
    ($day:expr, $test:expr, $content:expr, { $($key:expr => $module:ty),* }) => {
        match $day {
            $($key => if ($test) {<$module as Aoc>::test($content)} else {<$module as Aoc>::run($content)},)*
            _ => panic!("Day not implemented"),
        }
    };
}

fn main() {
    let args = Args::parse();
    let content = if args.test {
        load_content(format!("{}_test", args.day))
    } else {
        load_content(args.day.to_string())
    };

    matcher!(args.day, args.test, content, {
        1 => days::dec1::DecemberFirst,
        2 => days::dec2::DecemberSecond,
        3 => days::dec3::DecemberThird
    });
}
