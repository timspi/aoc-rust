use std::{time::Instant, u32};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut sum = 0;
    for machine in input.lines() {
        let mut parts: Vec<_> = machine.split(' ').collect();
        
        let joltages = parts.pop().unwrap();
        let joltages: Vec<_> = joltages[1..joltages.len() - 1].split(',').map(|s| s.parse::<u32>().unwrap()).collect();

        let mut buttons = Vec::new();
        let mut parts = parts.into_iter();
        let _lights = parts.next();
        for part in parts { // skip lights
            let button: Vec<_> = part[1..part.len() - 1].split(',').map(|b| b.parse::<usize>().unwrap()).collect();
            // let mut button = vec![false; len];
            // for ind in part[1..part.len() - 1].split(',').map(|b| b.parse::<usize>().unwrap()) {
            //     button[ind] = true;
            // }
            buttons.push(button);
        }

        // Get maximum # each button can be pressed
        let mut buttons: Vec<_> = buttons.into_iter().map(|btn| {
            let max = btn.iter().map(|ind| joltages[*ind]).min().unwrap();
            (btn, max)
        }).collect();
        
        buttons.sort_by_key(|(_, max)| *max);

        println!("joltages: {joltages:?}, btns: {buttons:?}");
        
        let btn_presses = vec![0; buttons.len()];
        let state = vec![0; joltages.len()];
        let res = check(btn_presses, state, 0, &joltages, &buttons);

        // println!("got {res:?}");

        sum += res.unwrap();
    }
    sum.to_string()
}

fn check(btn_presses: Vec<u32>, state: Vec<u32>, ind: usize, goal: &Vec<u32>, btns: &Vec<(Vec<usize>, u32)>) -> Option<u32> {
    // dbg!(&btn_presses, &state);
    if state == *goal {
        // reached the goal
        return Some(btn_presses.iter().sum());
    }
    for i in 0..state.len() {
        if state[i] > goal[i] {
            // overshot the goal, dead end
            return None;
        }
    }
    if ind >= btn_presses.len() {
        // no more options to press buttons
        return None;
    }

    let mut min = u32::MAX;
    for presses in 0..=btns[ind].1 {
        let mut btn_presses = btn_presses.clone();
        let mut state = state.clone();
        btn_presses[ind] = presses;
        btns[ind].0.iter().for_each(|i| state[*i] += presses);
        if let Some(res) = check(btn_presses, state, ind + 1, goal, btns) {
            if res < min {
                min = res;
            }
        }
    }
    if min < u32::MAX {
        Some(min)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let result = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(result, "10");
    }

    #[test]
    fn ex2() {
        let result = run("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(result, "12");
    }
    #[test]
    fn ex3() {
        let result = run("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, "11");
    }

    #[test]
    fn it_works() {
        let result = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, "33");
    }
}
