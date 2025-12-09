use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let corners: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut max = 0;
    for i in 0..corners.len() {
        for j in i + 1..corners.len() {
            let area_sq = ((corners[i].0 - corners[j].0).abs() + 1)
                * ((corners[i].1 - corners[j].1) + 1).abs();
            if area_sq > max {
                max = area_sq;
            }
        }
    }

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3");
        assert_eq!(result, "50");
    }
}
