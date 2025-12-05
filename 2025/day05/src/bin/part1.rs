use std::{cmp::Ordering, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());

    let now = Instant::now();
    println!(
        "alt: {}   ({} us)",
        run_alt(input),
        now.elapsed().as_micros()
    );
}

fn run(input: &str) -> String {
    let mut ranges = Vec::new();
    let mut parts = input.split("\n\n");

    // Ranges
    for line in parts.next().unwrap().lines() {
        let mut split = line.split('-');
        let from = split.next().unwrap().parse::<u64>().unwrap();
        let to = split.next().unwrap().parse::<u64>().unwrap();
        ranges.push((from, to));
    }

    let mut counter = 0;

    // Ingredients
    for line in parts.next().unwrap().lines() {
        let id = line.parse::<u64>().unwrap();
        if ranges.iter().any(|(from, to)| id >= *from && id <= *to) {
            counter += 1;
        }
    }
    counter.to_string()
}

fn run_alt(input: &str) -> String {
    let mut ranges = Vec::new();
    let mut parts = input.split("\n\n");

    // Ranges
    for line in parts.next().unwrap().lines() {
        let mut split = line.split('-');
        let from = split.next().unwrap().parse::<u64>().unwrap();
        let to = split.next().unwrap().parse::<u64>().unwrap();
        ranges.push((from, to));
    }

    ranges.sort_by_key(|(from, _)| *from);
    let mut counter = 0;

    // Remove overlaps to enable binary search
    for i in 0..ranges.len() - 1 {
        // check if next range is overlapping
        if ranges[i + 1].0 <= ranges[i].1 {
            // adjust from of next range to start after the current already counted one
            ranges[i + 1].0 = ranges[i].1 + 1;
        }
    }

    // Ingredients
    for line in parts.next().unwrap().lines() {
        let id = line.parse::<u64>().unwrap();
        if ranges
            .binary_search_by(|(from, to)| {
                if *from > id {
                    Ordering::Greater
                } else if *to < id {
                    Ordering::Less // this only is true when there are no overlaps
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
        {
            counter += 1;
        }
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("3-5
10-14
16-20
12-18

1
5
8
11
17
32");
        assert_eq!(result, "3".to_string());
    }
}
