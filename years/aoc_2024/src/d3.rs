use std::{fs, path::Path};

use anyhow::{Context, Result};
use cli::{part::Part, util::file_path};
use pest::Parser;
use pest_derive::Parser;

use crate::example_dir_for_day;

#[derive(Parser)]
#[grammar = "grammars/d3.pest"]
struct D3Parser;

enum Token {
    Mul { lhs: i64, rhs: i64 },
    Do,
    Dont,
}

pub(crate) fn run(base_dir: &Path, part: Part, example: bool) -> Result<String> {
    let path = file_path(&example_dir_for_day(base_dir, 3), part, example);
    let tokens = parse_file(&path)?;
    let solution = match part {
        Part::P1 => solve_p1(&tokens),
        Part::P2 => solve_p2(&tokens),
    };
    Ok(solution.to_string())
}

fn eval(tokens: &[Token], ignore_control_flow: bool) -> i64 {
    let mut enabled = true;
    let mut total = 0;
    for token in tokens {
        match token {
            Token::Mul { lhs, rhs } => {
                if ignore_control_flow || enabled {
                    total += lhs * rhs;
                }
            }
            Token::Do => {
                enabled = true;
            }
            Token::Dont => {
                enabled = false;
            }
        }
    }
    total
}

fn parse_input(input: &str) -> Result<Vec<Token>> {
    let mut values = vec![];
    let parsed = D3Parser::parse(Rule::root, input).context("invalid parse")?;
    for pair in parsed.flatten() {
        match pair.as_rule() {
            Rule::mul_expr => {
                let mut inner = pair.into_inner();
                let first_digits = inner.next().expect("bad grammar: first digit").as_str();
                let second_digits = inner.next().expect("bad grammar: second digit").as_str();
                let lhs: i64 = first_digits
                    .parse()
                    .expect("bad grammar: first digit not numeric");
                let rhs: i64 = second_digits
                    .parse()
                    .expect("bad grammar: second digit not numeric");
                values.push(Token::Mul { lhs, rhs });
            }
            Rule::eval_do => {
                values.push(Token::Do);
            }
            Rule::eval_dont => {
                values.push(Token::Dont);
            }
            _ => {}
        }
    }
    Ok(values)
}

fn parse_file(path: &Path) -> Result<Vec<Token>> {
    let input: String = fs::read_to_string(path)?;
    parse_input(&input)
}

fn solve_p1(tokens: &[Token]) -> i64 {
    eval(tokens, true)
}

fn solve_p2(tokens: &[Token]) -> i64 {
    eval(tokens, false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use cli::util::default_data_dir;

    #[test]
    fn test_example_1() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 3), Part::P1, true);
        let tokens = parse_file(&path).unwrap();
        assert_eq!(161, solve_p1(&tokens));
    }

    #[test]
    fn test_example_2() {
        let path = file_path(&example_dir_for_day(&default_data_dir(), 3), Part::P2, true);
        let tokens = parse_file(&path).unwrap();
        assert_eq!(48, solve_p2(&tokens));
    }

    #[test]
    fn test_part_1() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 3),
            Part::P1,
            false,
        );
        let tokens = parse_file(&path).unwrap();
        assert_eq!(189_600_467, solve_p1(&tokens));
    }

    #[test]
    fn test_part_2() {
        let path = file_path(
            &example_dir_for_day(&default_data_dir(), 3),
            Part::P2,
            false,
        );
        let tokens = parse_file(&path).unwrap();
        assert_eq!(107_069_718, solve_p2(&tokens));
    }
}
