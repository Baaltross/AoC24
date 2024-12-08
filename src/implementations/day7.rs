use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::utility::generic_error::{GenericError, GenericResult};

#[derive(Debug)]
struct Equation {
    expected_result: usize,
    equation_elements: Vec<usize>,
}

impl FromStr for Equation {
    type Err = GenericError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split_input = input.split(':');
        let expected_result = split_input.next().unwrap().parse()?;
        let equation_elements :Vec<usize> = split_input.next().unwrap().trim().split(' ').map(|s| s.parse().unwrap()).collect();
        Ok(Equation{expected_result, equation_elements})
    }
}

fn load_equations(input_path : &str) -> GenericResult<Vec<Equation>> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result: Vec<Equation> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        result.push(Equation::from_str(&line)?);
    }

    Ok(result)
}

fn recursive_fn_part1(test_result: usize, result_so_far:usize, remaining_components: &[usize]) -> bool {
    if remaining_components.len() == 0 {
        return result_so_far == test_result;
    }

    if result_so_far > test_result {
        return false;
    }

    recursive_fn_part1(test_result, result_so_far + &remaining_components[0], &remaining_components[1..])
        || recursive_fn_part1(test_result, result_so_far * &remaining_components[0], &remaining_components[1..])
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let equations = load_equations(input_path)?;

    let mut result = 0;
    for equation in equations {
        if recursive_fn_part1(equation.expected_result, equation.equation_elements[0], &equation.equation_elements[1..]) {
            result += equation.expected_result;
        }
    }

    Ok(result)
}

fn recursive_fn_part2(test_result: usize, result_so_far:usize, remaining_components: &[usize]) -> bool {
    if remaining_components.len() == 0 {
        return result_so_far == test_result;
    }

    if result_so_far > test_result {
        return false;
    }

    let concatenated_value = (result_so_far.to_string() + remaining_components[0].to_string().as_str()).parse().unwrap();

    recursive_fn_part2(test_result, concatenated_value, &remaining_components[1..])
        || recursive_fn_part2(test_result, result_so_far + &remaining_components[0], &remaining_components[1..])
        || recursive_fn_part2(test_result, result_so_far * &remaining_components[0], &remaining_components[1..])
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let equations = load_equations(input_path)?;

    let mut result = 0;
    for equation in equations {
        if recursive_fn_part2(equation.expected_result, equation.equation_elements[0], &equation.equation_elements[1..]) {
            result += equation.expected_result;
        }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day7/example.txt")?, 3749);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day7/example.txt")?, 11387);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", part_1(&input_path).unwrap());
    // Part 2
    println!("Part two result: {}", part_2(&input_path).unwrap());
    Ok(())
}