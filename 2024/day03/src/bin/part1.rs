use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches = re.captures_iter(input).map(|c| c.get(0).unwrap().as_str());

    let revals = Regex::new(r"\d+").unwrap();

    let mut sum = 0;
    for m in matches {
        let mut vals = revals
            .captures_iter(m)
            .map(|c| c.get(0).unwrap().as_str().parse::<i32>().unwrap());
        sum += vals.next().unwrap() * vals.next().unwrap();
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, "161".to_string());
    }
}
