use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let boxes = input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64, i64)>>();

    // Get all distance pairs
    let mut distances = Vec::new();
    for a in 0..boxes.len() {
        for b in a + 1..boxes.len() {
            let dist = get_dist(&boxes[a], &boxes[b]);
            distances.push(((a, b), dist));
        }
    }
    distances.sort_by_key(|(_, dist)| *dist);
    let mut distances = distances.into_iter();

    let mut circuits: Vec<Vec<usize>> = Vec::new();

    loop {
        let ((a, b), _) = distances.next().unwrap();

        // Build circuits
        let pos_a = circuits.iter().position(|v| v.contains(&a));
        let pos_b = circuits.iter().position(|v| v.contains(&b));
        if pos_a.is_some() && pos_b.is_some() {
            // Join circuits together, if not already the same
            if pos_a != pos_b {
                let mut circuit = circuits.swap_remove(pos_a.max(pos_b).unwrap()); // remove element at greater index so other index stays correct
                circuits[pos_a.min(pos_b).unwrap()].append(&mut circuit); // and append to element with lesser index
            }
        } else if pos_a.is_some() {
            circuits[pos_a.unwrap()].push(b);
        } else if pos_b.is_some() {
            circuits[pos_b.unwrap()].push(a);
        } else {
            circuits.push(vec![a, b]);
        }

        // We're done when all boxes are in one circuit
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            break boxes[a].0 * boxes[b].0;
        }
    }
    .to_string()
}

// No need to take square root as only interested in relative values
fn get_dist(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689");
        assert_eq!(result, "25272");
    }
}
