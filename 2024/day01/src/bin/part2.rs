use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut list1: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        list1.push(parts[0].parse().expect("Not a valid number"));

        let val2: i32 = parts[1].parse().expect("Not a valid number");
        if map.contains_key(&val2) {
            map.insert(val2, map.get(&val2).expect("exists") + 1);
        } else {
            map.insert(val2, 1);
        }
    }

    let mut sum = 0;
    for i in 0..list1.len() {
        let val2 = map.get(&list1[i]);
        if val2.is_some() {
            sum += list1[i] * val2.unwrap();
        }
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
        assert_eq!(result, "31".to_string());
    }
}
