#[derive(PartialEq, Eq)]
enum Cell {
    Number(i32),
    Part,
    Gear,
    Empty,
}

fn parse(s : &str) -> Vec<Vec<Cell>> {
    let mut cells = Vec::new();
    for l in s.lines() {
        cells.push(Vec::new());
        for c in l.chars() {
            if c == '.' {
                cells.last_mut().unwrap().push(Cell::Empty)
            } else if c == '*' {
                cells.last_mut().unwrap().push(Cell::Gear)
            } else if c.is_ascii_digit() {
                cells.last_mut().unwrap().push(Cell::Number(c.to_digit(10).unwrap() as i32))
            } else {
                cells.last_mut().unwrap().push(Cell::Part)
            }
        }
    }
    for x in 0..cells.len() {
        for y in 0..cells[x].len() {
            if let Cell::Number(v) = cells[x][y] {
                if y == 0 || (y > 0 && (if let Cell::Number(_) = cells[x][y-1] {false} else {true})) {
                    let mut label;
                    let mut label_len = 0;
                    label = v;
                    let mut y = y;
                    'up: loop {
                        y += 1;
                        if y == cells[x].len() {break;}
                        if let Cell::Number(v) = cells[x][y] {
                            label *= 10;
                            label += v;
                            label_len += 1;
                        } else {
                            break 'up;
                        }
                    }
                    for y in y-label_len-1..y {
                        cells[x][y] = Cell::Number(label);
                    }
                }
            }
        }
    }
    cells
}

pub fn chall_1(s : &String) -> i32 {
    let mut sum = 0;
    let cells = parse(s);
    for x in 0..cells.len() {
        for y in 0..cells[x].len() {
            if let Cell::Number(v) = cells[x][y] {
                if y == 0 || (y > 0 && (if let Cell::Number(_) = cells[x][y-1] {false} else {true})) {
                    let label = v;
                    let mut label_len = 1;
                    let mut y = y;
                    while y < cells[x].len()-1 {
                        y += 1;
                        label_len += 1;
                        if let Cell::Number(_) = cells[x][y] {
                        } else {
                            break;
                        }
                    }
                    let mut has_part = false;
                    if label_len > 4 {
                        dbg!(label, label_len, x, y);
                    }
                    'checkpart: for x1 in x as i32-1..=x as i32+1 {
                        for y1 in y as i32-label_len..=y as i32 {
                            if x1 >= 0 && x1 < cells.len() as i32 && y1 >= 0 && y1 < cells[x1 as usize].len() as i32 {
                                if cells[x1 as usize][y1 as usize] == Cell::Part || cells[x1 as usize][y1 as usize] == Cell::Gear {
                                    has_part = true;
                                    break 'checkpart;
                                }
                            }
                        }
                    }
                    if has_part {
                        sum += label;
                    }
                }
            }
        }
    }
    sum
}

pub fn chall_2(s : &String) -> i32 {
    let mut sum = 0;
    let cells = parse(s);
    for x in 0..cells.len() as i32 {
        for y in 0..cells[x as usize].len() as i32 {
            let mut neighbors = Vec::new();
            if cells[x as usize][y as usize] == Cell::Gear {
                for x1 in -1..=1 {
                    for y1 in -1..=1 {
                        if x+x1 >= 0 && x+x1 < cells.len() as i32 && y+y1 >= 0 && y+y1 < cells[(x+x1) as usize].len() as i32 {
                            if let Cell::Number(v) = cells[(x+x1) as usize][(y+y1) as usize] {
                                if neighbors.len() == 0 || neighbors[neighbors.len()-1] != v {
                                    neighbors.push(v);
                                }
                            }
                        }
                    }
                }
            }
            if neighbors.len() == 2 {
                sum += neighbors[0]*neighbors[1];
            }
        }
    }
    sum
}