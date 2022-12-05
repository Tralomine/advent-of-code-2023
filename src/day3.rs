fn letter_to_nbr(c : u8) -> i32 {
    let c = c as i32;
    if c >= 0x61 && c <= 0x7a {
        c - 0x60
    } else if c >= 0x41 && c <= 0x5a {
        c - 0x40 + 26
    } else {
        c
    }
}

pub fn chall_1(s : &String) -> i32 {
    let mut total : i32 = 0;
    for l in s.lines() {
        let mut map = [false;56];
        let l = l.as_bytes();
        let len = l.len();
        for k in &l[..len/2] {
            map[letter_to_nbr(*k) as usize] = true;
        }
        for k in &l[len/2..] {
            if map[letter_to_nbr(*k) as usize] {
                total += letter_to_nbr(*k);
                break;
            }
        }
    }
    total
}

pub fn chall_2(s : &String) -> i32 {
    let mut total : i32 = 0;
    let mut lines = s.lines();
    loop {
        let mut map = [false;56];
        let l = match lines.next() {
            Some(l) => l.as_bytes(),
            None => break,    //no next line, so end of file
        };
        for k in l {
            map[letter_to_nbr(*k) as usize] = true;
        }
        let mut map2 = [false;56];
        let l = if let Some(line) = lines.next() {line.as_bytes()} else {break;};
        for k in l {
            if map[letter_to_nbr(*k) as usize] {
                map2[letter_to_nbr(*k) as usize] = true;
            }
        }
        let l = if let Some(line) = lines.next() {line.as_bytes()} else {break;};
        for k in l {
            if map2[letter_to_nbr(*k) as usize] {
                total += letter_to_nbr(*k);
                break;
            }
        }
    }
    total
}