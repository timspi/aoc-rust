use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut lines = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<Vec<u8>>>();
    let ops = lines.pop().unwrap();

    let mut counter = 0;
    let mut is_multiply = false;
    let mut calc = 0;
    for ind in 0..lines[0].len() {
        // When there's a new operator, add and reset last calc and update is_multiply
        if ops[ind] != b' ' {
            is_multiply = ops[ind] == b'*';
            counter += calc;
            calc = if is_multiply { 1 } else { 0 };
        }
        let mut val: u64 = 0;
        for line in &lines {
            if line[ind] >= b'0' && line[ind] <= b'9' {
                val = 10 * val + (line[ind] - b'0') as u64;
            }
        }
        if val == 0 {
            continue; // the empty column in between calculations, would not work if there are 0 values
        }
        if is_multiply {
            calc *= val;
        } else {
            calc += val;
        }
    }
    // Add last calc
    counter += calc;

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
        assert_eq!(result, "3263827");
    }
}
