fn dec_from_snafu(s: &str) -> i64 {
    let mut v = Vec::new();
    for c in s.as_bytes() {
        v.push(match *c as char {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => i64::MAX,
        });
    }
    let mut n = 0;
    let mut k = 1;
    for i in v.iter().rev() {
        n += k*i;
        k *= 5;
    }
    n
}

fn snafu_from_dec(n: i64) -> String {
    let mut snafu = String::new();
    let mut n = n;
    while n > 0 {
        match n%5 {
            0 => {n = n/5; snafu.push('0');},
            1 => {n = n/5; snafu.push('1');},
            2 => {n = n/5; snafu.push('2');},
            3 => {n = n/5+1; snafu.push('=');},
            4 => {n = n/5+1; snafu.push('-');},
            _ => ()
        }
    }
    let mut s = String::new();
    for c in snafu.as_bytes().iter().rev() {
        s.push(*c as char);
    }
    s
}

pub fn chall_1(s: &str) -> String {
    let mut total = 0;
    for l in s.lines() {
        total += dec_from_snafu(l);
    }
    snafu_from_dec(total)
}
