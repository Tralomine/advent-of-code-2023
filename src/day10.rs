pub fn chall_1(s: &String) -> i64 {
    let mut regx = 1;
    let mut cycle = 0;
    let mut sig_str = 0;
    for l in s.lines() {
        if l.starts_with("addx") {
            if (cycle-20)%40 == 39 || (cycle-20)%40 == -1 {
                sig_str += (cycle+1) * regx;
            }
            cycle += 2;
            if (cycle-20)%40 == 0 {
                sig_str += cycle * regx;
            }
            let value = l.strip_prefix("addx ")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            regx += value;
        } else if l.starts_with("noop") {
            cycle += 1;
            if (cycle-20)%40 == 0 {
                sig_str += cycle * regx;
            }
        }
    }
    sig_str
}

pub fn chall_2(s: &String) -> () {
    let mut regx = 1;
    let mut cycle = 0;
    for l in s.lines() {
        if cycle%40 == 0 { println!(); }
        if l.starts_with("addx") {
            if (regx-cycle%40 as i64).abs() <= 1 { print!("#"); }
            else { print!("."); }
            if (cycle+1)%40 == 0 { println!();}
            if (regx-(cycle+1)%40).abs() <= 1 { print!("#"); }
            else { print!("."); }
            cycle += 2;
            let value = l.strip_prefix("addx ").unwrap().parse::<i64>().unwrap();
            regx += value;
        } else if l.starts_with("noop") {
            if (regx-cycle%40).abs() <= 1 { print!("#"); }
            else { print!("."); }
            cycle += 1;
        }
    }
}
