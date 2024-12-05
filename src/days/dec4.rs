use crate::runner::AocDay;

// cross-word puzzle
pub struct DecemberFourth(Vec<Vec<char>>);

impl DecemberFourth {
    fn recurse_find(&self, x: i32, y: i32, chars_left: &str, dx: i32, dy: i32) -> i32 {
        if chars_left.len() == 0 {
            return 1;
        }

        if x < 0 || y < 0 || x >= self.0.len() as i32 || y >= self.0[x as usize].len() as i32 {
            return 0;
        }

        if self.0[x as usize][y as usize] != chars_left.chars().next().unwrap() {
            return 0;
        }

        self.recurse_find(x as i32 + dx as i32, y + dy, &chars_left[1..], dx, dy)
    }

    fn find(&self, chars: &str) -> i32 {
        let mut n = 0;
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                for dx in -1..2 {
                    for dy in -1..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        n += self.recurse_find(x as i32, y as i32, chars, dx, dy);
                    }
                }
            }
        }

        n
    }

    fn find_mas_x(&self) -> i32 {
        let mut n = 0;
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                let pos = self.0[x][y];
                if pos != 'A' || x < 1 || y < 1 || x >= self.0.len() - 1 || y >= self.0[x].len() - 1 {
                    continue;
                }

                let from_tl_to_br = self.0[x - 1][y - 1] == 'M' && self.0[x + 1][y + 1] == 'S';
                let from_br_to_tl = self.0[x - 1][y - 1] == 'S' && self.0[x + 1][y + 1] == 'M';

                let frm_bl_to_tr = self.0[x + 1][y - 1] == 'M' && self.0[x - 1][y + 1] == 'S';
                let frm_tr_to_bl = self.0[x + 1][y - 1] == 'S' && self.0[x - 1][y + 1] == 'M';

                if (from_tl_to_br || from_br_to_tl) && (frm_bl_to_tr || frm_tr_to_bl) {
                    n += 1;
                }
            }
        }

        n
    }
}

impl AocDay for DecemberFourth {
    fn new(content: String) -> Self {
        Self(content.lines().map(|x| x.chars().collect()).collect())
    }

    fn part1(&self) -> String {
        let all = self.find("XMAS");
        all.to_string()
    }

    fn part2(&self) -> String {
        let all = self.find_mas_x();
        all.to_string()
    }
}