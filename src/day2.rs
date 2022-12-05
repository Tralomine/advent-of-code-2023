pub fn chall_1(s : &String) -> i32 {
    let mut score = 0;
    for l in s.lines() {
        // dbg!("{}", l.as_bytes()[0] as char);
        let opponent = l.as_bytes()[0] as char;
        let you = l.as_bytes()[2] as char;
        match opponent {
            'A' => match you {
                'X' => score += 1 + 3,
                'Y' => score += 2 + 6,
                'Z' => score += 3 + 0,
                _ => (),
            },
            'B' => match you {
                'X' => score += 1 + 0,
                'Y' => score += 2 + 3,
                'Z' => score += 3 + 6,
                _ => (),
            },
            'C' => match you {
                'X' => score += 1 + 6,
                'Y' => score += 2 + 0,
                'Z' => score += 3 + 3,
                _ => (),
            },
            _ => (),
        }
    }
    score
}

pub fn chall_2(s : &String) -> i32 {
    let mut score = 0;
    for l in s.lines() {
        // dbg!("{}", l.as_bytes()[0] as char);
        let opponent = l.as_bytes()[0] as char;
        let you = l.as_bytes()[2] as char;
        match opponent {
            'A' => match you {
                'X' => score += 3 + 0,
                'Y' => score += 1 + 3,
                'Z' => score += 2 + 6,
                _ => (),
            },
            'B' => match you {
                'X' => score += 1 + 0,
                'Y' => score += 2 + 3,
                'Z' => score += 3 + 6,
                _ => (),
            },
            'C' => match you {
                'X' => score += 2 + 0,
                'Y' => score += 3 + 3,
                'Z' => score += 1 + 6,
                _ => (),
            },
            _ => (),
        }
    }
    score}