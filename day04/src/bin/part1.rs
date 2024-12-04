fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let line_len = input.find("\n").unwrap();
    // println!("line_len={}", line_len);
    let chars = input.as_bytes();

    let offsets = vec![1, line_len, line_len + 1, line_len + 2]; // be aware of \n at end of line

    let mut sum = 0;

    for i in 0..chars.len() {
        if chars[i] == b'X' {
            for offset in &offsets {
                if i + 3 * offset < chars.len()
                    && chars[i + 1 * offset] == b'M'
                    && chars[i + 2 * offset] == b'A'
                    && chars[i + 3 * offset] == b'S'
                {
                    // println!("XMAS at index {} with offset {}", i, offset);
                    sum += 1;
                }
            }
        }
        if chars[i] == b'S' {
            for offset in &offsets {
                if i + 3 * offset < chars.len()
                    && chars[i + 1 * offset] == b'A'
                    && chars[i + 2 * offset] == b'M'
                    && chars[i + 3 * offset] == b'X'
                {
                    // println!("SAMX at index {} with offset {}", i, offset);
                    sum += 1;
                }
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
        assert_eq!(result, "18".to_string());
    }
}
