use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut is_do = true;
    let mut sum = 0;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let revals = Regex::new(r"\d+").unwrap();

    let matches = re.captures_iter(input).map(|c| c.get(0).unwrap().as_str());

    for m in matches {
        if m.starts_with("don't") {
            is_do = false;
        } else if m.starts_with("do") {
            is_do = true;
        } else if is_do {
            let mut vals = revals
                .captures_iter(m)
                .map(|c| c.get(0).unwrap().as_str().parse::<i32>().unwrap());
            sum += vals.next().unwrap() * vals.next().unwrap();
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            run("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48".to_string());
    }
}
