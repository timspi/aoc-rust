use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut disk = vec![];
    let mut id = 0;
    let mut is_file = true;
    for b in input.bytes() {
        let b = b - b'0';
        for _i in 0..b {
            if is_file {
                disk.push(id);
            } else {
                disk.push(-1);
            }
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    // dbg!(&disk);

    let mut i = 0;
    let mut sum = 0;
    while i < disk.len() {
        if disk[i] == -1 {
            disk[i] = disk.pop().unwrap();
        }
        if disk[i] != -1 {
            sum += i as i64 * disk[i];
            i += 1;
        }
    }
    // dbg!(&disk);

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("2333133121414131402");
        assert_eq!(result, "1928");
    }
}
