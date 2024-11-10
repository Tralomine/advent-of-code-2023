use std::cmp;


fn parse_line(s: &str) -> ((i64, i64), (i64, i64)) {
    let s = s.strip_prefix("Sensor at x=").unwrap();
    let mut s = s.split(": closest beacon is at x=");
    let mut s1 = s.next().unwrap().split(", y=");
    let mut s2 = s.next().unwrap().split(", y=");
    
    let x1 = s1.next().unwrap().parse::<i64>().unwrap();
    let y1 = s1.next().unwrap().parse::<i64>().unwrap();
    let x2 = s2.next().unwrap().parse::<i64>().unwrap();
    let y2 = s2.next().unwrap().parse::<i64>().unwrap();

    ((x1, y1), (x2, y2))
}

fn manhatan_distance(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0-p2.0).abs() + (p1.1-p2.1).abs()
}

pub fn chall_1(s: &str) -> i64 {
    const LINE: i64 = 10;

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    let (mut min, mut max) = ((i64::MAX, i64::MAX), (0, 0));
    let mut max_distance = 0;

    for l in s.lines() {
        let (sensor, beacon) = parse_line(l);
        let distance = manhatan_distance(sensor, beacon);

        max_distance = cmp::max(max_distance, distance);
        min.0 = cmp::min(min.0, sensor.0);
        min.1 = cmp::min(min.1, sensor.1);
        max.0 = cmp::max(max.0, sensor.0);
        max.1 = cmp::max(max.1, sensor.1);

        sensors.push((sensor, distance));
        
        if !beacons.contains(&beacon) {
            beacons.push(beacon);
        }
    }

    let mut imposs_pos = 0;

    'all_x: for x in min.0 - max_distance..max.0 + max_distance {
        let y = LINE;
        if beacons.contains(&(x, y)) {
            // imposs_pos += 1;
            continue 'all_x;
        }
        for s in &sensors {
            if manhatan_distance(s.0, (x, y)) <= s.1 {
                imposs_pos += 1;
                continue 'all_x;
            }
        }
    }

    imposs_pos
}

pub fn chall_2(s: &str) -> i64 {
    const MIN: i64 = 0;
    const MAX: i64 = 4_000_000; // size of the square

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    let (mut min, mut max) = ((i64::MAX, i64::MAX), (0, 0));
    let mut max_distance = 0;

    for l in s.lines() {
        let (sensor, beacon) = parse_line(l);
        let distance = manhatan_distance(sensor, beacon);

        max_distance = cmp::max(max_distance, distance);
        min.0 = cmp::min(min.0, sensor.0);
        min.1 = cmp::min(min.1, sensor.1);
        max.0 = cmp::max(max.0, sensor.0);
        max.1 = cmp::max(max.1, sensor.1);
        // min and max distance between a sensor and its closest beacon

        sensors.push((sensor, distance));
        
        if !beacons.contains(&beacon) {
            beacons.push(beacon);
        }
    }

    // for y in 0..4_000_000 {
    //     'all_x: for x in 0..4_000_000 {
    //         for s in &sensors {
    //             if manhatan_distance(s.0, (x, y)) <= s.1 {
    //                 continue 'all_x;
    //             }
    //         }
    //         return x*4_000_000+y;
    //     }
    // }    //ahah waaaaaayyy too slow

    for x in MIN..=MAX {
        // for each lines, check which sensor touch that line and where it covers it
        let mut min_max = Vec::new();
        for s in &sensors {
            let dist_x = s.1 - (x - s.0.0).abs(); //distance x
            let min = (s.0.1 as i64)-dist_x;
            let max = (s.0.1 as i64)+dist_x;
            if dist_x > 0 {
                min_max.push((min, max));
            }   //get all the ranges close enough to a sensor
        }

        min_max.sort_by(|a, b| a.0.cmp(&b.0));
        min_max.reverse();

        //  check if there's a hole in the union of the ranges
        let (_min, mut max) = min_max.pop().unwrap();
        while min_max.len() > 0 {
            if max > MAX {break;}
            let cur = min_max.pop().unwrap();
            if cur.0 <= max+1 && cur.1 >= max+1 {
                max = cur.1;
            } else if cur.0 > max+1 {
                return x*4_000_000 + max+1;
            }
        }
    }
    0
}
