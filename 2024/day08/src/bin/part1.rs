use std::collections::{HashMap, HashSet};

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
                let delta_row = row2 as i32 - *row as i32;
                let delta_col = col2 as i32 - *col as i32;
                let antinode1 = (*row as i32 - delta_row, *col as i32 - delta_col);
                let antinode2 = (row2 as i32 + delta_row, col2 as i32 + delta_col);
                // dbg!(atype, (row,col), (row2,col2), antinode1, antinode2);
                for node in [antinode1, antinode2] {
                    if node.0 >= 0 && node.0 < row_max && node.1 >= 0 && node.1 < col_max {
                        antinodes.insert(node);
                    }
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
        assert_eq!(result, "14".to_string());
    }
}
