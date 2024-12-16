use std::collections::HashMap;

pub fn main() {
    let input = std::fs::read_to_string("input/day1-input.txt").unwrap();

    let mut lcolumn = Vec::new();
    let mut rcolumn_freq = HashMap::new();

    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lcolumn.push(numbers[0]);
        *rcolumn_freq.entry(numbers[1]).or_insert(0) += 1;
    }

    let sum: u32 = lcolumn
        .iter()
        .filter_map(|&num| rcolumn_freq.get(&num).map(|&count| num * count))
        .sum();

    println!("Day 2 part 2: {}", sum);
}
