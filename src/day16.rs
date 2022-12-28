use std::{collections::HashMap};
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

fn release_pressure(start: &str, valves: &mut HashMap<String, Node>, time_left: i64) -> i64 {
    if time_left < 0 {return 0;}

    let mut pressure_released = 0;
    for (k, d) in valves[start].tunnels.clone().iter() {
        if valves[k].is_open {continue;}
        let vk = valves.get(k).unwrap().clone();
        valves.get_mut(k).unwrap().is_open = true;
        pressure_released = std::cmp::max(
            pressure_released,
            (time_left-d-1)*vk.rate + release_pressure(k,valves, time_left-d-1)
        );
        valves.get_mut(k).unwrap().is_open = false;
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
    // dbg!(&reduced);

    release_pressure("AA", &mut reduced, 30)
}

fn release_pressure2(starta: &str, startb: &str, timea: i64, timeb: i64, valves: &mut HashMap<String, Node>, time_left: i64) -> i64 {
    if time_left < 0 {return 0;}

    let mut pressure_released = 0;
    let mut tunnels_a = Vec::new();
    for (k, d) in valves[starta].tunnels.iter() {
        if !valves[k].is_open {
            tunnels_a.push((k.clone(), *d));
        }
    }
    let mut tunnels_b = Vec::new();
    for (k, d) in valves[startb].tunnels.iter() {
        if !valves[k].is_open {
            tunnels_b.push((k.clone(), *d));
        }
    }

    if timea <= 0 && timeb <= 0 {
        for (_i, (k1, d1)) in tunnels_a.iter().enumerate() {
            for (_j, (k2, d2)) in tunnels_b.iter().enumerate() {
                if k1 == k2 {continue;}
                // if time_left > 20 {
                //     for _ in 0..26-time_left {
                //         print!(" ");
                //     }
                //     println!("{_i}: {k1}, {_j}: {k2}");
                // }

                let rate1 = valves.get(k1).unwrap().rate;
                let rate2 = valves.get(k2).unwrap().rate;
                valves.get_mut(k1).unwrap().is_open = true;
                valves.get_mut(k2).unwrap().is_open = true;
                pressure_released = std::cmp::max(
                    pressure_released,
                    (time_left-d1-1)*rate1 + (time_left-d2-1)*rate2 + release_pressure2(k1, k2, *d1, *d2, valves, time_left-1)
                );
                valves.get_mut(k1).unwrap().is_open = false;
                valves.get_mut(k2).unwrap().is_open = false;
            }
        }
    } else if timea <= 0 {
        for (_i, (k, d)) in tunnels_a.iter().enumerate() {
            // if time_left > 20 {
            //     for _ in 0..26-time_left {
            //         print!(" ");
            //     }
            //     println!("{_i}: {k} you ({timeb})");
            // }

            let rate = valves.get(k).unwrap().rate;
            valves.get_mut(k).unwrap().is_open = true;
            pressure_released = std::cmp::max(
                pressure_released,
                (time_left-d-1)*rate + release_pressure2(k, startb, *d, timeb-1, valves, time_left-1)
            );
            pressure_released = std::cmp::max(
                pressure_released,
                (time_left-d-1)*rate + release_pressure(startb, valves, time_left-timeb-1)
            );
            valves.get_mut(k).unwrap().is_open = false;
        }
    } else if timeb <= 0 {
        for (_i, (k, d)) in tunnels_b.iter().enumerate() {
            // if time_left > 20 {
            //     for _ in 0..26-time_left {
            //         print!(" ");
            //     }
            //     println!("{_i}: {k} eleph ({timea})");
            // }

            let rate = valves.get(k).unwrap().rate;
            valves.get_mut(k).unwrap().is_open = true;
            pressure_released = std::cmp::max(
                pressure_released,
                (time_left-d-1)*rate + release_pressure2(starta, k, timea-1, *d, valves, time_left-1)
            );
            pressure_released = std::cmp::max(
                pressure_released,
                (time_left-d-1)*rate + release_pressure(starta, valves, time_left-timea-1)
            );
            valves.get_mut(k).unwrap().is_open = false;
        }

    } else {
        if timea > timeb {
            pressure_released = release_pressure2(starta, startb, timea-timeb, 0, valves, time_left-timeb);
        } else {
            pressure_released = release_pressure2(starta, startb, 0, timeb-timea, valves, time_left-timea);
        }
    }

    pressure_released
}


pub fn chall_2(s: &str) -> i64 {
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
                if k1 != k2 && (valves[k2].rate > 0) {
                    tunnels.push((k2.clone(), get_distance(k1, k2, &valves, valves.len(), &mut Vec::new())));
                }
            }
            let mut v = v.clone();
            v.tunnels = tunnels;
            reduced.insert(k1.clone(), v);
        }
    }

    if reduced["AA"].rate == 0 {
        reduced.get_mut("AA").unwrap().is_open = true;
    }

    // println!("{:?}", &reduced);

    release_pressure2("AA", "AA", 0, 0, &mut reduced, 26)
}