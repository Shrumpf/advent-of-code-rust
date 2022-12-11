use crate::common::Solution;

pub struct Day06 {}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

impl Solution for Day06 {
    fn name(&self) -> String {
        "Tuning Trouble".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        let chars = input.chars().collect::<Vec<char>>();
        let mut score = 3;
        for i in 0..chars.len() {
            score += 1;
            let mut ch: Vec<char> = Vec::new();
            for j in 0..4 {
                ch.push(chars[i + j]);
            }

            let str = String::from_iter(ch);
            print!("\"{}\" (length {})", str, str.chars().count());
            match unique(&str) {
                None => {
                    break;
                }
                Some((i, j, c)) => println!(
                    " is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
                    c, c as usize, i, j
                ),
            }
        }

        score.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let chars = input.chars().collect::<Vec<char>>();
        let mut score = 13;
        for i in 0..chars.len() {
            score += 1;

            let mut ch: Vec<char> = Vec::new();
            for j in 0..14 {
                ch.push(chars[i + j]);
            }

            let str = String::from_iter(ch);
            print!("\"{}\" (length {})", str, str.chars().count());
            match unique(&str) {
                None => {
                    break;
                }
                Some((i, j, c)) => println!(
                    " is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
                    c, c as usize, i, j
                ),
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
        let day = 6;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "10");
    }

    #[test]
    fn part_b() {
        let day = 6;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "19");
    }
}
