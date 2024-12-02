fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut count = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if is_save(&parts, true) || is_save(&parts, false) {
            count += 1;
        }
    }
    return count.to_string();
}

fn is_save(parts: &Vec<&str>, is_inc: bool) -> bool {
    let mut last: i32 = parts[0].parse().expect("should be a number");
    // let inc = parts.last().expect("exists").parse::<i32>().expect("should be a number") - last;
    // if inc == 0 {
    //     return false;
    // }
    // let is_inc = inc > 0;
    let mut used_dampening = false;
    // dbg!(&parts, &is_inc);
    for i in 1..parts.len() {
        let num: i32 = parts[i].parse().expect("should be a number");
        if !check(num, last, is_inc) {
            if used_dampening {
                return false;
            }
            // If only last entry is invalid, we're good
            if i == parts.len() - 1 {
                return true;
            }
            // Try skipping num
            let next: i32 = parts[i + 1].parse().expect("should be a number");
            if check(next, last, is_inc) {
                used_dampening = true;
                continue;
            }
            // Try skipping first (currently in variable last)
            if i == 1 {
                if check(next, num, is_inc) {
                    used_dampening = true;
                    last = num;
                    continue;
                }
            }
            return false;
        }
        last = num;
    }
    true
}

fn check(num1: i32, num2: i32, is_inc: bool) -> bool {
    let diff: i32 = num1 - num2;
    // dbg!(&diff);
    if diff == 0 || (diff > 0) != is_inc {
        return false;
    }
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, "4".to_string());
    }
}
