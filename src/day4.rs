fn parse(s : &str) -> [i32;4] {
    let s: Vec<&str> = s.split(&[',', '-']).collect();
    let mut a = [0;4];
    for (i, k) in s.iter().enumerate() {
        a[i] = k.parse().expect("")
    }
    a
}

pub fn chall_1(s : &String) -> i32 {
    let mut total : i32 = 0;
    for l in s.lines() {
        let l = parse(l);
        if  l[0] >= l[2] && l[1] <= l[3] ||
            l[0] <= l[2] && l[1] >= l[3] {
            total += 1;
        }
    }
    total
}

pub fn chall_2(s : &String) -> i32 {
    let mut total : i32 = 0;
    for l in s.lines() {
        let l = parse(l);
        if  l[0] >= l[2] && l[0] <= l[3] ||
            l[1] >= l[2] && l[1] <= l[3] ||
            l[2] >= l[0] && l[2] <= l[1] {
            total += 1;
        }
    }
    total
}