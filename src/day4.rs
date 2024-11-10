fn parse(s: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let s: Vec<&str> = s.strip_prefix("Card").unwrap().trim().split(":").collect();
    let id = s[0].trim().parse().unwrap();
    let s: Vec<&str> = s[1].split("|").collect();
    let w: Vec<&str> = s[0].split_ascii_whitespace().collect();
    let c: Vec<&str> = s[1].split_ascii_whitespace().collect();
    let mut win = Vec::new();
    for n in w {
        win.push(n.parse().unwrap())
    }
    let mut cur = Vec::new();
    for n in c {
        cur.push(n.parse().unwrap())
    }
    (id, win, cur)
}

pub fn chall_1(s : &String) -> i32 {
    let mut sum = 0;
    for l in s.lines() {
        let (_, winning, current) = parse(l);
        let mut res = 0;
        for c in current {
            if winning.contains(&c) {
                res += 1;
            }
        }
        if res > 0 {
            sum += 2_i32.pow(res-1);
        }
    }
    sum
}

pub fn chall_2(s : &String) -> i32 {
    let mut cards = Vec::new();
    for l in s.lines() {
        cards.push(parse(l));
    }
    let mut cards_n = vec![1; cards.len()];
    for x in 0..cards.len() {
        let mut res = 0;
        for c in cards[x].2.clone() {
            if cards[x].1.contains(&c) {
                res += 1;
            }
        }
        for y in x+1..x+res+1 {
            cards_n[y] += cards_n[x];
        }
    }
    let mut sum = 0;
    for n in cards_n {
        sum += n;
    }
    sum
}