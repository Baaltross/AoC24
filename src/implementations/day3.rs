use std::io::{BufRead, BufReader};

use crate::utility::generic_error::GenericResult;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    Unknown(char),
    Integer(usize),
    Multiply,
    OpenParen,
    CloseParen,
    Comma,
    Do,
    Dont,
}

trait TokenBuilder {
    fn try_consume_characters(&self, characters : &str, token_output : &mut Vec<Token>) -> usize;
}

struct StringTokenBuilder<'a>
{
    string_to_match : &'a str,
    token_to_output : Token,
}

impl<'a> StringTokenBuilder<'a> {
    fn new(string_to_match : &'a str, token_to_output: Token) -> Self {
        StringTokenBuilder {
            string_to_match : string_to_match,
            token_to_output : token_to_output }
    }
}

impl TokenBuilder for StringTokenBuilder<'_> {
    fn try_consume_characters(&self, characters : &str, token_output : &mut Vec<Token>) -> usize {
        if characters.starts_with(self.string_to_match) {
            token_output.push(self.token_to_output.clone());
            return self.string_to_match.len();
        }

        0
    }
}
struct IntegerTokenBuilder
{}

impl IntegerTokenBuilder {
    fn new() -> Self {
        IntegerTokenBuilder {}
    }
}

impl TokenBuilder for IntegerTokenBuilder {
    fn try_consume_characters(&self, characters : &str, token_output : &mut Vec<Token>) -> usize {
        let mut matched_so_far = 0;
        let mut value_so_far = 0;

        for character in characters.chars() {
            if let Some(digit) = character.to_digit(10) {
                value_so_far *= 10;
                value_so_far += digit;
                matched_so_far += 1;
            } else {
                break;
            }
        }

        if matched_so_far > 0 {
            token_output.push(Token::Integer(value_so_far as usize));
        }

        matched_so_far
    }
}


fn tokenise(input : &str, support_do_dont : bool) -> Vec<Token> {
    let mut result = Vec::new();

    let mut tokenisers : Vec<Box<dyn TokenBuilder>> = vec![
        Box::new(StringTokenBuilder::new("mul", Token::Multiply)),
        Box::new(StringTokenBuilder::new("(", Token::OpenParen)),
        Box::new(StringTokenBuilder::new(")", Token::CloseParen)),
        Box::new(StringTokenBuilder::new(",", Token::Comma)),
        Box::new(IntegerTokenBuilder::new())];

    if support_do_dont {
        let mut do_dont_tokenisers : Vec<Box<dyn TokenBuilder>> = vec![
            Box::new(StringTokenBuilder::new("do()", Token::Do)),
            Box::new(StringTokenBuilder::new("don't()", Token::Dont)),
        ];
        tokenisers.append(&mut do_dont_tokenisers);
    }

    let mut remaining = input;

    while remaining.len() > 0 {

        let mut consumed = 0;
        for tokeniser in tokenisers.iter() {
            consumed = tokeniser.try_consume_characters(remaining, &mut result);
            if consumed > 0 {
                break;
            }
        }

        if consumed > 0 {
            remaining = &remaining[consumed..];
        } else {
            result.push(Token::Unknown(remaining.chars().next().unwrap()));
            remaining = &remaining[1..];
        }
    }

    result
}

fn calculate_output(input_path : &str, support_do_dont : bool) -> GenericResult<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result : usize = 0;

    let mut enabled = true;

    for line in reader.lines() {
        let line = line?;
        let tokens = tokenise(line.as_str(), support_do_dont);

        for index in 0..tokens.len() {
            let test = &tokens[index..];
            match test {
                [Token::Multiply
                , Token::OpenParen
                , Token::Integer(value1)
                , Token::Comma
                , Token::Integer(value2)
                , Token::CloseParen
                , ..] => {
                    if enabled {
                        result += value1 * value2
                    }
                }
                [Token::Do, ..] => { enabled = true }
                [Token::Dont, ..] => { enabled = false }
                _ => {
                    // Do nothing
                }
            }
        }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(calculate_output("data/day3/example_part1.txt", false)?, 161);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(calculate_output("data/day3/example_part2.txt", true)?, 48);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", calculate_output(&input_path, false).unwrap());
    // Part 2
    println!("Part two result: {}", calculate_output(&input_path, true).unwrap());
    Ok(())
}