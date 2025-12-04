use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = run(input);
    print!("{}\n", output);
}

fn run(input: &str) -> String {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'@' {
                map.insert((x as i32, y as i32), true);
            }
        }
    }
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut counter = 0;
    loop {
        let mut to_remove = Vec::new();
        for (x, y) in map.keys() {
            let mut adj_rolls = 0;
            for (dx, dy) in offsets {
                if map.contains_key(&(x + dx, y + dy)) {
                    adj_rolls += 1;
                    if adj_rolls >= 4 {
                        break;
                    }
                }
            }
            if adj_rolls < 4 {
                counter += 1;
                to_remove.push((*x, *y));
            }
        }
        if to_remove.len() == 0 {
            break;
        }
        for key in to_remove {
            map.remove(&key);
        }
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.");
        assert_eq!(result, "43");
    }
}
