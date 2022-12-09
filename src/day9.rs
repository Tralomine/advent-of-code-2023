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
        match l.as_bytes()[0] as char {
            'U' => {
                for _ in 0..n {
                    h.1 += 1;
                    t = follow(h, t);
                    if !prev_pos.contains(&t) {
                        prev_pos.push(t);
                    }
                }
            },
            'D' => {
                for _ in 0..n {
                    h.1 -= 1;
                    t = follow(h, t);
                    if !prev_pos.contains(&t) {
                        prev_pos.push(t);
                    }
                }
            },
            'R' =>  {
                for _ in 0..n {
                    h.0 += 1;
                    t = follow(h, t);
                    if !prev_pos.contains(&t) {
                        prev_pos.push(t);
                    }
                }
            },
            'L' =>  {
                for _ in 0..n {
                    h.0 -= 1;
                    t = follow(h, t);
                    if !prev_pos.contains(&t) {
                        prev_pos.push(t);
                    }
                }
            },
            _ => (),
        }
    }
    prev_pos.len()
}

pub fn chall_2(s : &String) -> usize {
    let mut rope = [(0, 0);10];
    let mut prev_pos = vec![];
    for l in s.lines() {
        let n = l.replace([' ', 'D', 'U', 'L', 'R'], "").parse::<i64>().unwrap();
        match l.as_bytes()[0] as char {
            'U' => {
                for _ in 0..n {
                    rope[0].1 += 1;
                    for i in 0..rope.len()-1 {
                        rope[i+1] = follow(rope[i], rope[i+1]);

                    }
                    if !prev_pos.contains(&rope[9]) {
                        prev_pos.push(rope[9]);
                    }
                }
            },
            'D' => {
                for _ in 0..n {
                    rope[0].1 -= 1;
                    for i in 0..rope.len()-1 {
                        rope[i+1] = follow(rope[i], rope[i+1]);

                    }
                    if !prev_pos.contains(&rope[9]) {
                        prev_pos.push(rope[9]);
                    }
                }
            },
            'R' =>  {
                for _ in 0..n {
                    rope[0].0 += 1;
                    for i in 0..rope.len()-1 {
                        rope[i+1] = follow(rope[i], rope[i+1]);

                    }
                    if !prev_pos.contains(&rope[9]) {
                        prev_pos.push(rope[9]);
                    }
                }
            },
            'L' =>  {
                for _ in 0..n {
                    rope[0].0 -= 1;
                    for i in 0..rope.len()-1 {
                        rope[i+1] = follow(rope[i], rope[i+1]);

                    }
                    if !prev_pos.contains(&rope[9]) {
                        prev_pos.push(rope[9]);
                    }
                }
            },
            _ => (),
        }
    }
    prev_pos.len()
}