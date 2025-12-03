fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    input
        .lines()
        .map(|bank| get_bank_joltage(bank))
        .sum::<u64>()
        .to_string()
}

fn get_bank_joltage(bank: &str) -> u64 {
    let vals = bank.bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let len = vals.len();
    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..len {
        if i < len - 1 && vals[i] > max1 {
            max1 = vals[i];
            max2 = 0;
        } else if vals[i] > max2 {
            max2 = vals[i]
        }
    }
    let res = format!("{max1}{max2}");
    // println!("{bank}: {res}");
    res.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("987654321111111
811111111111119
234234234234278
818181911112111");
        assert_eq!(result, "357".to_string());
    }
}
