use core::time;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug)]
struct Node {
    valve: String,
    rate: i64,
    is_open: bool,
    tunnels: Vec<(String, i64)>,
}

fn parse_line(s: &str) -> Node {
    let mut tunnels = Vec::new();

    let s = s.strip_prefix("Valve ").unwrap();
    let valve = String::from(&s[..2]);
    let s = &s[17..];
    let re = Regex::new(r"; tunnels? leads? to valves? ").unwrap();
    let s = re.replace_all(s, "_");
    let mut s = s.split("_");
    let rate = s.next().unwrap().parse::<i64>().unwrap();
    let s = s.next().unwrap().split(", ");

    for n in s {
        tunnels.push((String::from(n), 1));
    }

    Node{is_open: false, valve, rate, tunnels: tunnels.clone()}
}

// fn open_best(valves: &mut HashMap<String, (bool, i64, Vec<String>)>, current: &str, time_left:i64) -> i64 {
//     if time_left == 0 {return 0;}
//     let mut best = 0;
//     let tunnels = (valves[current].2).clone();
//     for n in tunnels {
//         let i = open_best(valves, &n, time_left-1);
//         best = std::cmp::max(best, i);
//     }

//     let mut total = best;
//     for (_, v) in valves.iter() {
//         if v.0 {
//             total += v.1;
//         }
//     }
//     total
// }

fn get_distance(a: &str, b: &str, map: &HashMap<String, Node>, dist_max: usize, previous: &mut Vec<String>) -> i64 {
    if a == b {return 0;}
    if dist_max == 0 {return i64::MAX;}
    let mut min = i64::MAX;
    for (k, _) in &map[a].tunnels {
        if !previous.contains(&k) {
            previous.push(String::from(a));
            let mut dist = get_distance(&k, b, map, dist_max-1, previous);
            if dist < i64::MAX {dist += 1;}
            min = std::cmp::min(min, dist);
            previous.pop();
        }
    }
    min
}

fn release_pressure(start: &str, valves: HashMap<String, Node>, time_left: i64) -> i64 {
    if time_left < 0 {return 0;}
    // let mut distances = HashMap::new();
    // for k in valves.keys() {
    //     if !valves[k].is_open && valves[k].rate > 0 {
    //         distances.insert(k.clone(), get_distance(start, k, &valves, valves.len(), &mut Vec::new()));
    //     }
    // }

    let mut pressure_released = 0;
    for (i, (k, d)) in valves[start].tunnels.iter().enumerate() {
        if valves[k].is_open {continue;}
        let vk = valves.get(k).unwrap();
        let mut new_valves = valves.clone();
        new_valves.get_mut(k).unwrap().is_open = true;
        pressure_released = std::cmp::max(
            pressure_released,
            (time_left-d-1)*vk.rate + release_pressure(k, new_valves, time_left-d-1)
        );
    }

    pressure_released
}

pub fn chall_1(s: &str) -> i64 {
    let mut valves = HashMap::new();
    let mut reduced = HashMap::new();
    for l in s.lines() {
        let node = parse_line(l);
        valves.insert(node.valve.clone(), node.clone());
    }
 
 
 

    for (k1, v) in valves.iter() {
        if v.rate != 0 || v.valve == "AA" {
            let mut tunnels = Vec::new();
            for k2 in valves.keys() {
                if k1 != k2 && (valves[k2].rate > 0 || valves[k2].valve == "AA") {
                    tunnels.push((k2.clone(), get_distance(k1, k2, &valves, valves.len(), &mut Vec::new())));
                }
            }
            let mut v = v.clone();
            v.tunnels = tunnels;
            reduced.insert(k1.clone(), v);
            
        }
    }
    dbg!(&reduced);

    release_pressure("AA", reduced, 30)
}

pub fn chall_2(s: &str) -> i64 {
    0
}