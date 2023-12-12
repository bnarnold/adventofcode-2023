use std::collections::HashMap;
use std::iter::{once, repeat};

fn way_count(patterns: impl IntoIterator<Item = char>, lengths: &[usize]) -> usize {
    let mut way_count: HashMap<(usize, Option<usize>), usize> = HashMap::default();
    way_count.insert((0, None), 1);
    for c in patterns.into_iter().chain(once('.')) {
        let mut new_way_count = HashMap::default();
        if c == '#' || c == '?' {
            for (&(offset, length), &count) in &way_count {
                match length {
                    Some(length) if length > 0 => {
                        *new_way_count.entry((offset, Some(length - 1))).or_default() += count;
                    }
                    None if offset < lengths.len() => {
                        *new_way_count
                            .entry((offset + 1, Some(lengths[offset] - 1)))
                            .or_default() += count;
                    }
                    _ => {}
                }
            }
        }
        if c == '.' || c == '?' {
            for ((offset, length), count) in way_count {
                if length.unwrap_or(0) == 0 {
                    *new_way_count.entry((offset, None)).or_default() += count;
                }
            }
        }
        way_count = new_way_count;
    }
    way_count
        .get(&(lengths.len(), None))
        .copied()
        .unwrap_or_default()
        + way_count
            .get(&(lengths.len(), Some(0)))
            .copied()
            .unwrap_or_default()
}

fn find_way_counts(input: &str, repeat_count: usize) -> usize {
    input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|line| {
            let (patterns, lengths) = line.split_once(' ').expect("No space");
            let mut repeated_patterns = patterns.to_string();
            repeated_patterns.extend(repeat(["?", patterns]).take(repeat_count - 1).flatten());
            let lengths: Vec<usize> = lengths
                .split(',')
                .map(|x| x.parse().expect("parse length"))
                .collect();
            let lengths: Vec<usize> = repeat(lengths).take(repeat_count).flatten().collect();
            way_count(repeated_patterns.chars(), &lengths)
        })
        .sum()
}

fn main() {
    let input = r"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
    ";
    println!("{}", find_way_counts(input, 1));
    println!("{}", find_way_counts(input, 5));
}
