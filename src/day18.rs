fn parse(s: &str) -> (usize, usize, usize) {
    let mut ns = s.split(',');
    let mut ret = (0, 0, 0);
    ret.0 = ns.next().unwrap().parse::<usize>().unwrap();
    ret.1 = ns.next().unwrap().parse::<usize>().unwrap();
    ret.2 = ns.next().unwrap().parse::<usize>().unwrap();
    ret
}

pub fn chall_1(s: &str) -> i64 {
    let mut droplets = Vec::new();
    for l in s.lines() {
        droplets.push(parse(l));
    }
    let mut total_faces = 0;
    for d in droplets.iter() {
        let mut faces = 6;
        if droplets.contains(&(d.0+1, d.1, d.2)) {faces -= 1;}
        if droplets.contains(&(d.0.wrapping_sub(1), d.1, d.2)) {faces -= 1;}
        if droplets.contains(&(d.0, d.1+1, d.2)) {faces -= 1;}
        if droplets.contains(&(d.0, d.1.wrapping_sub(1), d.2)) {faces -= 1;}
        if droplets.contains(&(d.0, d.1, d.2+1)) {faces -= 1;}
        if droplets.contains(&(d.0, d.1, d.2.wrapping_sub(1))) {faces -= 1;}
        total_faces += faces;
    }
    total_faces
}

use std::cmp;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
enum Elem {
    Lava,
    Air,
    Water,
}

fn fill(grid: &mut Vec<Vec<Vec<Elem>>>, start: (usize, usize, usize)) {
    if start.0 >= grid.len() || start.1 >= grid[0].len() || start.2 >= grid[0][0].len() {
        return;
    }
    if grid[start.0][start.1][start.2] == Elem::Air {
        grid[start.0][start.1][start.2] = Elem::Water;
    } else {
        return;
    }
    fill(grid, (start.0+1, start.1, start.2));
    fill(grid, (start.0.wrapping_sub(1), start.1, start.2));
    fill(grid, (start.0, start.1+1, start.2));
    fill(grid, (start.0, start.1.wrapping_sub(1), start.2));
    fill(grid, (start.0, start.1, start.2+1));
    fill(grid, (start.0, start.1, start.2.wrapping_sub(1)));
}

pub fn chall_2(s: &str) -> i64 {
    let mut droplets = Vec::new();
    let mut max = (0, 0, 0);
    for l in s.lines() {
        droplets.push(parse(l));
        let d = droplets.last().unwrap();
        max.0 = cmp::max(max.0, d.0);
        max.1 = cmp::max(max.1, d.1);
        max.2 = cmp::max(max.2, d.2);
    }
    let mut grid = Vec::new();
    for x in 0..max.0+3 {
        grid.push(Vec::new());
        for y in 0..max.1+3 {
            grid[x].push(Vec::new());
            for z in 0..max.2+3 {
                if droplets.contains(&(x.wrapping_sub(1), y.wrapping_sub(1), z.wrapping_sub(1))) {
                    grid[x][y].push(Elem::Lava);
                } else {
                    grid[x][y].push(Elem::Air);
                }
            }
        }
    }

    fill(&mut grid, (0, 0, 0));

    let mut total_faces = 0;
    for x in 1..max.0+2 {
        for y in 1..max.1+2 {
            for z in 1..max.2+2 {
                if grid[x][y][z] == Elem::Lava {
                    if grid[x+1][y][z] == Elem::Water {total_faces += 1;}
                    if grid[x-1][y][z] == Elem::Water {total_faces += 1;}
                    if grid[x][y+1][z] == Elem::Water {total_faces += 1;}
                    if grid[x][y-1][z] == Elem::Water {total_faces += 1;}
                    if grid[x][y][z+1] == Elem::Water {total_faces += 1;}
                    if grid[x][y][z-1] == Elem::Water {total_faces += 1;}
                //     print!("{}", ansi_term::Colour::Red.paint("█"));
                // } else if grid[x][y][z] == Elem::Water {
                //     print!("{}", ansi_term::Colour::Blue.paint("~"));
                // } else {
                //     print!(" ")
                }
            }
            // println!("");
        }
        // println!("");
    }
    total_faces
}