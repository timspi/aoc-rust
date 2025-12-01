use std::collections::{HashMap, HashSet};
use num::integer::gcd;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let col_max = input.find('\n').unwrap() as i32;
    let row_max = input.len() as i32 / col_max;
    // dbg!(col_max, row_max);

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char)
                    .and_modify(|v| v.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }
    }

    // dbg!(&antennas);

    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for (_, antennas) in antennas {
        for (ind, (row, col)) in antennas.iter().enumerate() {
            for ind2 in ind+1..antennas.len() {
                // For each pair of antennas of this type
                let (row2, col2) = antennas[ind2];
                let delta_row: i32 = (row2 as i32) - (*row as i32);
                let delta_col: i32 = (col2 as i32) - (*col as i32);
                let div = gcd(delta_row, delta_col);
                let delta_row = delta_row / div;
                let delta_col = delta_col / div;

                let mut r = *row as i32;
                let mut c = *col as i32;
                loop {
                    r -= delta_row;
                    c -= delta_col;
                    if r < 0 || c < 0 || r >= row_max || c >= col_max {
                        break;
                    }
                    antinodes.insert((r, c));
                }

                let mut r = *row as i32;
                let mut c = *col as i32;
                loop {
                    if r < 0 || c < 0 || r >= row_max || c >= col_max {
                        break;
                    }
                    antinodes.insert((r, c));
                    r += delta_row;
                    c += delta_col;
                }
            }
        }
    }
    // dbg!(&antinodes);
    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
        assert_eq!(result, "34".to_string());
    }
}
