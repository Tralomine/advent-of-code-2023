#[derive(Clone)]
#[derive(Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
enum Cave {
    Air,
    Rock,
}


fn get_rock(n: usize) -> Vec<(usize, usize)> {
    let rocks = [vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)],
                vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (1, 0), (0, 1), (1, 1)]];
    rocks[n%rocks.len()].clone()
}

fn parse(s: &str) -> Vec<Dir> {
    let mut dirs = Vec::new();
    for c in s.as_bytes() {
        match *c as char {
            '>' => dirs.push(Dir::Right),
            '<' => dirs.push(Dir::Left),
            _ => (),
        }
    }
    dirs
}

fn get_top(cave: &Vec<[Cave;7]>) -> usize {
    let mut top = cave.len();
    'full: for l in cave.iter().rev() {
        for r in l {
            if *r == Cave::Rock {
                break 'full;
            }
        }
        top -= 1;
    }
    top
}

pub fn chall_1(s: &str) -> usize {
    let dirs = parse(s);
    let mut tetris: Vec<[Cave;7]> = Vec::new();
    tetris.push([Cave::Air;7]);
    let mut cur_dir = 0;
    for n in 0..2022 {
        let top = get_top(&tetris);
        for _ in 0..(7-(tetris.len()-top)) {
            tetris.push([Cave::Air;7]);
        }
        let mut new_rock = get_rock(n);
        for p in new_rock.iter_mut() {
            *p = (p.0+top+3, p.1+2)   //flip the piece it's not in the right direction ^^'
        }

        'falling_rock: loop {
            let dir = dirs[cur_dir%dirs.len()];
            cur_dir += 1;

            if {    //move on the sides
                let mut can_move = true;
                for p in &new_rock {
                    match dir {
                        Dir::Left => if p.1 == 0 || tetris[p.0][p.1-1] == Cave::Rock {
                            can_move = false;
                            break;
                        },
                        Dir::Right => if p.1 == 6 || tetris[p.0][p.1+1] == Cave::Rock {
                            can_move = false;
                            break;
                        },
                    }
                }
                can_move
            } {
                for p in new_rock.iter_mut() {
                    match dir {
                        Dir::Left => *p = (p.0, p.1-1),
                        Dir::Right => *p = (p.0, p.1+1),
                    }
                }
            }

            if {    //move down
                let mut can_move = true;
                for p in &new_rock {
                    if p.0 == 0 || tetris[p.0-1][p.1] == Cave::Rock {
                        can_move = false;
                        break;
                    }
                }
                can_move
            } {
                for p in new_rock.iter_mut() {
                    *p = (p.0-1, p.1);
                }
            } else {
                for p in new_rock {
                    tetris[p.0][p.1] = Cave::Rock;  //place piece
                }
                break 'falling_rock;
            }
        }

        // for l in tetris.iter().rev() {
        //     print!("|");
        //     for r in l {
        //         match r {
        //             Cave::Air => print!(" "),
        //             Cave::Rock => print!("#"),
        //         }
        //     }
        //     println!("|");
        // }
        // println!("+-------+");
        // std::io::stdin().read_line(&mut String::new());
    }
    get_top(&tetris)
}


pub fn chall_2(s: &str) -> usize {
    let mut true_bottom = 0; // me lmao
    let dirs = parse(s);
    let mut tetris: Vec<[Cave;7]> = Vec::new();
    tetris.push([Cave::Air;7]);
    let mut cur_dir = 0;

    let min_period = dirs.len()*5;
    let mut lengths = Vec::new();
    let mut n = 0;

    let mut period_check_min = 0;   //sieve until I find a length above which all are unique in the period
    let mut period = 0;
    let mut period_height = 0;
    let mut period_height_calc = usize::MAX;
    let mut finish = false;

    const MAX: usize = 1_000_000_000_000; 
    while n < MAX {
        if period == 0 && n%min_period == 0 {
            lengths.push(tetris.len());
            if tetris.len() > period_check_min {
                for (i, l) in lengths[lengths.len()/2..lengths.len()-1].iter().rev().enumerate() {
                    if *l == tetris.len() {
                        //I know I'm only comparing the height of the piles and not the whole piles so idk if it's trully a period but idc I just want an answer and it's enough in this case
                        let period_candidate = i+1;
                        let mut true_period = true;
                        for k in 0..lengths.len()-period_candidate {
                            if lengths[k] != lengths[k+period_candidate] {
                                true_period = false;
                                break;
                            }
                        }
                        if true_period {
                            period = period_candidate * min_period;
                            // println!("Period found! length {period}");
                        } else {
                            period_check_min += 5;
                            lengths.clear();
                        }
                        break;
                    }
                }
            }
        }

        if period > 0 {
            if period_height == 0 {
                period_height = true_bottom; //start
                period_height_calc = period;
                finish = true;  //dirty hack
            }
            if period_height_calc == 0 {
                period_height = true_bottom - period_height; //end
                // println!("period height: {}", period_height);
                period_height_calc = usize::MAX;
                finish = false;  //dirty hack
            }
            if period_height_calc < usize::MAX {
                period_height_calc -= 1;
            }
        }


        let top = get_top(&tetris); //my girlfriend
        for _ in 0..(7-(tetris.len()-top)) {
            tetris.push([Cave::Air;7]);
        }
        let mut new_rock = get_rock(n);
        for p in new_rock.iter_mut() {
            *p = (p.0+top+3, p.1+2)   //flip the piece it's not in the right direction ^^'
        }

        'falling_rock: loop {
            let dir = dirs[cur_dir%dirs.len()];
            cur_dir += 1;

            if {    //move on the sides
                let mut can_move = true;
                for p in &new_rock {
                    match dir {
                        Dir::Left => if p.1 == 0 || tetris[p.0][p.1-1] == Cave::Rock {
                            can_move = false;
                            break;
                        },
                        Dir::Right => if p.1 == 6 || tetris[p.0][p.1+1] == Cave::Rock {
                            can_move = false;
                            break;
                        },
                    }
                }
                can_move
            } {
                for p in new_rock.iter_mut() {
                    match dir {
                        Dir::Left => *p = (p.0, p.1-1),
                        Dir::Right => *p = (p.0, p.1+1),
                    }
                }
            }

            if {    //move down
                let mut can_move = true;
                for p in &new_rock {
                    if p.0 == 0 || tetris[p.0-1][p.1] == Cave::Rock {
                        can_move = false;
                        break;
                    }
                }
                can_move
            } {
                for p in new_rock.iter_mut() {
                    *p = (p.0-1, p.1);
                }
            } else {
                for p in new_rock {
                    tetris[p.0][p.1] = Cave::Rock;  //place piece
                }
                break 'falling_rock;
            }
        }

        let bottom = {
            let mut opening = [true;7];
            let top = get_top(&tetris);
            let mut bottom = 0;
            for i in (0..top).rev() {
                let mut new_opening = [true;7];
                for j in 0..7 {
                    if tetris[i][j] == Cave::Rock {
                        new_opening[j] = false;
                    } else {
                        let mut opened = false;
                        for k in (0..j).rev() {
                            if tetris[i][k] == Cave::Rock {break;}
                            if opening[k] == true {opened = true;}
                        }
                        for k in j..7 {
                            if tetris[i][k] == Cave::Rock {break;}
                            if opening[k] == true {opened = true;}
                        }
                        if !opened {
                            new_opening[j] = false;
                        }
                    }
                }
                opening = new_opening;

                let mut opened = false;
                for o in opening {
                    if o {
                        opened = true;
                        break;
                    }
                }
                if !opened {
                    bottom = i;
                    break;
                }
            }
            bottom
        };

        tetris.drain(0..bottom);
        true_bottom += bottom;

        if period == 0 || finish == true {
            n += 1;
        } else {
            let left = MAX - n;
            // println!("calculating {} periods", left/period);
            true_bottom += (left/period) * period_height; //add the height of all the reminding periods
            n = MAX - left%period + 1;  //move n to after everything when there's less than a period left
            finish = true;
        }
    }
    true_bottom + get_top(&tetris)
}