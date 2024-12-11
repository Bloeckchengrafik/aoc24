use std::collections::HashMap;

use crate::runner::AocDay;

type Int = i128;

pub struct PlutonianPebbles{
    pebbles: Vec<Int>
}

fn count_stones_emerging(cache: &mut HashMap<(Int, usize), Int>, pebble: Int, depth: usize) -> Int {
    if depth == 0 {
        return 1;
    }

    if let Some(k) = cache.get(&(pebble, depth)) {
        return *k;
    }

    let result = if pebble == 0 {
        count_stones_emerging(cache, 1, depth - 1)
    } else {
        let pebble_string = pebble.to_string();
        let len = pebble_string.len();

        if len % 2 == 0 {
            // split in the middle
            let middle = len / 2;
            let left_pebble = pebble_string[0..middle].parse().unwrap(); 
            let right_pebble = pebble_string[middle..len].parse().unwrap();

            count_stones_emerging(cache, left_pebble, depth - 1) + count_stones_emerging(cache, right_pebble, depth - 1)
        } else {
            count_stones_emerging(cache, pebble * 2024, depth - 1)
        }
    };

    cache.insert((pebble, depth), result);

    result
}

impl AocDay for PlutonianPebbles {
    fn new(content: String) -> Self {
        Self {
            pebbles: content.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect()
        }
    }

    fn part1(&self) -> String {
        let mut cache = HashMap::new();
        let mut sum: Int = 0;
        for pebble in &self.pebbles {
            sum += count_stones_emerging(&mut cache, *pebble, 25);
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut cache = HashMap::new();
        let mut sum: Int = 0;
        for pebble in &self.pebbles {
            sum += count_stones_emerging(&mut cache, *pebble, 75);
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::days::dec11::count_stones_emerging;

    #[test]
    fn test_count_rule1() {
        let mut cache = HashMap::new();
        assert_eq!(1, count_stones_emerging(&mut cache, 0, 1));
    }

    #[test]
    fn test_count_rule2() {
        let mut cache = HashMap::new();
        assert_eq!(2, count_stones_emerging(&mut cache, 10, 1));
    }

    #[test]
    fn test_count_rule2_multi() {
        let mut cache = HashMap::new();
        assert_eq!(2, count_stones_emerging(&mut cache, 10, 2));
        let mut cache = HashMap::new();
        assert_eq!(3, count_stones_emerging(&mut cache, 10, 3));
    }

    #[test]
    fn test_count_rule3() {
        let mut cache = HashMap::new();
        assert_eq!(1, count_stones_emerging(&mut cache, 101, 1));
    }
}