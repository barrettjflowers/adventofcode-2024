use itertools::Itertools;

pub fn main() {
    println!(
        "{}",
        include_bytes!("input/day2-input.txt") // reads file at compile time and returns its contents as a reference to an array of bytes
            .split(|&b| b == b'\n') // splits on newlines
            .filter(|line| {
                line.split(|&b| b == b' ') // splits on spaces
                    .map(|n| atoi::atoi::<isize>(n).unwrap()) // converts each number to an integer
                    .tuple_windows() // creates tuples of adjacent numbers
                    .try_fold(0, |ord, (a, b)| {
                        // compares adjacent numbers
                        if ord >= 0 && (1..=3).contains(&(b - a)) {
                            // check if order is correct and if the difference between b and a is between 1 and 3
                            Ok(1) // if it is, return 1
                        } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                            // check if order is correct and if the difference between a and b is between 1 and 3
                            Ok(-1) // if it is, return -1
                        } else {
                            Err(())
                        }
                    })
                    .is_ok()
            })
            .count(), // counts SAFE lines
    );
}
