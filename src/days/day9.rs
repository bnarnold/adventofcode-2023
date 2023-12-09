use std::path::StripPrefixError;

use nom::{
    character::complete::{i64, space1},
    combinator::{eof, success},
    Parser,
};
use nom_supreme::{
    final_parser::final_parser,
    multi::{collect_separated_terminated, parse_separated_terminated},
    tag::complete::tag,
};

use crate::util::prelude::*;

#[derive(Debug, Default)]
struct SequencePredictor {
    coefficients: Vec<i64>,
}

impl Extend<i64> for SequencePredictor {
    fn extend<T: IntoIterator<Item = i64>>(&mut self, iter: T) {
        for x in iter {
            self.coefficients.push(x - self.predict_next())
        }
    }
}

impl SequencePredictor {
    fn predict(&self, n: i64) -> i64 {
        let mut binom = 1;
        let mut result = 0;
        for (k, coefficient) in self.coefficients.iter().enumerate() {
            let k = k as i64;
            result += binom * coefficient;
            binom *= n - k;
            binom /= k + 1;
        }
        result
    }

    fn predict_next(&self) -> i64 {
        self.predict(self.coefficients.len() as i64)
    }
}

fn parse_predictor(input: &str) -> ParseResult<SequencePredictor> {
    collect_separated_terminated(i64, space1, tag("\n")).parse(input)
}

pub fn level1(input: &str) -> i64 {
    let result: ParseFinalResult<_> = final_parser(parse_separated_terminated(
        parse_predictor,
        success(()),
        eof,
        || 0_i64,
        |x, predict| x + predict.predict_next(),
    ))(input);
    result.expect("parse error")
}

pub fn level2(input: &str) -> i64 {
        let result: ParseFinalResult<_> = final_parser(parse_separated_terminated(
        parse_predictor,
        success(()),
        eof,
        || 0_i64,
        |x, predict| x + predict.predict(-1),
    ))(input);
    result.expect("parse error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day9.txt");
        assert_eq!(level1(test_input), 114)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day9.txt");
        assert_eq!(level2(test_input), 2)
    }
}
