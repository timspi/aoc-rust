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

    let mut beams_x: Vec<u64> = vec![0; len];
    beams_x[start] = 1;

    for line in lines {
        let bytes: Vec<u8> = line.bytes().collect();
        for x in 0..len {
            if beams_x[x] > 0 && bytes[x] == b'^' {
                if x > 0 {
                    beams_x[x - 1] += beams_x[x];
                }
                if x + 1 < bytes.len() {
                    beams_x[x + 1] += beams_x[x];
                }
                beams_x[x] = 0;
            }
        }
    }
    beams_x.iter().sum::<u64>().to_string()
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
        assert_eq!(result, "40");
    }
}
