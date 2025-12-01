fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    // let mut map = HashMap::new();
    let mut start_pos: (i32, i32) = (0, 0);
    let mut obstructions = Vec::new();

    let lines: Vec<_> = input.lines().collect();

    let height = lines.len().try_into().unwrap();
    let width = lines.first().unwrap().len().try_into().unwrap();

    for (y, line) in lines.iter().enumerate() {
        for (x, field) in line.bytes().enumerate() {
            let coords = (x.try_into().unwrap(), y.try_into().unwrap());
            if field == b'#' {
                obstructions.push(coords);
            } else if field == b'^' {
                start_pos = coords;
            }
        }
    }

    // dbg!(&obstructions);

    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut pos: (i32, i32) = start_pos.clone();
    let mut dir_index = 0;
    let mut visited = Vec::new();
    let mut possible_obst = Vec::new();
    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        // dbg!(pos);
        if !visited.contains(&pos) {
            visited.push(pos);
        }

        let next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        if !possible_obst.contains(&next) {
            possible_obst.push(next);
        }
        if obstructions.contains(&next) {
            // Rotate right
            dir_index = (dir_index + 1) % 4;
        }

        pos.0 += dirs[dir_index].0;
        pos.1 += dirs[dir_index].1;
    }
    // dbg!(&possible_obst);

    // Test all possible locations (only sensible on the actual path of the guard, skip first two as on and in front of guard)
    let mut sum = 0;
    for obst_pos in &possible_obst {
        // if obstructions.contains(&(x, y)) {
        //     // obstruction already there
        //     continue;
        // }
        if obst_pos.0 == start_pos.0 && obst_pos.1 == start_pos.1 {
            // cannot place on top of guard
            continue;
        }

        let mut test_obst = obstructions.clone();
        test_obst.push(*obst_pos);
        if test_loop(start_pos, &test_obst, width, height) {
            // dbg!(obst_pos);
            sum += 1;
        }
    }
    // let mut test_obst = obstructions.clone();
    // test_obst.push((3, 6));
    // if test_loop(start_pos, &test_obst, width, height) {
    //     sum += 1;
    // }

    // dbg!(&obstructions);
    sum.to_string()
}

fn test_loop(start: (i32, i32), obstructions: &Vec<(i32, i32)>, width: i32, height: i32) -> bool {
    let mut pos = start.clone();
    let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_index = 0;
    let mut visited = Vec::new();
    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        // dbg!(pos);
        let state = (pos.0, pos.1, dirs[dir_index].0, dirs[dir_index].1);
        if visited.contains(&state) {
            // Found a loop
            // dbg!("loop", &visited);
            return true;
        } else {
            visited.push(state);
        }

        let mut next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        while obstructions.contains(&next) {
            // Rotate right
            dir_index = (dir_index + 1) % 4;
            next = (pos.0 + dirs[dir_index].0, pos.1 + dirs[dir_index].1);
        }

        pos.0 += dirs[dir_index].0;
        pos.1 += dirs[dir_index].1;
    }
    // Left the map -> no loop
    // dbg!("no loop", &visited);
    return false;
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
