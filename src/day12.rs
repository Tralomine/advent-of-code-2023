fn parse(s: &str) -> (Vec<Vec<(u8, usize)>>, (usize, usize), (usize, usize)) {
    let mut heightmap = Vec::new();
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    for (x, l) in s.lines().enumerate() {
        heightmap.push(Vec::new());
        for (y, c) in l.as_bytes().iter().enumerate() {
            let c = *c as char;
            if c == 'S' {
                pos1 = (x, y);
                heightmap[x].push((0, usize::MAX));
            } else if c == 'E' {
                pos2 = (x, y);
                heightmap[x].push((25, usize::MAX));
            } else {
                heightmap[x].push((c as u8 - 'a' as u8, usize::MAX));
            }
        }
    }
    (heightmap, pos1, pos2)
}

fn astar_weight(hm: &mut Vec<Vec<(u8, usize)>>, pos: (usize, usize), weight: usize) {
    if hm[pos.0][pos.1].1 != usize::MAX && hm[pos.0][pos.1].1 <= weight {
        return;
    }
    hm[pos.0][pos.1].1 = weight;
    //should start with the lowest one, to avoid calculating some multiple times
    if pos.0 > 0 {
        let new_pos = (pos.0-1, pos.1);
        if hm[pos.0][pos.1].0 <= hm[new_pos.0][new_pos.1].0 + 1 {
            astar_weight(hm, new_pos, weight+1);
        }
    }
    if pos.0 < hm.len()-1 {
        let new_pos = (pos.0+1, pos.1);
        if hm[pos.0][pos.1].0 <= hm[new_pos.0][new_pos.1].0 + 1 {
            astar_weight(hm, new_pos, weight+1);
        }
    }
    if pos.1 > 0 {
        let new_pos = (pos.0, pos.1-1);
        if hm[pos.0][pos.1].0 <= hm[new_pos.0][new_pos.1].0 + 1 {
            astar_weight(hm, new_pos, weight+1);
        }
    }
    if pos.1 < hm[0].len()-1 {
        let new_pos = (pos.0, pos.1+1);
        if hm[pos.0][pos.1].0 <= hm[new_pos.0][new_pos.1].0 + 1 {
            astar_weight(hm, new_pos, weight+1);
        }
    }
}

pub fn chall_1(s : &String) -> usize {
    let (mut heightmap, curr_pos, end) = parse(s);
    astar_weight(&mut heightmap, end, 0);
    // dbg!(heightmap, curr_pos, end);
    heightmap[curr_pos.0][curr_pos.1].1
}

pub fn chall_2(s : &String) -> usize {
    let (mut heightmap, _, end) = parse(s);
    astar_weight(&mut heightmap, end, 0);
    let mut min = usize::MAX;
    for i in heightmap {
        for j in i {
            if j.0 == 0 && j.1 < min {
                min = j.1;
            }
        }
    }
    min
}