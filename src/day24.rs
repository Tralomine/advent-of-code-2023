#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
    Left,
    Up,
    Down,
    Right,
    _None,
}

#[derive(Debug, Copy, Clone)]
struct Blizz {
    pos: (usize, usize),
    dir: Dir
}

fn parse(s: &str) -> ((usize, usize), Vec<Blizz>) {
    let mut s = s.lines();
    let mut size = (0, 0);
    let first_line = s.next().unwrap();
    size.1 = first_line.as_bytes().len();
    let mut blizzs = Vec::new();
    for (x, l) in s.enumerate() {
        size.0 += 1;
        for (y, c) in l.as_bytes().iter().enumerate() {
            match *c as char {
                '>' => blizzs.push(Blizz{pos:(x+1, y), dir:Dir::Right}),
                '<' => blizzs.push(Blizz{pos:(x+1, y), dir:Dir::Left}),
                'v' => blizzs.push(Blizz{pos:(x+1, y), dir:Dir::Down}),
                '^' => blizzs.push(Blizz{pos:(x+1, y), dir:Dir::Up}),
                _ => (),
            }
        }
    }
    size.0 += 1;
    (size, blizzs)
}

#[derive(Debug, PartialEq)]
enum MapTile {
    Block,
    Pass(i64),
}

fn get_map(size: (usize, usize), blizzs: &Vec<Blizz>) -> Vec<Vec<MapTile>> {
    let mut map = Vec::new();
    for x in 0..size.0 {
        map.push(Vec::new());
        for y in 0..size.1 {
            if x == 0 || y == 0 || x == size.0-1 || y == size.1-1 {
                if x == 0 && y == 1 || x == size.0-1 && y == size.1-2 {
                    map[x].push(MapTile::Pass(i64::MAX));
                } else {
                    map[x].push(MapTile::Block);
                }
            } else if blizzs.iter().any(|b| b.pos == (x, y)) {
                map[x].push(MapTile::Block);
            } else {
                map[x].push(MapTile::Pass(i64::MAX));
            }
        }
    }
    map
}

fn move_blizzs(size: (usize, usize), blizzs: &mut Vec<Blizz>) {
    for b in blizzs {
        match b.dir {
            Dir::Down => b.pos.0 = if b.pos.0 == size.0-2 {1} else {b.pos.0 + 1},
            Dir::Up => b.pos.0 = if b.pos.0 == 1 {size.0-2} else {b.pos.0 - 1},
            Dir::Right => b.pos.1 = if b.pos.1 == size.1-2 {1} else {b.pos.1 + 1},
            Dir::Left => b.pos.1 = if b.pos.1 == 1 {size.1-2} else {b.pos.1 - 1},
            Dir::_None => (),
        }
    }
}

fn _print_blizzs(size: (usize, usize), blizzs: &Vec<Blizz>, map: &Vec<Vec<MapTile>>) {
    for x in 0..size.0 {
        for y in 0..size.1 {
            if let MapTile::Pass(k) = map[x][y] {
                if k < i64::MAX {
                    print!("{}", ansi_term::Colour::Red.paint(k.to_string()));
                } else {
                    print!(".");
                }
            } else {
                if x == 0 || y == 0 || x == size.0-1 || y == size.1-1 {
                    if x == 0 && y == 1 || x == size.0-1 && y == size.1-2 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                } else {
                    let mut nb_blizz = 0;
                    let mut blizz_dir = Dir::_None;
                    for b in blizzs {
                        if b.pos == (x, y) {
                            nb_blizz += 1;
                            blizz_dir = b.dir;
                        }
                    }
                    if nb_blizz > 1 {
                        print!("{}", ansi_term::Colour::Cyan.paint(nb_blizz.to_string()));
                    } else {
                        match blizz_dir {
                            Dir::Down => print!("{}", ansi_term::Colour::Blue.paint("v")),
                            Dir::Up => print!("{}", ansi_term::Colour::Blue.paint("^")),
                            Dir::Left => print!("{}", ansi_term::Colour::Blue.paint("<")),
                            Dir::Right => print!("{}", ansi_term::Colour::Blue.paint(">")),
                            Dir::_None => print!("."),
                        }
                    }
                }
            }
        }
        println!("");
    }
    println!("");
}

pub fn chall_1(s: &str) -> i64 {
    let (size, mut blizzs) = parse(s);

    let mut prev = get_map(size, &blizzs);
    prev[0][1] = MapTile::Pass(0);

    loop {
        // _print_blizzs(size, &blizzs, &maps.last().unwrap());
        move_blizzs(size, &mut blizzs);
        let mut new = get_map(size, &blizzs);
        if let MapTile::Pass(k) = prev[prev.len()-2][prev[0].len()-2] {
            if k < i64::MAX {break k+1;}
        }
        new[0][1] = MapTile::Pass(if let MapTile::Pass(k) = prev[0][1] {k+1} else {0});
        for x in 1..new.len()-1 {
            for y in 1..new[x].len()-1 {
                if new[x][y] == MapTile::Pass(i64::MAX) {
                    let mut min = i64::MAX;
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x-1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x+1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y-1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y+1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    new[x][y] = MapTile::Pass(min);
                }
            }
        }
        prev = new;
        // std::io::stdin().read_line(&mut String::new());
    }
}

pub fn chall_2(s: &str) -> i64 {
    let (size, mut blizzs) = parse(s);

    let mut prev = get_map(size, &blizzs);
    prev[0][1] = MapTile::Pass(0);

    let first = loop {
        // _print_blizzs(size, &blizzs, &prev);
        move_blizzs(size, &mut blizzs);
        let mut new = get_map(size, &blizzs);
        if let MapTile::Pass(k) = prev[size.0-2][prev[0].len()-2] {
            if k < i64::MAX {break k+1;}
        }
        new[0][1] = MapTile::Pass(if let MapTile::Pass(k) = prev[0][1] {k+1} else {0});
        for x in 1..size.0-1 {
            for y in 1..size.1-1 {
                if new[x][y] == MapTile::Pass(i64::MAX) {
                    let mut min = i64::MAX;
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x-1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x+1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y-1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y+1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    new[x][y] = MapTile::Pass(min);
                }
            }
        }
        prev = new;
        // std::io::stdin().read_line(&mut String::new());
    };
    let mut prev = get_map(size, &blizzs);
    prev[size.0-1][size.1-2] = MapTile::Pass(first);

    let second = loop {
        // _print_blizzs(size, &blizzs, &prev);
        move_blizzs(size, &mut blizzs);
        let mut new = get_map(size, &blizzs);
        if let MapTile::Pass(k) = prev[1][1] {
            if k < i64::MAX {break k+1;}
        }
        new[size.0-1][size.1-2] = MapTile::Pass(if let MapTile::Pass(k) = prev[size.0-1][size.1-2] {k+1} else {i64::MAX});
        for x in 1..new.len()-1 {
            for y in 1..new[x].len()-1 {
                if new[x][y] == MapTile::Pass(i64::MAX) {
                    let mut min = i64::MAX;
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x-1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x+1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y-1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y+1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    new[x][y] = MapTile::Pass(min);
                }
            }
        }
        prev = new;
        // std::io::stdin().read_line(&mut String::new());
    };
    let mut prev = get_map(size, &blizzs);
    prev[0][1] = MapTile::Pass(second);

    loop {
        // _print_blizzs(size, &blizzs, &prev);
        move_blizzs(size, &mut blizzs);
        let mut new = get_map(size, &blizzs);
        if let MapTile::Pass(k) = prev[prev.len()-2][prev[0].len()-2] {
            if k < i64::MAX {break k+1;}
        }
        new[0][1] = MapTile::Pass(if let MapTile::Pass(k) = prev[0][1] {k+1} else {0});
        for x in 1..new.len()-1 {
            for y in 1..new[x].len()-1 {
                if new[x][y] == MapTile::Pass(i64::MAX) {
                    let mut min = i64::MAX;
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x-1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x+1][y] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y-1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    min = std::cmp::min(if let MapTile::Pass(k) = prev[x][y+1] {if k == i64::MAX {k} else {k+1}} else {min}, min);
                    new[x][y] = MapTile::Pass(min);
                }
            }
        }
        prev = new;
        // std::io::stdin().read_line(&mut String::new());
    }
}
