use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn check_if_order_is_valid(
    order_rules_before: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
) -> bool {
    /*
    Checks that no number violates the the order rules
    */

    !numbers.iter().enumerate().any(|(j, &number)| {
        numbers.iter().skip(j).any(|&number_after| {
            order_rules_before
                .get(number)
                .map_or(false, |set| set.contains(number_after))
        })
    })
}

fn get_first_possible_index(
    order_rules_after: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
    number: &str,
) -> usize {
    let mut index: usize = 0;

    if order_rules_after.get(number).is_some() {
        let after_set: &HashSet<String> = order_rules_after.get(number).unwrap();

        for (i, &number_after) in numbers.iter().enumerate() {
            if after_set.contains(number_after) {
                index = std::cmp::max(index, i);
                break;
            }
        }
    }

    index
}

fn get_last_possible_index(
    order_rules_before: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
    number: &str,
) -> usize {
    let mut index: usize = 0;

    if order_rules_before.get(number).is_some() {
        let after_set: &HashSet<String> = order_rules_before.get(number).unwrap();

        for (i, &number_after) in numbers.iter().enumerate() {
            if after_set.contains(number_after) {
                index = i;
                break;
            }
        }
    }

    index
}

fn get_min_and_max_possible_indixes(
    order_rules_before: &HashMap<String, HashSet<String>>,
    order_rules_after: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
    number: &str,
) -> (usize, usize) {
    for (i, &number) in numbers.iter().enumerate() {
        let mut min_index: usize = 0;
        let mut max_index: usize = numbers.len() - 1;

        if order_rules_before.get(number).is_some() {
            let before_set: &HashSet<String> = order_rules_before.get(number).unwrap();
            let mut min_before_index: usize = 0;
            let mut max_before_index: usize = numbers.len() - 1;

            for (j, &number_before) in numbers.iter().enumerate() {
                if before_set.contains(number_before) {
                    min_before_index = j;
                    break;
                }
            }

            for (j, &number_before) in numbers.iter().enumerate().rev() {
                if before_set.contains(number_before) {
                    max_before_index = j;
                    break;
                }
            }

            min_index = std::cmp::max(min_index, min_before_index);
            max_index = std::cmp::min(max_index, max_before_index);
        }

        if order_rules_after.get(number).is_some() {
            let after_set: &HashSet<String> = order_rules_after.get(number).unwrap();
            let mut min_after_index: usize = 0;
            let mut max_after_index: usize = numbers.len() - 1;

            for (j, &number_after) in numbers.iter().enumerate() {
                if after_set.contains(number_after) {
                    min_after_index = j;
                    break;
                }
            }

            for (j, &number_after) in numbers.iter().enumerate().rev() {
                if after_set.contains(number_after) {
                    max_after_index = j;
                    break;
                }
            }

            min_index = std::cmp::max(min_index, min_after_index);
            max_index = std::cmp::min(max_index, max_after_index);
        }

        if min_index <= i && i <= max_index {
            return (min_index, max_index);
        }
    }

    (0, numbers.len() - 1)
}

fn order_numbers_correctly(
    order_rules_before: &HashMap<String, HashSet<String>>,
    order_rules_after: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
) -> Vec<&str> {
    let mut correct_order: Vec<&str> = Vec::new();
    for number in numbers {}
}

fn main() -> io::Result<()> {
    let file: File = File::open("./input/day5a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut order_map_before: HashMap<String, HashSet<String>> = HashMap::new();
    let mut order_map_after: HashMap<String, HashSet<String>> = HashMap::new();
    let mut seperator_line = 0;
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;

    // iterate through all x | y rules
    for i in 0..lines.len() {
        let line_str = lines[i].to_string();

        if line_str.is_empty() {
            seperator_line = i + 1;
            break;
        }

        let split_str: Vec<&str> = line_str.split("|").collect::<Vec<&str>>();
        let first = split_str[0].to_string();
        let second = split_str[1].to_string();
        order_map_before
            .entry(second)
            .or_insert_with(HashSet::new)
            .insert(first);
    }

    let mut valid_middle_count: i32 = 0;

    // iterate through all orders
    for i in seperator_line..lines.len() {
        let line_str = lines[i].to_string();

        let split_str: Vec<&str> = line_str.split(",").collect::<Vec<&str>>();

        let valid = check_if_order_is_valid(&order_map_before, &split_str);
        if !valid {
            let middle_index: usize = (split_str.len() - 1) / 2;
            valid_middle_count += split_str[middle_index].parse::<i32>().unwrap();
        } else {
        }
    }
    print!("{:}", valid_middle_count);
    Ok(())
}
