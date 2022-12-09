fn parse_boxes(s : &str) -> (usize, Vec<Vec<char>>) {
    let lines: Vec<&str> = s.lines().collect();
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
    let mut parsed = Vec::new();
    let box_nbr = {
        let l = lines[i];
        (l.as_bytes()[l.len()-2] - '0' as u8) as usize
    };
    for _ in 0..box_nbr {
        parsed.push(Vec::new());
    }
    for j in (0..i).rev() {
        for k in 0..box_nbr {
            let current_box = lines[j].as_bytes()[k*4+1] as char;
            if current_box == ' ' {continue;}
            parsed[k].push(current_box);
        }
    }
    (i+2, parsed)
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let line = line.strip_prefix("move ").expect("invalid line");
    let line = line.replace(" from ", " to ");
    let v: Vec<&str> = line.split(" to ").collect();
    (   v[0].parse::<usize>().expect("invalid line"),
        v[1].parse::<usize>().expect("invalid line")-1,
        v[2].parse::<usize>().expect("invalid line")-1)
}

pub fn chall_1(s : &String) -> String {
    let (start, mut boxes) = parse_boxes(s);
    let lines: Vec<&str> = s.lines().collect();

    for i in start..lines.len() {
        let (a, b, c) = parse_line(lines[i]);
        for _ in 0..a {
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
    let (start, mut boxes) = parse_boxes(s);
    let lines: Vec<&str> = s.lines().collect();

    for i in start..lines.len() {
        let (a, b, c) = parse_line(lines[i]);
        let mut stack : Vec<char> = Vec::new();
        for _ in 0..a {
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