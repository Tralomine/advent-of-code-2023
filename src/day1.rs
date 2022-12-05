pub fn chall_1(s : &String) -> i32 {
    let s = s.clone();
    let mut max = 0;
    let mut k = 0;
    for l in s.lines() {
        // dbg!("{:?}", l);
        if l.trim().is_empty() {
            max = if k > max {k} else {max};
            k = 0;
        } else {
            k += l.trim().parse::<i32>().expect("");
        }
    }
    max
}

pub fn chall_2(s : &String) -> i32 {
    let s = s.clone();
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    let mut k = 0;
    for l in s.lines() {
        if l.trim().is_empty() {
            if k > max1 {
                (max3, max2, max1) = (max2, max1, k);
            } else if k > max2 {
                (max3, max2) = (max2, k);
            } else if k > max3 {
                max3 = k;
            }
            k = 0;
        } else {
            k += l.trim().parse::<i32>().expect("");
        }
    }
    // dbg!("{} {} {}", max1, max2, max3);
    max1 + max2 + max3
}