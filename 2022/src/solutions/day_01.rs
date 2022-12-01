use crate::common::{self, Solution};

pub struct Day01 {}

impl Solution for Day01 {
    fn name(&self) -> String {
        "Calorie Counting".to_owned()
    }

    fn part_a(&self) -> String {
        common::load("01")
            .split("\n\n")
            .map(|f| {
                f.split('\n')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|s| s.iter().sum())
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
            .to_string()
    }

    fn part_b(&self) -> String {
        let mut numbers: Vec<u32> = common::load("01")
            .split("\n\n")
            .map(|f| {
                f.split('\n')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|s| s.iter().sum())
            .collect::<Vec<u32>>();
        numbers.sort();
        numbers.reverse();
        let max = vec![numbers[0], numbers[1], numbers[2]].iter().sum::<u32>();

        max.to_string()
    }
}
