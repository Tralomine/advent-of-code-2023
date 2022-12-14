use std::cmp;

#[derive(PartialEq, Eq)]
enum Cave {
    Air,
    Sand,
    Rock,
}

fn get_dim(s: &str) -> ((usize, usize), (usize, usize)) {
    let mut min = (usize::MAX, 0);
    let mut max = (0, 0);

    for l in s.lines() {
        for p in l.split(" -> ") {
            let mut pair = p.split(',');
            let x = pair.next().unwrap().parse::<usize>().unwrap();
            let y = pair.next().unwrap().parse::<usize>().unwrap();
            if min.0 > x {min.0 = x;}
            if min.1 > y {min.1 = y;}
            if max.0 < x {max.0 = x;}
            if max.1 < y {max.1 = y;}
        }
    }
    (min, max)
}

fn parse(s: &str, min: (usize, usize), max: (usize, usize)) -> Vec<Vec<Cave>> {
    let mut grid = Vec::new();
    
    for x in min.0..=max.0 {
        grid.push(Vec::new());
        for _ in min.1..=max.1 {
            grid[x-min.0].push(Cave::Air);
        }
    }

    for l in s.lines() {
        let mut end_pos = (0, 0);
        for p in l.split(" -> ") {
            let mut pair = p.split(',');
            let start_pos = end_pos;
            end_pos = {
                let x = pair.next().unwrap().parse::<usize>().unwrap();
                let y = pair.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            };
            if start_pos != (0, 0) {
                if start_pos.0 != end_pos.0 {
                    for x in cmp::min(start_pos.0, end_pos.0)..=cmp::max(start_pos.0, end_pos.0) {                        grid[x - min.0][start_pos.1 - min.1] = Cave::Rock;
                    }
                }
                if start_pos.1 != end_pos.1 {
                    for y in cmp::min(start_pos.1, end_pos.1)..=cmp::max(start_pos.1, end_pos.1) {
                        grid[start_pos.0 - min.0][y - min.1] = Cave::Rock;
                    }
                }
            }
        }
    }
    grid
}

pub fn chall_1(s: &str) -> i64 {
    let (min, max) = get_dim(s);
    let mut grid = parse(s, min, max);
    let sand_start = (500, 0);
    let mut time = 0;
    'infinity: loop {
        let mut sand_pos = sand_start;
        loop {
            if sand_pos.1 + 1 > max.1 {
                break 'infinity;
            }
            if grid[sand_pos.0 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.1 += 1;
            } else if sand_pos.0-1 < min.0 {
                break 'infinity;
            } else if grid[sand_pos.0-1 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if sand_pos.0+1 > max.0 {
                break 'infinity;
            } else if grid[sand_pos.0+1 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                grid[sand_pos.0 - min.0][sand_pos.1 - min.1] = Cave::Sand;
                time += 1;
                break;
            }
        }
    }
    time
}

pub fn chall_2(s: &str) -> i64 {
    let (min, max) = get_dim(s);
    let mut sn = String::new();
    sn += &(min.0 - max.1 - 1).to_string();
    sn += ",";
    sn += &(max.1 + 2).to_string();
    sn += " -> ";
    sn += &(max.0 + max.1 + 1).to_string();
    sn += ",";
    sn += &(max.1 + 2).to_string();
    sn += "\n";

    let (min, max) = get_dim(&sn);
    let mut grid = parse(&sn, min, max);

    let sand_start = (500, 0);
    let mut time = 0;
    'infinity: loop {
        let mut sand_pos = sand_start;
        loop {
            if grid[sand_pos.0 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.1 += 1;
            } else if grid[sand_pos.0-1 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if grid[sand_pos.0+1 - min.0][sand_pos.1+1 - min.1] == Cave::Air {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                if grid[sand_pos.0 - min.0][sand_pos.1 - min.1] == Cave::Sand {
                    break 'infinity;
                }
                grid[sand_pos.0 - min.0][sand_pos.1 - min.1] = Cave::Sand;
                time += 1;
                break;
            }
        }
    }
    time
}