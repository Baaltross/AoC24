use std::{collections::HashMap, io::{BufRead, BufReader}};

use crate::utility::generic_error::GenericResult;

fn populate_lists_from_file(file_handle : std::fs::File) -> (Vec<i32>, Vec<i32>) {
    let reader = BufReader::new(file_handle);

    let mut list1 : Vec<i32> = Vec::new();
    let mut list2 : Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let words = line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        let values: Vec<i32> = words.map(|s| s.parse().unwrap()).collect();

        println!("Values: {} {}", values[0], values[1]);
        list1.push(values[0]);
        list2.push(values[1]);
    }

    (list1, list2)
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let mut lists = populate_lists_from_file(file_handle);

    lists.0.sort();
    lists.1.sort();

    let mut result = 0;

    for value_pair in lists.0.iter().zip(lists.1.iter()) {
        result += ((value_pair.0 - value_pair.1).abs()) as usize;
    }

    println!("Part one result: {}", result);

    Ok(result)
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let lists = populate_lists_from_file(file_handle);

    let mut occurrences_in_list2 : HashMap<i32, usize> = HashMap::new();

    for value in lists.1 {
        (*occurrences_in_list2.entry(value).or_insert(0)) += 1
    }

    let mut result = 0;

    for value in lists.0 {
        result += (value as usize) * occurrences_in_list2.get(&value).unwrap_or(&0);
    }

    println!("Part one result: {}", result);

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day1/example.txt")?, 11);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day1/example.txt")?, 31);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    part_1(&input_path)?;
    part_2(&input_path)?;
    Ok(())
}