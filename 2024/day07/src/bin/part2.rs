fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut count = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let test_val = parts.next().unwrap().parse::<i64>().unwrap();
        let terms: Vec<i64> = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        if test(test_val, 0, &terms[..], Op::Add) {
            count += test_val;
        }
    }
    count.to_string()
}

enum Op {
    Add,
    Mul,
    Cat,
}

fn test(val: i64, acc: i64, terms: &[i64], op: Op) -> bool {
    let res = match op {
        Op::Add => acc + terms[0],
        Op::Mul => acc * terms[0],
        Op::Cat => acc * 10_i64.pow(terms[0].ilog10() + 1) + terms[0],
    };
    if terms.len() > 1 {
        if test(val, res, &terms[1..], Op::Add) {
            return true;
        }
        if test(val, res, &terms[1..], Op::Mul) {
            return true;
        }
        if test(val, res, &terms[1..], Op::Cat) {
            return true;
        }
        return false;
    }
    val == res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20");
        assert_eq!(result, "11387".to_string());
    }
}
