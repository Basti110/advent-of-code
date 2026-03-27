use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn check_if_order_is_valid(
    order_rules_before: &HashMap<String, HashSet<String>>,
    numbers: &Vec<&str>,
) -> bool {
    !numbers.iter().enumerate().any(|(j, &number)| {
        numbers.iter().skip(j).any(|&number_after| {
            order_rules_before
                .get(number)
                .map_or(false, |set| set.contains(number_after))
        })
    })
}

fn main() -> io::Result<()> {
    let file: File = File::open("./input/day5a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut order_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut seperator_line = 0;
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
    for i in 0..lines.len() {
        let line_str = lines[i].to_string();

        if line_str.is_empty() {
            seperator_line = i + 1;
            break;
        }

        let split_str: Vec<&str> = line_str.split("|").collect::<Vec<&str>>();
        let first = split_str[0].to_string();
        let second = split_str[1].to_string();
        order_map
            .entry(second)
            .or_insert_with(HashSet::new)
            .insert(first);
    }

    let mut valid_middle_count: i32 = 0;
    for i in seperator_line..lines.len() {
        // for line in reader.lines() {

        let line_str = lines[i].to_string();

        // let line_str = line.unwrap();
        let split_str: Vec<&str> = line_str.split(",").collect::<Vec<&str>>();

        let valid = check_if_order_is_valid(&order_map, &split_str);
        if valid {
            let middle_index: usize = (split_str.len() - 1) / 2;
            // print!("{:?}", split_str[middle_index]);
            valid_middle_count += split_str[middle_index].parse::<i32>().unwrap();
        }
    }
    // println!("{:?}", order_map);
    print!("{:}", valid_middle_count);
    Ok(())
}
