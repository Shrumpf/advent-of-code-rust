use crate::common::{self, Solution};

pub struct Day01 {}

impl Solution for Day01 {
    fn name(&self) -> String {
        "Sonar Sweep".to_owned()
    }

    fn part_a(&self) -> String {
        let numbers = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut sum = 0;

        let mut first;
        let mut second = 122;

        for line in numbers {
            first = second;
            second = line;

            if second > first {
                sum += 1;
            }
        }
        sum.to_string()
    }

    fn part_b(&self) -> String {
        let numbers = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut sum = 0;
        let mut first;
        let mut second = 367;

        for i in 1..numbers.len() - 2 {
            first = second;
            second = numbers[i] + numbers[i + 1] + numbers[i + 2];

            if second > first {
                sum += 1;
            }
        }

        sum.to_string()
    }
}
