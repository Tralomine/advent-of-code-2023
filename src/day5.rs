use std::ops::Range;

fn parse(s : &str) -> (usize, Vec<Vec<char>>) {
    let lines : Vec<&str> = s.lines().collect();
    let i = {
        let mut i = 0;
        for (j, l) in lines.iter().enumerate() {
            if l.as_bytes()[1] == '1' as u8 {
                i = j;
                break;
            }
        }
        i
    };
    let mut parsed : Vec<Vec<char>> = Vec::new();
    let box_nbr = {
        let l = lines[i];
        (l.as_bytes()[l.len()-2] - '0' as u8) as usize
    };
    for j in {Range{start:0, end: box_nbr}} {
        parsed.push(Vec::new());
    }
    for j in {Range{start: 0, end: i}}.rev() {
        for k in {Range{start: 0, end: box_nbr}} {
            let current_box = lines[j].as_bytes()[k*4+1] as char;
            if current_box == ' ' {continue;}
            parsed[k].push(current_box);
        }
    }
    (i+2, parsed)
}

pub fn chall_1(s : &String) -> String {
    let (start, mut boxes) = parse(s);

    let lines : Vec<&str> = s.lines().collect();

    for i in {Range{start, end: lines.len()}} {
        let (a, b, c) = {
            let line = lines[i].strip_prefix("move ").expect("invalid line");
            // dbg!(line);
            let s: Vec<&str> = line.split(" from ").collect();
            let v1 = s[0];
            let s: Vec<&str> = s[1].split(" to ").collect();
            (   v1.parse::<usize>().expect("invalid line"),
                s[0].parse::<usize>().expect("invalid line")-1,
                s[1].parse::<usize>().expect("invalid line")-1
            )
        };
        for j in {Range{start: 0, end: a}} {
            let moving_box = boxes[b].pop().expect("empty column");
            boxes[c].push(moving_box);
        }
    }

    let mut result = String::new();
    for mut b in boxes {
        result += &String::from(b.pop().expect("empty column"));
    }
    result
}

pub fn chall_2(s : &String) -> String {
    let (start, mut boxes) = parse(s);

    let lines : Vec<&str> = s.lines().collect();

    for i in {Range{start, end: lines.len()}} {
        let (a, b, c) = {
            let line = lines[i].strip_prefix("move ").expect("invalid line");
            let s: Vec<&str> = line.split(" from ").collect();
            let v1 = s[0];
            let s: Vec<&str> = s[1].split(" to ").collect();
            (   v1.parse::<usize>().expect("invalid line"),
                s[0].parse::<usize>().expect("invalid line")-1,
                s[1].parse::<usize>().expect("invalid line")-1
            )
        };
        let mut stack : Vec<char> = Vec::new();
        for j in {Range{start: 0, end: a}} {
            let moving_box = boxes[b].pop().expect("empty column");
            stack.push(moving_box);
        }
        loop {
            match stack.pop() {
                Some(b) => boxes[c].push(b),
                None => break,
            }
        }
    }

    let mut result = String::new();
    for mut b in boxes {
        result += &String::from(b.pop().expect("empty column"));
    }
    result
}