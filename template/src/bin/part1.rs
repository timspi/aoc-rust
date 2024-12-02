fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    dbg!(output);
}

fn run(input: &str) -> String {
    "TODO".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("");
        assert_eq!(result, "".to_string());
    }
}
