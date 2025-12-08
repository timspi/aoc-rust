use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut start_pos: (i32, i32) = (0, 0);
    let mut obstructions = HashSet::new();

    let lines: Vec<_> = input.lines().collect();

    let height = lines.len() as i32;
    let width = lines.first().unwrap().len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, field) in line.bytes().enumerate() {
            let coords = (x as i32, y as i32);
            if field == b'#' {
                obstructions.insert(coords);
            } else if field == b'^' {
                start_pos = coords;
            }
        }
    }

    // Find possible obstacle positions

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut pos: (i32, i32) = start_pos.clone();
    let mut dir_index = 0;
    let mut possible_obst = HashMap::new();

    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        let next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        if obstructions.contains(&next) {
            // Rotate right
            dir_index = (dir_index + 1) % 4;
            continue;
        }
        // Store the current pos as the start for checking that loop
        // (only the first, as would be impossible to reach the second when obstacle was already added)
        possible_obst.entry(next).or_insert((pos, dir_index));

        pos = next;
    }

    // Test all possible locations
    let sum: u32 = possible_obst
        .par_iter()
        .map(|(obstacle, (start, start_dir))| {
            // Cannot place obstacle on top or in front of guard
            if *obstacle == start_pos || *obstacle == (pos.0, pos.1 - 1) {
                return 0;
            }

            let mut test_obst = obstructions.clone();
            test_obst.insert(*obstacle);
            if test_loop(&test_obst, start, start_dir, width, height) {
                1
            } else {
                0
            }
        })
        .sum();

    sum.to_string()
}

fn test_loop(
    obstructions: &HashSet<(i32, i32)>,
    start: &(i32, i32),
    start_dir: &usize,
    width: i32,
    height: i32,
) -> bool {
    let mut pos = start.clone();
    let mut dir_index = start_dir.clone();

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited = HashSet::new();
    loop {
        // Rotate until next position is free
        let next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        if obstructions.contains(&next) {
            // Rotate right
            dir_index = (dir_index + 1) % 4;
            continue; // might cause endless loop if completely blocked in
        }

        pos = next;

        if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
            // Stop when leaving map -> no loop
            return false;
        }

        let state = (pos.0, pos.1, dir_index as u8);
        if !visited.insert(state) {
            // Found a loop
            return true;
        }
    }
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
        assert_eq!(result, "6".to_string());
    }
}
