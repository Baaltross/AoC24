use std::io::{BufRead, BufReader};

use crate::utility::generic_error::GenericResult;

#[derive(PartialEq, Eq)]
enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

fn create_dampened_vec(original_vec : &Vec<i32>, remove_index : usize) -> Vec<i32> {
    let mut result = Vec::new();
    result.extend(&original_vec[..remove_index]);
    result.extend(&original_vec[(remove_index+1)..]);
    result
}

fn is_stable(values : Vec<i32>, allow_dampening : bool) -> bool {
    let mut previous_value = values[0];
    let mut direction = Direction::Unknown;

    let mut result = true;

    let attempt_dampening = |allow_dampening : bool, values : &Vec<i32>, error_found_at : usize| -> bool {
        if !allow_dampening {
            return false;
        }

        // let mut naive_result = false;
        // for naive_index in 0..values.len() {
        //     if is_stable(create_dampened_vec(values, naive_index), false) {
        //         naive_result = true;
        //         break;
        //     }
        // }

        let mut result = is_stable(create_dampened_vec(values, error_found_at), false);

        // In some very early cases, removing the first or second index would create a stable report rather than the current index
        if error_found_at <= 2 {
            if is_stable(create_dampened_vec(values, 0), false) {
                result = true;
            }

            if error_found_at == 2 {
                if is_stable(create_dampened_vec(values, 1), false) {
                    result = true;
                }
            }
        }
        
        //assert_eq!(naive_result, result, "Mismatched with naive implementation on {:?}", values);
        result
    };

    for current_index in 1..values.len() {
        let current_value = values[current_index];
        let difference = current_value - previous_value;

        if difference == 0 || difference.abs() > 3 {
            //println!("Unstable: difference was {}", difference);
            result = attempt_dampening(allow_dampening, &values, current_index);
            break;
        }

        if difference > 0 {
            // Unknown is valid, so just check for equality with opposite direction
            if direction == Direction::Decreasing {
                //println!("Unstable: Increasing changed to decreasing");
                result = attempt_dampening(allow_dampening, &values, current_index);
                break;
            }
            direction = Direction::Increasing;
        } else {
            // Unknown is valid, so just check for equality with opposite direction
            if direction == Direction::Increasing {
                //println!("Unstable: Decreasing changed to increasing");
                result = attempt_dampening(allow_dampening, &values, current_index);
                break;
            }
            direction = Direction::Decreasing;
        }

        previous_value = current_value;
    }

    //println!("Values: {:?} - {}", values, result);
    result
}

fn count_safe_reports(input_path : &str, allow_dampening : bool) -> GenericResult<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result : usize = 0;

    for line in reader.lines() {
        let line = line?;

        let words = line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        let values: Vec<i32> = words.map(|s| s.parse().unwrap()).collect();

        if is_stable(values, allow_dampening) {
            result += 1;
        }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(count_safe_reports("data/day2/example.txt", false)?, 2);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(count_safe_reports("data/day2/example.txt", true)?, 9);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", count_safe_reports(&input_path, false).unwrap());
    // Part 2
    println!("Part two result: {}", count_safe_reports(&input_path, true).unwrap());
    Ok(())
}