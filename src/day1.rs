pub fn chall_1(s : &String) -> i32 {
    let s = s.clone();
    let mut max = 0;
    let mut k = 0;
    for l in s.lines() {
        if l.trim().is_empty() {
            max = if k > max {k} else {max};
            k = 0;
        } else {
            k += l.trim().parse::<i32>().expect("");
        }
    }
    max = if k > max {k} else {max};
    max
}

pub fn chall_2(s : &String) -> i32 {
    let s = s.clone();
    let mut max = [0;3];
    let mut k = 0;
    for l in s.lines() {
        if l.trim().is_empty() {
            if k > max[1] {
                (max[2], max[1], max[0]) = (max[1], max[0], k);
            } else if k > max[1] {
                (max[2], max[1]) = (max[1], k);
            } else if k > max[2] {
                max[2] = k;
            }
            k = 0;
        } else {
            k += l.trim().parse::<i32>().expect("invalid line: not a number");
        }
    }
    max[0] + max[1] + max[2]
}