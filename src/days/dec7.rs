use std::fmt::{Debug, Formatter};
use crate::runner::AocDay;

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
            Operation::Concatenate => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone)]
struct PartialTerm(i64, Operation);
#[derive(Clone)]
struct Term(i64, Vec<PartialTerm>);

impl Debug for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        for term in self.1.iter() {
            write!(f, " {:?} {}", term.1, term.0)?;
        }
        Ok(())
    }
}

impl Term {
    fn calculate(&self) -> i64 {
        let mut result = self.0;
        for term in self.1.iter() {
            match term.1 {
                Operation::Add => result += term.0,
                Operation::Multiply => result *= term.0,
                Operation::Concatenate => result = result*10_i64.pow((term.0 as f64).log10() as u32 + 1) + term.0,
            }
        }
        result
    }
}

struct Equation(i64, Vec<i64>);
impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let mut parts = s.split(":");
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
        Equation(first, second)
    }
}

impl Equation {
    fn solve_to_terms_partial(numbers_left: &[i64], current_terms: &Vec<Term>, do_concat: bool) -> Vec<Term> {
        if numbers_left.len() == 0 {
            return current_terms.clone();
        }

        let mut new_terms = vec![];
        for term in current_terms.iter() {
            let new_number = numbers_left[0];

            let mut new_term_add = term.clone();
            new_term_add.1.push(PartialTerm(new_number, Operation::Add));
            new_terms.push(new_term_add);

            let mut new_term_mul = term.clone();
            new_term_mul.1.push(PartialTerm(new_number, Operation::Multiply));
            new_terms.push(new_term_mul);

            if do_concat {
                let mut new_term_concat = term.clone();
                new_term_concat.1.push(PartialTerm(new_number, Operation::Concatenate));
                new_terms.push(new_term_concat);
            }
        }

        Equation::solve_to_terms_partial(&numbers_left[1..], &new_terms, do_concat)
    }


    fn solve_to_terms(&self, do_concat: bool) -> Vec<Term> {
        Equation::solve_to_terms_partial(&self.1[1..], &vec![
            Term(self.1[0], vec![]),
        ], do_concat)
    }

    fn solve(&self, do_concat: bool) -> Option<i64> {
        // use * or + to put between the numbers in the vector to get the target number
        // if it's possible to get the target number, return the number of ways to get it
        // otherwise, return None
        let target = self.0;
        let terms = self.solve_to_terms(do_concat);
        let mut ways = 0;
        for term in terms.iter() {
            if term.calculate() == target {
                ways += 1;
            }
        }
        if ways > 0 {
            Some(ways)
        } else {
            None
        }
    }
}

pub struct BridgeRepair {
    equations: Vec<Equation>,
}

impl AocDay for BridgeRepair {
    fn new(content: String) -> Self {
        BridgeRepair {
            equations: content.lines().map(|x| Equation::from(x)).collect()
        }
    }

    fn part1(&self) -> String {
        let mut sum = 0;
        for equation in &self.equations {
            if let Some(_) = equation.solve(false) {
                sum += equation.0;
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut sum = 0;
        for equation in &self.equations {
            if let Some(_) = equation.solve(true) {
                sum += equation.0;
            }
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_equation() {
        let equation = Equation::from("1: 2 3 4");
        assert_eq!(equation.0, 1);
        assert_eq!(equation.1, vec![2, 3, 4]);
    }

    #[test]
    fn test_solve_equation() {
        assert_eq!(Equation::from("5: 1 2 2").solve(false), Some(1));
        assert_eq!(Equation::from("8: 4 2").solve(false), Some(1));
        assert_eq!(Equation::from("16: 8 4 2").solve(false), None);
        assert_eq!(Equation::from("16: 4 2 8").solve(false), Some(1));
    }

    #[test]
    fn test_to_terms() {
        let equation = Equation::from("5: 1 2 2");
        let terms = equation.solve_to_terms(false);
        for term in terms.iter() {
            println!("{:?}", term);
        }
        assert_eq!(terms.len(), 4);
    }
}