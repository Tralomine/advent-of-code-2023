
pub fn chall_1(s : &String) -> i32 {
    let width = s.lines().next().unwrap().len();
    let height = s.lines().count();
    let mut grid = vec![vec![0;width];height];

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.as_bytes().iter().enumerate() {
            grid[y][x] = (c - '0' as u8) as i32;
        }
    }

    let mut visible_trees = 0;
    for x in 0..width {
        for y in 0..height {
            let tree = grid[x][y];
            let mut is_visible = [true;4];
            for x1 in 0..x {
                if grid[x1][y] >= tree {
                    is_visible[0] = false;
                }
            }
            for x1 in x+1..width {
                if grid[x1][y] >= tree {
                    is_visible[1] = false;
                }
            }
            for y1 in 0..y {
                if grid[x][y1] >= tree {
                    is_visible[2] = false;
                }
            }
            for y1 in y+1..height {
                if grid[x][y1] >= tree {
                    is_visible[3] = false;
                }
            }
            if is_visible[0] || is_visible[1] || is_visible[2] || is_visible[3] {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

use std::cmp::max;

pub fn chall_2(s : &String) -> i32 {
    let width = s.lines().next().unwrap().len();
    let height = s.lines().count();
    let mut grid = vec![vec![0;width];height];

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.as_bytes().iter().enumerate() {
            grid[y][x] = (c - '0' as u8) as i32;
        }
    }

    let mut max_scenic_score = 0;
    for x in 0..width {
        for y in 0..height {
            let tree = grid[x][y];
            let mut scenic_score = [0;4];
            for x1 in (0..x).rev() {
                scenic_score[0] += 1;
                if grid[x1][y] >= tree {break;}
            }
            for x1 in x+1..width {
                scenic_score[1] += 1;
                if grid[x1][y] >= tree {break;}
            }
            for y1 in (0..y).rev() {
                scenic_score[2] += 1;
                if grid[x][y1] >= tree {break;}
            }
            for y1 in y+1..height {
                scenic_score[3] += 1;
                if grid[x][y1] >= tree {break;}
            }
            max_scenic_score = max(max_scenic_score, scenic_score[0]*scenic_score[1]*scenic_score[2]*scenic_score[3]);
        }
    }
    max_scenic_score
}