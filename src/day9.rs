fn follow(h: (i64, i64), t:(i64, i64)) -> (i64, i64) {
    let mut t = t;
    if (h.0 - t.0).abs() > 1 || (h.1 - t.1).abs() > 1 {
        if t.0 > h.0 {t.0 -= 1}
        if t.0 < h.0 {t.0 += 1}
        if t.1 > h.1 {t.1 -= 1}
        if t.1 < h.1 {t.1 += 1}
    }
    t
}


pub fn chall_1(s : &String) -> usize {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut prev_pos = vec![];
    for l in s.lines() {
        let n = l.replace([' ', 'D', 'U', 'L', 'R'], "").parse::<i64>().unwrap();
        for _ in 0..n {
            match l.as_bytes()[0] as char {
                'U' => h.1 += 1,
                'D' => h.1 -= 1,
                'R' => h.0 += 1,
                'L' => h.0 -= 1,
                _ => (),
            }
            t = follow(h, t);
            if !prev_pos.contains(&t) {
                prev_pos.push(t);
            }
        }
    }
    prev_pos.len()
}

pub fn chall_2(s : &String) -> usize {
    let mut rope = [(0, 0);10];
    let mut prev_pos = vec![];
    for l in s.lines() {
        let n = l.replace([' ', 'D', 'U', 'L', 'R'], "").parse::<i64>().unwrap();
        for _ in 0..n {
            match l.as_bytes()[0] as char {
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                'R' => rope[0].0 += 1,
                'L' => rope[0].0 -= 1,
                _ => (),
            }
            for i in 0..rope.len()-1 {
                rope[i+1] = follow(rope[i], rope[i+1]);
            }
            if !prev_pos.contains(&rope[9]) {
                prev_pos.push(rope[9]);
            }
        }
    }
    prev_pos.len()
}