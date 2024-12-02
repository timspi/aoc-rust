fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut count = 0;
    for line in input.lines() {
        if is_save(line) {
            count += 1;
        }
    }
    return count.to_string();
}

fn is_save(line: &str) -> bool {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let mut last: i32 = parts[0].parse().expect("should be a number");
    let inc = parts[1].parse::<i32>().expect("should be a number") - last;
    if inc == 0 {
        return false;
    }
    let is_inc = inc > 0;
    // dbg!(&parts, &is_inc);
    for i in 1..parts.len() {
        let num: i32 = parts[i].parse().expect("should be a number");
        let diff: i32 = num - last;
        // dbg!(&diff);
        if diff == 0 || (diff > 0) != is_inc {
            return false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        last = num;
    }
    true
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
        assert_eq!(result, "2".to_string());
    }


    #[test]
    fn tc1() {
        let result = run("43 40 41 42 43 46 47 44");
        assert_eq!(result, "0".to_string());
    }
    
}
