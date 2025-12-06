use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut lines = input.lines();
    let ops = lines.next_back().unwrap().split_ascii_whitespace();
    let lines = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|el| el.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let mut counter = 0;
    for (ind, op) in ops.enumerate() {
        counter += match op {
            "+" => lines.iter().map(|l| l[ind]).sum::<u64>(),
            "*" => lines.iter().map(|l| l[ind]).product::<u64>(),
            _ => unreachable!(),
        };
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ");
        assert_eq!(result, "4277556");
    }
}
