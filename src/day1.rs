pub fn main() {
    let i = std::fs::read_to_string("day1-input.txt").unwrap();
    let mut l = Vec::new();
    let mut r = Vec::new();

    for line in i.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        l.push(numbers[0]);
        r.push(numbers[1]);
    }
    l.sort();
    r.sort();

    let sum: u32 = l
        .iter()
        .zip(r.iter())
        .map(|(left, right)| right.abs_diff(*left))
        .sum();
    println!("Day 1 part 1: {}", sum);
}
