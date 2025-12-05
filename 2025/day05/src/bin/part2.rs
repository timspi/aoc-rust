use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
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

    ranges.sort_by_key(|(from, _)| *from);
    let mut counter = 0;

    for i in 0..ranges.len() {
        if ranges[i].1 >= ranges[i].0 {
            counter += ranges[i].1 - ranges[i].0 + 1;
        }
        if i + 1 < ranges.len() {
            // check if next range is overlapping
            if ranges[i + 1].0 <= ranges[i].1 {
                // adjust from of next range to start after the current already counted one
                ranges[i + 1].0 = ranges[i].1 + 1;
                // if the next range becomes invalid, use the to of the current to avoid double counting
                if ranges[i + 1].1 < ranges[i + 1].0 {
                    ranges[i + 1].1 = ranges[i].1;
                }
            }
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
        assert_eq!(result, "14".to_string());
    }
}
