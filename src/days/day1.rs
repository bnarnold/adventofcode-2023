use crate::util::prelude::*;

fn parse_line(line: &str) -> Option<u32> {
    let (first, last) = line.chars().fold((None, None), |(first, last), c| {
        if let Some(d) = c.to_digit(10) {
            (first.or(Some(d)), Some(d))
        } else {
            (first, last)
        }
    });
    Some(first? * 10 + last?)
}
fn parse_line_with_words(line: &str) -> Option<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let regex = regex::Regex::new(&format!(r#"(\d|{})"#, digits.iter().join("|"))).unwrap();
    let first_match = regex.find(line)?.as_str();
    let first = match digits.iter().position(|digit| *digit == first_match) {
        Some(i) => Some(i as u32 + 1),
        None => first_match.parse().ok(),
    }?;

    let reversed_digits: Vec<String> = digits.iter().map(|d| d.chars().rev().collect()).collect();
    let reversed_line: String = line.chars().rev().collect();
    let reversed_regex =
        regex::Regex::new(&format!(r#"(\d|{})"#, reversed_digits.iter().join("|"))).unwrap();
    let last_match = &reversed_regex.find(&reversed_line)?.as_str();
    let last = match reversed_digits.iter().position(|digit| digit == last_match) {
        Some(i) => Some(i as u32 + 1),
        None => last_match.parse().ok(),
    }?;

    Some(first * 10 + last)
}

pub fn level1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line).expect("Line contains no digits"))
        .sum()
}

pub fn level2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line_with_words(line).expect("Line contains no digits"))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day1.txt");
        assert_eq!(level1(test_input), 142)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day1_large.txt");
        assert_eq!(level2(test_input), 281)
    }
}
