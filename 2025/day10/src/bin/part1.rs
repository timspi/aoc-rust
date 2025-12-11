use std::{time::Instant, u32};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut sum = 0;
    for machine in input.lines() {
        let mut split = machine.split(' ');
        let res = split.next().unwrap();
        // let desired_state = desired_state[1..desired_state.len() - 1].replace('#', "1").replace('.', "0");
        // let desired_state = u32::from_str_radix(&desired_state, 2).unwrap();
        let mut desired_state = 0;
        for (i, b) in res[1..res.len() - 1].bytes().enumerate() {
            if b == b'#' {
                desired_state += 1 << i;
            }
        }

        let mut buttons = Vec::new();
        for part in split {
            if part.starts_with('(') {
                let button: Vec<_> = part[1..part.len() - 1].split(',').map(|b| b.parse::<usize>().unwrap()).collect();
                // let mut button = vec![false; len];
                // for ind in part[1..part.len() - 1].split(',').map(|b| b.parse::<usize>().unwrap()) {
                //     button[ind] = true;
                // }
                buttons.push(button);
            }
        }

        let mut min = u32::MAX;
        let len = buttons.len();

        // println!("desired state: {desired_state} ({desired_state:#b}), butons: {buttons:?}");
        for btn_mask in 0..2_usize.pow(len as u32 + 1) {
            let mut state = 0;
            let mut count = 0;
            
            // println!("checking: {btn_mask} ({btn_mask:#b})");
            for ind in 0..len {
                if (btn_mask >> ind) & 1 == 1 {
                    // Press button
                    for b in &buttons[ind] {
                        state ^= 1 << b;
                    }
                    count += 1;
                }
            }
            // println!("{state:?} {:?}", state == desired_state);
            if state == desired_state {
                // println!("found solution: {i:b}");
                if count < min {
                    min = count;
                }
            }
        }
        sum += min;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let result = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(result, "2");
    }

    #[test]
    fn ex2() {
        let result = run("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(result, "3");
    }
    #[test]
    fn ex3() {
        let result = run("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works() {
        let result = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, "7");
    }
}
