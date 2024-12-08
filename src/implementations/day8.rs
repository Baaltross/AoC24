use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use crate::utility::generic_error::GenericResult;
use crate::utility::grid_2d::{self, Coordinates};

type AntennaMap = HashMap<char, Vec<grid_2d::Coordinates>>;
type AntinodeGrid = grid_2d::Grid<bool>;

fn load_antennas(input_path : &str) -> GenericResult<(AntennaMap, usize, usize)> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result: AntennaMap = HashMap::new();
    let mut width = 0;
    let mut height= 0;

    for (y_index, line) in reader.lines().enumerate() {
        let line = line?;
        height += 1;
        assert!(width == 0 || width == line.len());
        width = line.len();
        
        for (x_index, character) in line.chars().enumerate() {
            if character == '.' {
                continue;
            }

            result.entry(character).or_default().push(grid_2d::Coordinates{x: x_index as i64, y: y_index as i64});
        }
    }

    Ok((result, width, height))
}

fn generate_antinodes_part1(nodes_of_same_type: &Vec<Coordinates>, antinode_grid: &mut AntinodeGrid) {
    for i in 1..nodes_of_same_type.len() {
        for j in 0..i {
            let antinode_coords_1 = (nodes_of_same_type[i] * 2) - nodes_of_same_type[j];
            antinode_grid.set_value(antinode_coords_1, true);

            let antinode_coords_2 = (nodes_of_same_type[j] * 2) - nodes_of_same_type[i];
            antinode_grid.set_value(antinode_coords_2, true);
        }
    }
}

fn print_antinodes(antinode_grid: &AntinodeGrid) {
    for i in 0..antinode_grid.num_rows() {
        let mut row_str = String::new();
        for j in 0..antinode_grid.num_columns() {
            if let Some(value) = antinode_grid.get_value(Coordinates { x: j as i64, y: i as i64 }) {
                if *value {
                    row_str.push('#');
                } else {
                    row_str.push('.');
                }
            }
        }
        println!("{}", row_str);
    }
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let (antennas, width, height) = load_antennas(input_path)?;

    let mut antinode_grid: AntinodeGrid = AntinodeGrid::new(width, height, false);

    for antenna_type in antennas {
        generate_antinodes_part1(&antenna_type.1, &mut antinode_grid);
    }

    let mut result = 0;
    for antinode_value in antinode_grid.iter() {
        if *antinode_value {
            result += 1;
        }
    }

    Ok(result)
}

fn generate_antinodes_part2(nodes_of_same_type: &Vec<Coordinates>, antinode_grid: &mut AntinodeGrid) {
    for i in 1..nodes_of_same_type.len() {
        for j in 0..i {
            let diff = nodes_of_same_type[i] - nodes_of_same_type[j];

            let mut current_antinode_coords = nodes_of_same_type[i];
            while antinode_grid.set_value(current_antinode_coords, true) {
                current_antinode_coords += diff;
            }
            
            current_antinode_coords = nodes_of_same_type[j];
            while antinode_grid.set_value(current_antinode_coords, true) {
                current_antinode_coords -= diff;
            }
        }
    }
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let (antennas, width, height) = load_antennas(input_path)?;

    let mut antinode_grid: AntinodeGrid = AntinodeGrid::new(width, height, false);

    for antenna_type in antennas {
        generate_antinodes_part2(&antenna_type.1, &mut antinode_grid);
    }

    let mut result = 0;
    for antinode_value in antinode_grid.iter() {
        if *antinode_value {
            result += 1;
        }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day8/example.txt")?, 14);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day8/example.txt")?, 34);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", part_1(&input_path).unwrap());
    // Part 2
    println!("Part two result: {}", part_2(&input_path).unwrap());
    Ok(())
}