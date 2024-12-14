#![feature(test)]

use crate::runner::Aoc;
use clap::Parser;

extern crate test;

mod runner;
pub mod days;
pub mod utils;

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
        mod stages {
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
        1 => days::dec1::HistorianHysteria, dec1, "11", "31",
        2 => days::dec2::RedNosedReports, dec2, "2", "4",
        3 => days::dec3::MullItOver, dec3, "161", "48",
        4 => days::dec4::CeresSearch, dec4, "18", "9",
        5 => days::dec5::PrintQueue, dec5, "143", "123",
        6 => days::dec6::GuardGallivant, dec6, "41", "5",
        7 => days::dec7::BridgeRepair, dec7, "3749", "11387",
        8 => days::dec8::ResonantCollinearity, dec8, "14", "34",
        9 => days::dec9::DiskFragmenter, dec9, "1928", "2858",
        10 => days::dec10::HoofIt, dec10, "36", "81",
        11 => days::dec11::PlutonianPebbles, dec11, "55312", "149161030616311",
        12 => days::dec12::GardenGroups, dec12, "140", "368",
        13 => days::dec13::ClawContraption, dec13, "480", "875318608908",
        14 => days::dec14::RestroomRedoubt, dec14, "12", "0",
});


fn main() {
    let args = Args::parse();
    let content = load_content(args.day.to_string());
    run_day(args.day, content);
}
