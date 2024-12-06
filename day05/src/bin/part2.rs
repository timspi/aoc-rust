use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let updates = parts.next().unwrap();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    rules.lines().for_each(|l| {
        let mut nums = l.split("|");
        let key = nums.next().unwrap().parse::<i32>().unwrap();
        let val = nums.next().unwrap().parse::<i32>().unwrap();
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap().push(val);
        } else {
            map.insert(key, vec![val]);
        }
    });

    let mut sum = 0;
    for line in updates.lines() {
        let line: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
        let mut is_valid = true;
        let mut seen = vec![];
        'line: for num in &line {
            if map.contains_key(&num) {
                for other in map.get(&num).unwrap() {
                    if seen.contains(&other) {
                        // This line is invalid!
                        is_valid = false;
                        break 'line;
                    }
                }
            }
            seen.push(num);
        }
        if !is_valid {
            // Reorder
            let mut ordered = line.clone();
            let mut was_mutated = true;
            while was_mutated {
                was_mutated = false;

                for i in 0..ordered.len() {
                    if map.contains_key(&ordered[i]) {
                        for other in map.get(&ordered[i]).unwrap() {
                            let ind = ordered.iter().position(|r| r == other);
                            if ind.is_some() && ind.unwrap() < i {
                                ordered.swap(i, ind.unwrap());
                                was_mutated = true;
                            }
                        }
                    }
                }
            }
            sum += ordered[(ordered.len() - 1) / 2];
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(result, "123".to_string());
    }
}
