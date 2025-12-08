use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut id: u64 = 0;
    let mut is_file = true;

    let cap = input.len() / 2;
    let mut free_space = Vec::with_capacity(cap);
    let mut files = Vec::with_capacity(cap);
    let mut pos = 0;

    for b in input.bytes() {
        let size = b - b'0';
        if is_file {
            files.push((pos, size, id));
            id += 1;
        } else {
            free_space.push((pos, size));
        }
        pos += size as u64;
        is_file = !is_file;
    }

    for (pos, size, _file_id) in files.iter_mut().rev() {
        // Find first free space that fits the file
        if let Some((space_pos, space_size)) = free_space.iter_mut().find(|(_, s)| *size <= *s) {
            // Only move file forward!!!
            if space_pos >= pos {
                continue;
            }

            // println!("moving file #{file_id} to {space_pos}");
            // Update file position
            *pos = *space_pos;

            // Update free space
            *space_pos += *size as u64;
            *space_size -= *size;
        }
    }

    files
        .iter()
        .map(|(pos, size, id)| {
            (0..*size as u64)
                .map(|offset| (*pos + offset) * *id)
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_at_end() {
        let result = run("9953877292941");
        assert_eq!(result, "5768".to_string());
    }

    #[test]
    fn it_works() {
        let result = run("2333133121414131402");
        assert_eq!(result, "2858".to_string());
    }
}
