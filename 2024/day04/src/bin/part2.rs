fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let line_len = input.find("\n").unwrap();
    // println!("line_len={}", line_len);
    let chars = input.as_bytes();

    let mut sum = 0;

    for i in line_len + 2..chars.len() - line_len - 2 {
        if chars[i] == b'A' {
            if ((chars[i - line_len - 2] == b'M' && chars[i + line_len + 2] == b'S')
                || (chars[i - line_len - 2] == b'S' && chars[i + line_len + 2] == b'M'))
                && ((chars[i - line_len] == b'M' && chars[i + line_len] == b'S')
                    || (chars[i - line_len] == b'S' && chars[i + line_len] == b'M'))
            {
                // println!("X with center at index {}", i);
                sum += 1;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(result, "9".to_string());
    }
}
