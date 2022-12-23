#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    None,
    Wall,
    Walkable,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Left,
    Right,
    Forward(i64),
}

fn parse(s: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let map = parse_map(s);
    let instructions = parse_instructions(s.lines().last().unwrap());
    (map, instructions)
}

fn parse_map(s: &str) -> Vec<Vec<Tile>> {
    let mut map = Vec::new();
    for (i, l) in s.lines().enumerate() {
        map.push(Vec::new());
        if l.is_empty() {break;}
        for c in l.as_bytes() {
            if *c == ' ' as u8 {
                map[i].push(Tile::None);
            }
            if *c == '.' as u8 {
                map[i].push(Tile::Walkable);
            }
            if *c == '#' as u8 {
                map[i].push(Tile::Wall);
            }
        }
    }
    let mut max_len = 0;
    for l in &map {
        max_len = std::cmp::max(max_len, l.len());
    }
    //pad the map with wrapping tiles
    map.insert(0, Vec::new());
    map.push(Vec::new());
    for l in &mut map {
        while l.len() < max_len {
            l.push(Tile::None);
        }
        l.insert(0, Tile::None);
        l.push(Tile::None);
    }

    map
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut list = Vec::new();
    let mut n = 0;
    for c in s.as_bytes() {
        if *c >= '0' as u8 && *c <= '9' as u8 {
            n *= 10;
            n += (*c - '0' as u8) as i64;
        }
        if *c == 'R' as u8 {
            list.push(Instruction::Forward(n));
            list.push(Instruction::Right);
            n = 0;
        }
        if *c == 'L' as u8 {
            list.push(Instruction::Forward(n));
            list.push(Instruction::Left);
            n = 0;
        }
    }
    if n >= 0 {
        list.push(Instruction::Forward(n));
    }
    list
}

fn move_forward(map: &Vec<Vec<Tile>>, pos: (usize, usize), dir: i64, distance: i64) -> (usize, usize) {
    let mut pos = pos;
    'movef: for _ in 0..distance {
        if dir == 0 {
            pos = match map[pos.0][pos.1+1] {
                Tile::Walkable => (pos.0, pos.1+1),
                Tile::Wall => {break 'movef;},
                Tile::None => {
                    let mut x = 0;
                    loop {
                        if map[pos.0][x] == Tile::Wall {break 'movef;}
                        if map[pos.0][x] == Tile::Walkable {break (pos.0, x);}
                        x += 1;
                    }
                }
            }
        }
        if dir == 2 {
            pos = match map[pos.0][pos.1-1] {
                Tile::Walkable => (pos.0, pos.1-1),
                Tile::Wall => {break 'movef;},
                Tile::None => {
                    let mut x = map[pos.0].len()-1;
                    loop {
                        if map[pos.0][x] == Tile::Wall {break 'movef;}
                        if map[pos.0][x] == Tile::Walkable {break (pos.0, x);}
                        x -= 1;
                    }
                }
            }
        }
        if dir == 1 {
            pos = match map[pos.0+1][pos.1] {
                Tile::Walkable => (pos.0+1, pos.1),
                Tile::Wall => {break 'movef;},
                Tile::None => {
                    let mut y = 0;
                    loop {
                        if map[y][pos.1] == Tile::Wall {break 'movef;}
                        if map[y][pos.1] == Tile::Walkable {break (y, pos.1);}
                        y += 1;
                    }
                }
            }
        }
        if dir == 3 {
            pos = match map[pos.0-1][pos.1] {
                Tile::Walkable => (pos.0-1, pos.1),
                Tile::Wall => {break 'movef;},
                Tile::None => {
                    let mut y = map.len()-1;
                    loop {
                        if map[y][pos.1] == Tile::Wall {break 'movef;}
                        if map[y][pos.1] == Tile::Walkable {break (y, pos.1);}
                        y -= 1;
                    }
                }
            }
        }
    }
    pos
}

pub fn chall_1(s: &str) -> usize {
    let (map, instr) = parse(s);

    let mut pos = {
        let mut startx = 1;
        for x in 1..map[1].len() {
            if map[1][x] == Tile::Walkable {
                startx = x;
            }
        }
        (1, startx)
    };
    let mut dir: i64 = 0;    // 0 right, 1 down, 2 left 3 up

    for i in instr {
        match i {
            Instruction::Left => {
                dir -= 1;
                dir = dir.rem_euclid(4);
            },
            Instruction::Right => {
                dir += 1;
                dir %= 4;
            },
            Instruction::Forward(n) => pos = move_forward(&map, pos, dir, n),
        }
    }

    pos.0 * 1000 + pos.1 * 4 + dir as usize
}

fn move_cube(map: &Vec<Vec<Tile>>, pos: (usize, usize), dir: i64, distance: i64) -> ((usize, usize), i64) {
    (pos, dir)
}

pub fn chall_2(s: &str) -> usize {
    let (map, instr) = parse(s);

    let mut pos = {
        let mut startx = 1;
        for x in 1..map[1].len() {
            if map[1][x] == Tile::Walkable {
                startx = x;
            }
        }
        (1, startx)
    };
    let mut dir: i64 = 0;    // 0 right, 1 down, 2 left 3 up

    for i in instr {
        match i {
            Instruction::Left => {
                dir -= 1;
                dir = dir.rem_euclid(4);
            },
            Instruction::Right => {
                dir += 1;
                dir %= 4;
            },
            //TODO implement the algorithm to wrap around a cube
            Instruction::Forward(n) => (pos, dir) = move_cube(&map, pos, dir, n),
        }
    }

    pos.0 * 1000 + pos.1 * 4 + dir as usize
}