fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut dial = 50;
    let mut counter = 0;
    for line in input.lines() {
        let mut line = line.to_string();
        let val = line.split_off(1).parse::<i32>().unwrap();
        let last = dial;

        // Add full rotations directly to the counter and only rotate the remainder
        counter += val / 100;
        if line == "R" {
            dial += val % 100;
        }
        if line == "L" {
            dial -= val % 100;
        }

        if last != 0 && (dial <= 0 || dial >= 100) {
            counter += 1;
        }
        dial = (dial + 100) % 100;
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82");
        assert_eq!(result, "6".to_string());
    }
}
