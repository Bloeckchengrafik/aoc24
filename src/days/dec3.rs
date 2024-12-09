use crate::runner::AocDay;

#[derive(Debug)]
pub struct Mul(Vec<i32>);

impl Mul {
    pub fn new(values: Vec<i32>) -> Self {
        Mul(values)
    }

    pub fn mul(&self) -> i32 {
        self.0.iter().fold(1, |acc, x| acc * x)
    }
}

pub struct MullItOver {
    content: String,
}


impl AocDay for MullItOver {
    fn new(content: String) -> Self {
        Self { content }
    }

    fn part1(&self) -> String {
        let regex = regex::Regex::new(r"mul\((\d+,\d+)\)").unwrap();
        let mut muls = vec![];

        // find all matches
        let matches = regex.find_iter(&self.content);
        for mat in matches {
            let mut digits = mat.as_str();
            digits = digits.trim_start_matches("mul(");
            digits = digits.trim_end_matches(")");
            let values: Vec<i32> = digits.split(",").map(|x| x.parse().unwrap()).collect();
            let mul = Mul::new(values);
            muls.push(mul);
        }

        
        let mut sum = 0;
        for mul in muls {
            sum += mul.mul();
        }
        sum.to_string()
    }

    fn part2(&self) -> String { 
        let do_dont_regex = regex::Regex::new(r"do(n't)?\(\)").unwrap();
        let mut allowed_ranges = vec![];
        let mut is_do = true;
        let mut last_do_pos = 0;
        fn push_range(allowed_ranges: &mut Vec<(i32, i32)>, is_do: bool, last_do_pos: i32, pos: i32) {
            if is_do {
                allowed_ranges.push((last_do_pos, pos));
            }
        }

        for ma in do_dont_regex.find_iter(&self.content) {
            if ma.as_str().contains("n't") {
                push_range(&mut allowed_ranges, is_do, last_do_pos, ma.start() as i32);
                is_do = false;
                last_do_pos = ma.end() as i32;
            } else {
                push_range(&mut allowed_ranges, is_do, last_do_pos, ma.start() as i32);
                is_do = true;
                last_do_pos = ma.end() as i32;
            }
        }

        push_range(&mut allowed_ranges, is_do, last_do_pos, self.content.len() as i32);

        let regex = regex::Regex::new(r"mul\((\d+,\d+)\)").unwrap();
        let mut muls = vec![];

        // find all matches
        let matches = regex.find_iter(&self.content);
        for mat in matches {
            let start = mat.start() as i32;
            let mut is_allowed = false;
            for range in &allowed_ranges {
                if start >= range.0 && start <= range.1 {
                    is_allowed = true;
                    break;
                }
            }

            if !is_allowed {
                continue;
            }

            let mut digits = mat.as_str();
            digits = digits.trim_start_matches("mul(");
            digits = digits.trim_end_matches(")");
            let values: Vec<i32> = digits.split(",").map(|x| x.parse().unwrap()).collect();
            let mul = Mul::new(values);
            muls.push(mul);
        }

        
        let mut sum = 0;
        for mul in muls {
            sum += mul.mul();
        }
        sum.to_string()
    }
}