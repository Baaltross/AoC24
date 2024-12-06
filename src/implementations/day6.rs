use std::io::{BufRead, BufReader};

use crate::utility::generic_error::{GenericError, GenericResult};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    blockages: Vec<bool>,
    width: i64,
}

type Coordinates = (i64, i64);

impl Grid {
    fn convert_coordinates_to_index(&self, coordinates : Coordinates) -> i64 {
        return coordinates.1 * self.width + coordinates.0;
    }

    fn is_blocked(&self, index: Coordinates) -> Option<bool> {
        let test_index = self.convert_coordinates_to_index(index);
    
        // Note test against width since that would wrap around to the next line
        if test_index < 0 || (test_index >= self.blockages.len() as i64)
        || index.0 < 0 || index.0 >= self.width {
            return None;
        }
    
        Some(self.blockages[test_index as usize])
    }
}

fn load_grid(input_path : &str) -> GenericResult<(Grid, Coordinates, Direction)> {
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

    let mut blockages: Vec<bool> = Vec::new();
    let mut guard_location : Coordinates = (-1, -1);
    let mut guard_facing : Direction = Direction::Up;

    for (y_index,line) in peekable_lines.enumerate() {
        let line = line?;
        for (x_index, character) in line.chars().enumerate() {
            let mut is_blocked = false;
            match character {
                '.' => {}
                '#' => { is_blocked = true; }
                '^' => { guard_location = (x_index as i64, y_index as i64); guard_facing = Direction::Up; }
                'v' => { guard_location = (x_index as i64, y_index as i64); guard_facing = Direction::Down; }
                '<' => { guard_location = (x_index as i64, y_index as i64); guard_facing = Direction::Left; }
                '>' => { guard_location = (x_index as i64, y_index as i64); guard_facing = Direction::Right; }
                _ => { assert!(false, "Invalid character {}", character); }
            }
            blockages.push(is_blocked);
        }
    }

    if guard_location == (-1,-1) {
        return Err(GenericError::BasicError("Did not find guard location".to_string()));
    }

    Ok((Grid{blockages, width}, guard_location, guard_facing))
}

fn coordinate_in_direction(coordinate: &Coordinates, direction: &Direction) -> Coordinates {
    match direction {
        Direction::Up => return (coordinate.0, coordinate.1 - 1),
        Direction::Down => return (coordinate.0, coordinate.1 + 1),
        Direction::Left => return (coordinate.0 - 1, coordinate.1),
        Direction::Right => return (coordinate.0 + 1, coordinate.1),
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => return Direction::Right,
        Direction::Down => return Direction::Left,
        Direction::Left => return Direction::Up,
        Direction::Right => return Direction::Down,
    }
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let (grid, mut guard_location, mut guard_facing) = load_grid(input_path)?;

    //println!("Grid: {:?}", grid);

    let mut visited : Vec<bool> = Vec::new();
    visited.resize(grid.blockages.len(), false);

    visited[grid.convert_coordinates_to_index(guard_location) as usize] = true;

    loop {
        let new_location = coordinate_in_direction(&guard_location, &guard_facing);

        if let Some(is_blocked) = grid.is_blocked(new_location) {
            if is_blocked {
                guard_facing = turn_right(&guard_facing);
            } else {
                guard_location = new_location;
                visited[grid.convert_coordinates_to_index(guard_location) as usize] = true;
            }
        } else {
            // We've left the grid
            break;
        }
    }

    let result = visited.iter().filter(|s| **s).count();
    //println!("Visited: {:?}", visited);
    Ok(result)
}

fn does_infinite_loop(grid: &Grid, mut visited:Vec<bool>, start_location:Coordinates, start_facing:Direction) -> bool {
    let grid_size = grid.blockages.len();
    let mut current_location = start_location;
    let mut current_facing = start_facing;

    loop {
        let new_location = coordinate_in_direction(&current_location, &current_facing);

        if let Some(is_blocked) = grid.is_blocked(new_location) {
            if is_blocked {
                current_facing = turn_right(&current_facing);
            } else {
                current_location = new_location;
            }

            let visited_index = grid.convert_coordinates_to_index(current_location) as usize + (grid_size * current_facing as usize);
            if visited[visited_index] {
                // Infinite loop detected
                return true;
            }

            visited[visited_index] = true;
        } else {
            // We've left the grid
            break;
        }
    }

    false
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let mut result : Vec<Coordinates> = Vec::new();

    let (mut grid, mut guard_location, mut guard_facing) = load_grid(input_path)?;
    let grid_size = grid.blockages.len();

    let mut visited : Vec<bool> = Vec::new();
    visited.resize(grid_size * 4, false);
    visited[grid.convert_coordinates_to_index(guard_location) as usize + (grid_size * guard_facing as usize)] = true;

    loop {
        let new_location = coordinate_in_direction(&guard_location, &guard_facing);

        if let Some(is_blocked) = grid.is_blocked(new_location) {
            if is_blocked {
                guard_facing = turn_right(&guard_facing);
            } else {
                // If we're not turning then check if we would enter an infinite loop if we *did* turn
                // Note that we need to check that we're not blocking a previously traversed tile
                let mut visited_in_any_direction = false;
                for i in 0..4 {
                    visited_in_any_direction |= visited[grid.convert_coordinates_to_index(new_location) as usize + (grid_size * i)];
                }
                if !visited_in_any_direction {
                    let right_direction = turn_right(&guard_facing);
                
                    let test_added_blockage_index = grid.convert_coordinates_to_index(new_location) as usize;
                    grid.blockages[test_added_blockage_index] = true;
                    if does_infinite_loop(&grid, visited.clone(), guard_location, right_direction) {
                        if !result.contains(&new_location) {
                            result.push(new_location);
                        }
                    }
                    grid.blockages[test_added_blockage_index] = false;
                }

                guard_location = new_location;
            }

            visited[grid.convert_coordinates_to_index(guard_location) as usize + (grid_size * guard_facing as usize)] = true;
        } else {
            // We've left the grid
            break;
        }
    }

    //println!("Visited: {:?}", visited);
    Ok(result.len())
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day6/example.txt")?, 41);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day6/example.txt")?, 7);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", part_1(&input_path).unwrap());
    // Part 2
    println!("Part two result: {}", part_2(&input_path).unwrap());
    Ok(())
}