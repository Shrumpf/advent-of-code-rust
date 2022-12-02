use crate::common::Solution;
use std::collections::HashSet;

pub struct Day03 {}

impl Solution for Day03 {
    fn name(&self) -> String {
        "Rucksack Reorganization".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        let num = input
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .collect::<Vec<(&str, &str)>>();
        let mut score = 0;
        for c in num {
            let mut a = HashSet::new();
            let mut b = HashSet::new();

            for c in c.0.chars() {
                a.insert(c);
            }

            for c in c.1.chars() {
                b.insert(c);
            }

            let intersected = a.intersection(&b).collect::<Vec<&char>>()[0];

            if intersected.is_lowercase() {
                score += (*intersected as i16) - 96;
            } else {
                score += (*intersected as i16) - 64 + 26;
            }
        }
        score.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let binding = input.split('\n').collect::<Vec<&str>>();
        let groups = binding.chunks(3).collect::<Vec<&[&str]>>();
        let mut score = 0;
        for group in groups {
            let gv = group.to_vec();

            let mut a = HashSet::new();
            let mut b = HashSet::new();
            let mut c = HashSet::new();

            for ch in gv[0].chars().collect::<Vec<char>>().iter() {
                a.insert(*ch);
            }

            for ch in gv[1].chars().collect::<Vec<char>>().iter() {
                b.insert(*ch);
            }

            for ch in gv[2].chars().collect::<Vec<char>>().iter() {
                c.insert(*ch);
            }

            let ab = a.intersection(&b).collect::<Vec<&char>>();

            let mut ab_hs = HashSet::new();

            for ch in ab {
                ab_hs.insert(*ch);
            }

            let abc = ab_hs.intersection(&c).collect::<Vec<&char>>()[0];

            if abc.is_lowercase() {
                score += (*abc as i16) - 96;
            } else {
                score += (*abc as i16) - 64 + 26;
            }
        }
        score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{common, solutions};

    #[test]
    fn part_a() {
        let day = 3;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "157");
    }

    #[test]
    fn part_b() {
        let day = 3;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "70");
    }
}
