fn parse(s : &str) -> (i32, Vec<(i32, i32, i32)>){
    let k: Vec<&str> = s.split(": ").collect();
    let n: i32 = k[0].strip_prefix("Game ").unwrap().parse().unwrap();
    let k: Vec<&str> = k[1].split("; ").collect();

    let mut colors : Vec<(i32, i32, i32)> = Vec::new();
    for set in k {
        let mut c = (0, 0, 0);
        for o in set.split(", ").collect::<Vec<&str>>() {
            let o: Vec<&str> = o.split(" ").collect();
            let num: i32 = o[0].parse().unwrap();
            if o[1] == "red" { c.0 = num;}
            if o[1] == "green" { c.1 = num;}
            if o[1] == "blue" { c.2 = num;}
        }
        colors.push(c);
    }

    (n, colors)
}

pub fn chall_1(s : &String) -> i32 {
    let mut ret = 0;
    for l in s.lines() {
        let x = parse(l);
        let mut is_possible = true;
        for g in x.1 {
            if  g.0 > 12 || g.1 > 13 || g.2 > 14 {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            ret += x.0;
        }
    }
    ret
}

pub fn chall_2(s : &String) -> i32 {
    let mut ret = 0;
    for l in s.lines() {
        let x = parse(l);
        let mut mins = (0, 0, 0);
        for g in x.1 {
            mins = (i32::max(mins.0, g.0), i32::max(mins.1, g.1), i32::max(mins.2, g.2));
        }
        ret += mins.0 * mins.1 * mins.2;
    }
    ret
}
