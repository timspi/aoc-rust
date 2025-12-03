fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut counter = 0;
    for range in input.split(",") {
        let mut split = range.split("-");
        let from = split.next().unwrap().parse::<u64>().unwrap();
        let to = split.next().unwrap().parse::<u64>().unwrap();
        for num in from..=to {
            if !is_valid(num.to_string()) {
                counter += num;
            }
        }
    }
    counter.to_string()
}

fn is_valid(num: String) -> bool {
    for i in 1..=num.len() / 2 {
        let (seq, remainder) = num.split_at(i);
        if remainder.split(seq).all(|part| part.len() == 0) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_works() {
        assert_eq!(is_valid("11".to_string()), false);
        assert_eq!(is_valid("999".to_string()), false);
        assert_eq!(is_valid("38593859".to_string()), false);
        assert_eq!(is_valid("12345".to_string()), true);
        assert_eq!(is_valid("1698522".to_string()), true);
    }

    #[test]
    fn parts_work() {
        assert_eq!(run("11-22"), "33");
        assert_eq!(run("95-115"), "210");
        assert_eq!(run("998-1012"), "2009");
        assert_eq!(run("1188511880-1188511890"), "1188511885");
    }

    #[test]
    fn it_works() {
        let result = run("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        assert_eq!(result, "4174379265".to_string());
    }
}
