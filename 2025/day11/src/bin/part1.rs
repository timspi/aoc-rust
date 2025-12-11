use std::{collections::HashMap, time::Instant};

type Map = HashMap<String, (Vec<String>, Option<u32>)>;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let mut machines: Map = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let id = split.next().unwrap().replace(':', "");
        let conns: Vec<_> = split.map(|s| s.to_string()).collect();
        machines.insert(id, (conns, None));
    }
    // dbg!(&machines);

    let count = check(&mut machines, &"you".to_string());
    count.to_string()
}

fn check(machines: &mut Map, id: &String) -> u32 {
    let (conns, count) = machines.get(id).unwrap();
    if let Some(c) = count {
        return *c;
    }
    let mut counter = 0;
    for conn in conns.clone() {
        if conn == "out" {
            counter += 1;
        } else {
            counter += check(machines, &conn);
        }
    }
    machines.get_mut(id).unwrap().1 = Some(counter);
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out");
        assert_eq!(result, "5");
    }
}
