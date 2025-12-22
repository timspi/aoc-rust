use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<(i32, i32)>,
}

impl Shape {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let _ = lines.next().unwrap(); // id
        let mut points = Vec::new();
        for (y, line) in lines.enumerate() {
            for (x, p) in line.bytes().enumerate() {
                if p == b'#' {
                    points.push((x as i32, y as i32));
                }
            }
        }
        Shape {
            points,
        }
    }
}

fn run(input: &str) -> String {
    let mut blocks: Vec<&str> = input.split("\n\n").collect();
    let regions = blocks.pop().unwrap();

    let shapes: Vec<_> = blocks.into_iter().map(|b| Shape::parse(b)).collect();

    let mut sum = 0;
    for region in regions.lines() {
        // print!("{region} -> ");
        let mut parts = region.split_ascii_whitespace();
        let mut dim = parts
            .next()
            .unwrap()
            .split(['x', ':'])
            .map(|n| n.parse::<i32>().unwrap());
        let width = dim.next().unwrap();
        let height = dim.next().unwrap();

        let mut counts: Vec<_> = parts.map(|n| n.parse::<usize>().unwrap()).collect();
        // dbg!(&width, &height, &counts);
        if check(&shapes, &mut counts, width, height) {
            sum += 1;
        }
    }
    sum.to_string()
}

fn check(shapes: &Vec<Shape>, counts: &mut Vec<usize>, width: i32, height: i32) -> bool {
    // Check if there even is enough space if it fits perfectly
    let total_points: usize = counts
        .iter()
        .enumerate()
        .map(|(ind, c)| shapes[ind].points.len() * *c)
        .sum();
    let total_space = (width * height) as usize;
    if total_points > total_space {
        // println!("can never fit");
        return false;
    }

    // Check if there is enough space without overlap
    let parts = (width / 3) * (height / 3);
    if counts.iter().sum::<usize>() <= (parts as usize) {
        // println!("fits trivially");
        return true;
    }

    // println!("needs checking");
    unreachable!(); // are you kidding me??
}
