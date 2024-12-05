use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use crate::utility::generic_error::GenericResult;

#[derive(Default, Debug, Clone)]
struct OrderingRuleEntry {
    after_this : Vec<usize>,
}

type OrderingRules = HashMap<usize, OrderingRuleEntry>;
type UpdateList = Vec<Vec<usize>>;

fn load_ordering_rules(lines : &mut std::io::Lines<BufReader<std::fs::File>>) -> GenericResult<OrderingRules> {
    let mut result : OrderingRules = HashMap::new();

    for line in lines {
        let line = line?;
        if line.len() == 0 {
            // Reached the end of this section
            break;
        }

        let parsed = sscanf::sscanf!(line.trim(), "{}|{}", usize, usize)?;
        let entry = result.entry(parsed.0).or_insert(Default::default());
        entry.after_this.push(parsed.1);
    }

    Ok(result)
}

fn load_update_list(lines : &mut std::io::Lines<BufReader<std::fs::File>>) -> GenericResult<UpdateList> {
    let mut result : UpdateList = Vec::new();

    for line in lines {
        let line = line?;
        let split_line : Vec<usize> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();
        result.push(split_line);
    }
    Ok(result)
}

fn load_document(input_path : &str) -> GenericResult<(OrderingRules, UpdateList)> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut lines = reader.lines();

    let ordering_rules = load_ordering_rules(&mut lines)?;
    let update_list = load_update_list(&mut lines)?;


    Ok((ordering_rules, update_list))
}

fn is_valid_update(ordering_rules: &OrderingRules, update: &Vec<usize>) -> bool {
    if update.len() <= 1 {
        return true;
    }

    for (entry_index, entry_value) in update.iter().enumerate() {
        for test_entry in &update[0..entry_index] {
            if let Some(entry_rules) = ordering_rules.get(entry_value) {
                if entry_rules.after_this.contains(test_entry) {
                    return false;
                }
            }
        }
    }

    true
}

fn part_1(input_path : &str) -> GenericResult<usize> {
    let mut result = 0;
    let (ordering_rules, update_list) = load_document(input_path)?;

    for update in update_list {
        assert_eq!(update.len() % 2, 1);
        if is_valid_update(&ordering_rules, &update) {
            result += update[update.len() / 2];
        }
    }

    Ok(result)
}

fn part_2(input_path : &str) -> GenericResult<usize> {
    let mut result = 0;
    let (ordering_rules, update_list) = load_document(input_path)?;

    for mut update in update_list {
        assert_eq!(update.len() % 2, 1);
        if !is_valid_update(&ordering_rules, &update) {
            // Apparently the ordering graph in not acyclic across all entries,
            // and is only guaranteed to be resolvable for particular given updates
            // So fuck it, we're doing a slow sort
            for _ in 0..update.len() {
                for i in 1..update.len() {
                    if let Some(i_ordering) = ordering_rules.get(&update[i]) {
                        if i_ordering.after_this.contains(&update[i-1]) {
                            update.swap(i-1, i);
                        }
                    }
                }
            }
            result += update[update.len() / 2];
        }
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("data/day5/example.txt")?, 143);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("data/day5/example.txt")?, 123);
    Ok(())
}

pub fn run(input_path : &String) -> GenericResult<()> {
    // Part 1
    println!("Part one result: {}", part_1(&input_path).unwrap());
    // Part 2
    println!("Part two result: {}", part_2(&input_path).unwrap());
    Ok(())
}