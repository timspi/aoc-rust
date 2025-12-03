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
    let mut maxs = vec![0; 12];
    for i in 0..len {
        for max_ind in 0..12 {
            // can only set new maximum for max_ind, if after max_ind (12 - max_ind - 1 elements) every element can still get a value (len - i remaining values)
            if 12 - max_ind - 1 < len - i && vals[i] > maxs[max_ind] {
                // new maximum and still enough spots left behind
                maxs[max_ind] = vals[i];
                maxs[max_ind + 1..].iter_mut().for_each(|m| *m = 0); // reset everything after
                break;
            }
        }
        // println!("after {i}: {maxs:?}");
    }
    let res = String::from_utf8(maxs.iter().map(|m| m + b'0').collect()).unwrap();
    // println!("{bank}: {res}");
    res.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_work() {
        assert_eq!(get_bank_joltage("987654321111111"), 987654321111);
        assert_eq!(get_bank_joltage("811111111111119"), 811111111119);
    }

    #[test]
    fn it_works() {
        let result = run("987654321111111
811111111111119
234234234234278
818181911112111");
        assert_eq!(result, "3121910778619".to_string());
    }
}
