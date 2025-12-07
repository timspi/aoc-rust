use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let start = first.find('S').unwrap();
    let len = first.len();
    let mut counter = 0;

    let mut beams_x = vec![true; len];
    beams_x[start] = true;

    for line in lines {
        let bytes: Vec<u8> = line.bytes().collect();
        for x in 0..len {
            if beams_x[x] && bytes[x] == b'^' {
                counter += 1;
                if x > 0 {
                    beams_x[x - 1] = true;
                }
                if x + 1 < bytes.len() {
                    beams_x[x + 1] = true;
                }
                beams_x[x] = false;
            }
        }
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............");
        assert_eq!(result, "21");
    }
}
