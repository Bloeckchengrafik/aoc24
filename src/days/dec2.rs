use crate::runner::AocDay;

#[derive(Debug)]
pub struct ReportLine(Vec<i32>);

impl From<String> for ReportLine {
    fn from(value: String) -> Self {
        let values: Vec<i32> = value.split(" ").map(|x| x.parse().unwrap()).collect();
        ReportLine(values)
    }
}

impl ReportLine {
    pub fn is_all_increasing(&self) -> bool {
        let mut last = self.0[0];
        for i in 1..self.0.len() {
            if self.0[i] <= last || (self.0[i] - last).abs() > 3 {
                return false;
            }
            last = self.0[i];
        }
        true
    }

    pub fn is_same_direction_skip(&self) -> bool {
        for i in 0..self.0.len() {
            let mut new_vec = self.0.clone();
            new_vec.remove(i);
            let diffs = new_vec.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i32>>();
            if (diffs.iter().all(|x| *x > 0) || diffs.iter().all(|x| *x < 0)) && diffs.iter().all(|x| x.abs() <= 3) {
                return true;
            }
        }

        false
    }

    pub fn is_all_decreasing(&self) -> bool {
        let mut last = self.0[0];
        for i in 1..self.0.len() {
            if self.0[i] >= last || (self.0[i] - last).abs() > 3 {
                return false;
            }
            last = self.0[i];
        }
        true
    }
}

pub struct RedNosedReports {
    lines: Vec<ReportLine>
}

impl AocDay for RedNosedReports {
    fn new(content: String) -> Self {
        RedNosedReports {
            lines: content.lines().map(|x| ReportLine::from(x.to_string())).collect()
        }
    }

    fn part1(&self) -> String {
        let all_increasing_or_decreasing = self.lines.iter().filter(|x| x.is_all_increasing() || x.is_all_decreasing()).count();
        all_increasing_or_decreasing.to_string()
    }

    fn part2(&self) -> String { 
        let all_increasing_or_decreasing = self.lines.iter().filter(|x| x.is_same_direction_skip()).count();
        all_increasing_or_decreasing.to_string()
    }
}