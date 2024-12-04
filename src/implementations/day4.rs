use std::io::{BufRead, BufReader};

use crate::utility::generic_error::{GenericError, GenericResult};

fn load_grid(input_path : &str) -> GenericResult<(String, i64)> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut width : i64 = 0;
    let mut peekable_lines = reader.lines().peekable();
    if let Some(Ok(first_line)) = peekable_lines.peek() {
        width = first_line.len() as i64;
    }
    
    if width == 0 {
        return Err(GenericError::BasicError("Failed to get first line".to_string()));
    }

    let grid : String = peekable_lines.map(|s| s.unwrap()).collect();
    Ok((grid, width))
}

fn get_value_at_index(grid : &String, index: (i64, i64), width : i64) -> Option<u8> {
    let test_index = index.1 * width + index.0;

    // Note test against width since that would wrap around to the next line
    if test_index < 0 || (test_index >= grid.len() as i64)
    || index.0 < 0 || index.0 >= width {
        return None;
    }

    Some(grid.as_bytes()[test_index as usize])
}

fn check_for_xmas(grid: &String, initial_index : (i64, i64), offset : (i64, i64), width : i64) -> bool {
    let search_chars = "XMAS";
    
    // Note 1 because we already know that the X is at initial_index
    for search_index in 1..(search_chars.len() as i64) {
        let x_index = initial_index.0 + (search_index * offset.0);
        let y_index = initial_index.1 + (search_index * offset.1);

        if let Some(value_at_index) = get_value_at_index(&grid, (x_index, y_index), width) {
            if value_at_index == search_chars.as_bytes()[search_index as usize] {
                // Match, continue to checking next character
                continue;
            }
        }

        // Mismatch
        return false;
    }

    // Successfully matched all search chars 
    true
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let mut result = 0;
    let (grid, stride) = load_grid(input_path)?;

    let offsets_to_test = [
        (0, -1), // Up
        (0, 1), // Down
        (-1, 0), // Left
        (1, 0), // Right
        (-1, -1), // Up Left
        (-1, 1), // Up Right
        (1, -1), // Down Left
        (1, 1), // Down Right
    ];

    for index in grid.match_indices('X') {
        let x_index = index.0 as i64 % stride;
        let y_index = index.0 as i64 / stride;
        for offset in offsets_to_test.iter() {
            if check_for_xmas(&grid, (x_index, y_index), *offset, stride) {
                result += 1;
            }
        }
    }

    Ok(result)
}

// offset passed should be one of the diagonal offsets, this checks it and its opposite
fn check_for_diagonal_mas(grid: &String, initial_index : (i64, i64), offset : (i64, i64), width : i64) -> bool {
    assert!(offset.0.abs() == 1 && offset.1.abs() == 1);

    if let Some(value_at_initial_corner) = get_value_at_index(&grid, (initial_index.0 + offset.0, initial_index.1 + offset.1), width) {
        if value_at_initial_corner as char == 'M' {
            if let Some(value_at_opposite_corner) = get_value_at_index(&grid, (initial_index.0 - offset.0, initial_index.1 - offset.1), width) {
                return value_at_opposite_corner as char == 'S';
            }
        } else if value_at_initial_corner as char == 'S' {
            if let Some(value_at_opposite_corner) = get_value_at_index(&grid, (initial_index.0 - offset.0, initial_index.1 - offset.1), width) {
                return value_at_opposite_corner as char == 'M';
            }
        }
    }

    false
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let mut result = 0;
    let (grid, stride) = load_grid(input_path)?;

    for index in grid.match_indices('A') {
        let x_index = index.0 as i64 % stride;
        let y_index = index.0 as i64 / stride;

        if check_for_diagonal_mas(&grid, (x_index, y_index), (-1,-1), stride)
            && check_for_diagonal_mas(&grid, (x_index, y_index), (-1,1), stride) {
                result += 1;
            }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day4/example_part1.txt")?, 18);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day4/example_part2.txt")?, 9);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", part_1(&input_path).unwrap());
    // Part 2
    println!("Part two result: {}", part_2(&input_path).unwrap());
    Ok(())
}