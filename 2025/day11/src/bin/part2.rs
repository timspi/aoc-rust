use std::{collections::HashMap, time::Instant};

type Map = HashMap<String, (Vec<String>, HashMap<(bool,bool),u64>)>;

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
        machines.insert(id, (conns, HashMap::new()));
    }
    // dbg!(&machines);

    let count = check(&mut machines, &"svr".to_string(), false, false);
    count.to_string()
}

fn check(machines: &mut Map, id: &String, mut has_fft: bool, mut has_dac: bool) -> u64 {
    // println!("{id} {has_fft} {has_dac}");
    let (conns, count) = machines.get_mut(id).unwrap();
    if let Some(c) = count.get(&(has_fft, has_dac)) {
        return *c;
    }
    if id == "fft" {
        has_fft = true;
    } else if id == "dac" {
        has_dac = true;
    }

    let mut counter = 0;
    for conn in conns.clone() {
        if conn == "out" {
            if has_fft && has_dac {
                counter += 1;
            }
        } else {
            counter += check(machines, &conn, has_fft, has_dac);
        }
    }
    let m = machines.get_mut(id).unwrap();
    m.1.insert((has_fft, has_dac), counter);

    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run("svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out");
        assert_eq!(result, "2");
    }
}
