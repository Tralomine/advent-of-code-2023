use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Elf {
    p: (i64, i64),
    will_move: bool,
    move_to: Option<(i64, i64)>,
}

impl std::cmp::PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Elf {
    fn new(p: (i64, i64)) -> Elf {
        Elf{p, will_move: false, move_to: None}
    }
}

fn parse(s: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.as_bytes().iter().enumerate() {
            if *c == '#' as u8 {
                elves.push(Elf::new((x as i64, y as i64)));
            }
        }
    }
    elves
}

fn print_elves(elves: &Vec<Elf>) {
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    for e in elves {
        min = (std::cmp::min(min.0, e.p.0), std::cmp::min(min.1, e.p.1));
        max = (std::cmp::max(max.0, e.p.0), std::cmp::max(max.1, e.p.1));
    }
    for y in min.1..max.1+1 {
        for x in min.0..max.0+1 {
            if elves.contains(&Elf::new((x, y))) {print!("█");}
            else {print!(" ")}
        }
        println!("");
    }
    println!("");
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn move_dir(e: &mut Elf, elves: &Vec<Elf>, dir: Dir) -> bool {
    if dir == Dir::North && 
      !elves.contains(&Elf::new((e.p.0-1, e.p.1-1))) &&
      !elves.contains(&Elf::new((e.p.0, e.p.1-1))) &&
      !elves.contains(&Elf::new((e.p.0+1, e.p.1-1))) {
        e.move_to = Some((e.p.0, e.p.1-1));
        return true;
    }
    if dir == Dir::South &&
      !elves.contains(&Elf::new((e.p.0-1, e.p.1+1))) &&
      !elves.contains(&Elf::new((e.p.0, e.p.1+1))) &&
      !elves.contains(&Elf::new((e.p.0+1, e.p.1+1))) {
        e.move_to = Some((e.p.0, e.p.1+1));
        return true;
    }
    if dir == Dir::West &&
      !elves.contains(&Elf::new((e.p.0-1, e.p.1-1))) &&
      !elves.contains(&Elf::new((e.p.0-1, e.p.1))) &&
      !elves.contains(&Elf::new((e.p.0-1, e.p.1+1))) {
        e.move_to = Some((e.p.0-1, e.p.1));
        return true;
    }
    if dir == Dir::East &&
      !elves.contains(&Elf::new((e.p.0+1, e.p.1-1))) &&
      !elves.contains(&Elf::new((e.p.0+1, e.p.1))) &&
      !elves.contains(&Elf::new((e.p.0+1, e.p.1+1))) {
        e.move_to = Some((e.p.0+1, e.p.1));
        return true;
    }
    false
}

pub fn chall_1(s: &str) -> i64 {
    let mut elves = parse(s);
    let mut dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    for _ in 0..10 {
        let mut new_elves = Vec::new();
        let mut propal_move = HashMap::new();
        for e in &elves {
            let mut new_elf = Elf::new(e.p);
            if elves.contains(&Elf::new((e.p.0-1, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0-1, e.p.1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0-1, e.p.1+1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0, e.p.1+1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1+1))) {new_elf.will_move = true;}

            if new_elf.will_move {
                if move_dir(&mut new_elf, &elves, dirs[0]) {}
                else if move_dir(&mut new_elf, &elves, dirs[1]) {}
                else if move_dir(&mut new_elf, &elves, dirs[2]) {}
                else if move_dir(&mut new_elf, &elves, dirs[3]) {}
                else {new_elf.will_move = false;}
            }
            if new_elf.will_move {
                let count = *propal_move.entry(new_elf.move_to.unwrap()).or_insert(0);
                propal_move.insert(new_elf.move_to.unwrap(), count+1);
            }
            new_elves.push(new_elf);
        }
        for e in &mut new_elves {
            if e.will_move {
                if propal_move[&e.move_to.unwrap()] == 1 {
                    e.p = e.move_to.unwrap();
                }
                e.will_move = false;
            }
        }
        let d = dirs.remove(0);
        dirs.push(d);
        elves = new_elves;
    }
    // print_elves(&elves);
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    for e in &elves {
        min = (std::cmp::min(min.0, e.p.0), std::cmp::min(min.1, e.p.1));
        max = (std::cmp::max(max.0, e.p.0), std::cmp::max(max.1, e.p.1));
    }
    let mut empty = (max.0-min.0+1) * (max.1-min.1+1);
    for y in min.1..max.1+1 {
        for x in min.0..max.0+1 {
            if elves.contains(&Elf::new((x, y))) {empty -= 1;}
        }
    }
    empty
}

pub fn chall_2(s: &str) -> i64 {
    let mut elves = parse(s);
    let mut dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    let mut count = 0;
    loop {
        let mut has_moved = false;
        let mut new_elves = Vec::new();
        let mut propal_move = HashMap::new();
        for e in &elves {
            let mut new_elf = Elf::new(e.p);
            if elves.contains(&Elf::new((e.p.0-1, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0-1, e.p.1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0-1, e.p.1+1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0, e.p.1+1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1-1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1))) {new_elf.will_move = true;}
            else if elves.contains(&Elf::new((e.p.0+1, e.p.1+1))) {new_elf.will_move = true;}

            if new_elf.will_move {
                if move_dir(&mut new_elf, &elves, dirs[0]) {}
                else if move_dir(&mut new_elf, &elves, dirs[1]) {}
                else if move_dir(&mut new_elf, &elves, dirs[2]) {}
                else if move_dir(&mut new_elf, &elves, dirs[3]) {}
                else {new_elf.will_move = false;}
            }
            if new_elf.will_move {
                let count = *propal_move.entry(new_elf.move_to.unwrap()).or_insert(0);
                propal_move.insert(new_elf.move_to.unwrap(), count+1);
            }
            new_elves.push(new_elf);
        }
        for e in &mut new_elves {
            if e.will_move {
                if propal_move[&e.move_to.unwrap()] == 1 {
                    e.p = e.move_to.unwrap();
                    has_moved = true;
                }
                e.will_move = false;
            }
        }
        let d = dirs.remove(0);
        dirs.push(d);
        elves = new_elves;
        count += 1;
        if !has_moved {break;}
    }
    print_elves(&elves);
    count
}
