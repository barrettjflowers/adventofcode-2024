pub fn main() {
    let input = std::fs::read_to_string("day1-input.txt").unwrap();
    let mut lcolumn = Vec::new();
    let mut rcolumn = Vec::new();

    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        lcolumn.push(numbers[0]);
        rcolumn.push(numbers[1]);
    }

    lcolumn.sort();
    rcolumn.sort();

    let sum: u32 = lcolumn
        .iter()
        .zip(rcolumn.iter())
        .map(|(left, right)| right.abs_diff(*left))
        .sum();

    println!("Day 1: {}", sum);
}
