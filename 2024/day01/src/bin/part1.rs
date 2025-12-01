fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        list1.push(parts[0].parse().expect("Not a valid number"));
        list2.push(parts[1].parse().expect("Not a valid number"));
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("3   4
        4   3
        2   5
        1   3
        3   9
        3   3");
        assert_eq!(result, "11".to_string());
    }
}
