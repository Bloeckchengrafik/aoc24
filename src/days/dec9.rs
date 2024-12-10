use std::fmt::{Debug, Formatter};
use crate::runner::AocDay;

#[derive(Clone, Debug, PartialEq)]
enum DiskFragment {
    Used {
        id: usize,
    },
    Free,
}

#[derive(Clone)]
struct Fragments(Vec<DiskFragment>, usize);

impl Fragments {
    fn is_dense(&self) -> bool {
        let mut last_used = true;
        for fragment in self.0.iter() {
            match fragment {
                DiskFragment::Used { .. } => {
                    if !last_used {
                        return false;
                    }
                }
                DiskFragment::Free => {
                    last_used = false;
                }
            }
        }
        true
    }

    fn densify(&mut self) {
        while !self.is_dense() {
            let last_used = self.0.iter().rposition(|f| matches!(f, DiskFragment::Used { .. })).unwrap();
            let first_free = self.0.iter().position(|f| matches!(f, DiskFragment::Free)).unwrap();
            self.0.swap(last_used, first_free);
        }
    }

    fn densify_fast(&mut self) {
        let max_iters = self.0.len();
        for i in 0..max_iters {
            if i % 1000 == 0 {
                println!("Iteration {}/{}", i, max_iters);
            }
            let last_used = self.0.iter().rposition(|f| matches!(f, DiskFragment::Used { .. })).unwrap();
            let first_free = self.0.iter().position(|f| matches!(f, DiskFragment::Free)).unwrap();
            if last_used < first_free {
                break;
            }
            self.0.swap(last_used, first_free);
        }
    }

    fn densify_blocks(&mut self) {
        let max_id = self.1;
        for id in (0..max_id).rev() {
            if id % 1000 == 0 {
                println!("Iteration {}/{}", max_id - id, max_id);
            }
            let used_frag = DiskFragment::Used { id };
            let first_pos = self.0.iter().position(|f| f == &used_frag).unwrap();
            let last_pos = self.0.iter().rposition(|f| f == &used_frag).unwrap();
            let len = last_pos - first_pos + 1;
            let free_frag = DiskFragment::Free;
            let mut free_start = 0;
            while free_start < first_pos {
                let mut all_free = true;
                let mut advance = 1;
                for i in 0..len {
                    if self.0[free_start + i] != free_frag {
                        all_free = false;
                        if i > 0 {
                            advance = i;
                        }
                        break;
                    }
                }

                if all_free {
                    for i in 0..len {
                        self.0.swap(free_start + i, first_pos + i);
                    }

                    break;
                } else {
                    free_start += advance;
                }
            }
        }
    }

    fn checksum(&self) -> i64 {
        let mut checksum = 0i64;
        for (i, fragment) in self.0.iter().enumerate() {
            match fragment {
                DiskFragment::Used { id } => checksum += (*id * i) as i64,
                DiskFragment::Free => {}
            }
        }

        checksum
    }
}

impl Debug for Fragments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for fragment in self.0.iter() {
            match fragment {
                DiskFragment::Used { id } => write!(f, "{}", id)?,
                DiskFragment::Free => write!(f, ".")?,
            }
        }

        Ok(())
    }
}

pub struct DiskFragmenter {
    fragments: Fragments,
}

impl AocDay for DiskFragmenter {
    fn new(content: String) -> Self {
        let mut fragments = Vec::new();
        let mut id = 0;
        let mut last_space = true;

        for c in content.as_str().chars() {
            let width: usize = c.to_string().parse().unwrap();
            if last_space {
                for _ in 0..width {
                    fragments.push(DiskFragment::Used { id });
                }
                id += 1;
            } else {
                for _ in 0..width {
                    fragments.push(DiskFragment::Free);
                }
            }
            last_space = !last_space;
        }

        Self { fragments: Fragments(fragments, id) }
    }

    fn part1(&self) -> String {
        let mut fragments = self.fragments.clone();
        fragments.densify_fast();
        fragments.checksum().to_string()
    }

    fn part2(&self) -> String {
        let mut fragments = self.fragments.clone();
        fragments.densify_blocks();
        fragments.checksum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::AocDay;
    use test::{Bencher, black_box};

    #[test]
    fn test_parse() {
        let content = "12345";
        let disk_fragmenter = DiskFragmenter::new(content.to_string());
        assert_eq!(disk_fragmenter.fragments.0.len(), "0..111....22222".len());
        assert!(matches!(disk_fragmenter.fragments.0[0], DiskFragment::Used { .. }));
        assert!(matches!(disk_fragmenter.fragments.0[1], DiskFragment::Free));
        assert!(matches!(disk_fragmenter.fragments.0[2], DiskFragment::Free));
        assert!(matches!(disk_fragmenter.fragments.0[3], DiskFragment::Used { .. }));
    }

    #[test]
    fn test_dense() {
        let not_dense = DiskFragmenter::new("12345".to_string());
        assert!(!not_dense.fragments.is_dense());

        let dense = DiskFragmenter::new("90909".to_string());
        assert!(dense.fragments.is_dense());
    }

    #[test]
    fn test_densify() {
        let mut not_dense = DiskFragmenter::new("12345".to_string());
        not_dense.fragments.densify();
        assert!(not_dense.fragments.is_dense());
    }

    #[test]
    fn test_densify_fast() {
        let mut not_dense = DiskFragmenter::new("12345".to_string());
        not_dense.fragments.densify_fast();
        assert!(not_dense.fragments.is_dense());
    }

    #[bench]
    fn bench_densify(b: &mut Bencher) {
        let not_dense = DiskFragmenter::new("12345".to_string());

        b.iter(|| {
            let mut fragments = not_dense.fragments.clone();
            black_box(fragments.densify());
        });
    }

    #[bench]
    fn bench_densify_fast(b: &mut Bencher) {
        let not_dense = DiskFragmenter::new("12345".to_string());

        b.iter(|| {
            let mut fragments = not_dense.fragments.clone();
            black_box(fragments.densify_fast());
        });
    }

    #[test]
    fn test_densify_blocks() {
        let mut disk_fragmenter = DiskFragmenter::new("90909".to_string());
        disk_fragmenter.fragments.densify_blocks();
        assert_eq!(disk_fragmenter.fragments.checksum(), 513);

        let mut disk_fragmenter = DiskFragmenter::new("133".to_string());
        disk_fragmenter.fragments.densify_blocks();
        assert_eq!(disk_fragmenter.fragments.checksum(), 6);

        let mut disk_fragmenter = DiskFragmenter::new("12302".to_string());
        disk_fragmenter.fragments.densify_blocks();
        assert_eq!(disk_fragmenter.fragments.checksum(), 18);
    }

    #[test]
    fn test_checksum() {
        let mut disk_fragmenter = DiskFragmenter::new("123".to_string());
        disk_fragmenter.fragments.densify();
        assert_eq!(disk_fragmenter.fragments.checksum(), 6);
    }
}