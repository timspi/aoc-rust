use std::{collections::HashSet, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    // let mut map = HashMap::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut obstructions = HashSet::new();

    let lines: Vec<_> = input.lines().collect();

    let height = lines.len().try_into().unwrap();
    let width = lines.first().unwrap().len().try_into().unwrap();

    for (y, line) in lines.iter().enumerate() {
        for (x, field) in line.bytes().enumerate() {
            let coords = (x.try_into().unwrap(), y.try_into().unwrap());
            if field == b'#' {
                obstructions.insert(coords);
            } else if field == b'^' {
                pos = coords;
            }
        }
    }

    // dbg!(&obstructions);

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_index = 0;
    let mut visited = HashSet::new();
    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        // dbg!(pos);
        visited.insert(pos);

        let next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        if obstructions.contains(&next) {
            // Rotate right
            dir_index = (dir_index + 1) % 4;
        }

        pos.0 += dirs[dir_index].0;
        pos.1 += dirs[dir_index].1;
    }
    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
        assert_eq!(result, "41");
    }
}
