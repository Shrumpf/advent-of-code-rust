use crate::common::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn name(&self) -> String {
        "Calorie Counting".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        input
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

    fn part_b(&self, input: String) -> String {
        let mut numbers: Vec<u32> = input
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

#[cfg(test)]
mod tests {
    use crate::{common, solutions};

    #[test]
    fn part_a() {
        let day = 1;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "24000");
    }

    #[test]
    fn part_b() {
        let day = 1;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "45000");
    }
}
