#[derive(Debug, Clone)]
struct Map {
    items: Vec<[i64;3]>,
}

impl Map {
    fn new() -> Map {
        Map {
            items: Vec::new(),
        }
    }

    fn convert(self, n: i64) ->  i64 {
        for i in self.items {
            if i[1] <= n && i[1]+i[2] >= n {
                return n - i[1] + i[0];
            }
        }
        n
    }

    fn cut_range(self, n: (i64, i64)) -> Vec<(i64, i64)> {
        let mut ranges = Vec::new();
        ranges.push(n);
        for i in self.items {
            
        }
        ranges
    }

    fn get_ranges(self, n: (i64, i64)) -> Vec<(i64, i64)> {
        let mut ranges = Vec::new();
        for i in self.items {
            if i[1] <= n.0  && i[1]+i[2] >= n.0 {
                if i[1]+i[2] <= n.0+n.1{
                    ranges.push((n.0-i[1]+i[0], n.1));
                } else {
                    ranges.push((n.0-i[1]+i[0], i[1]+i[2] - n.0))
                }
            } else if i[1] >= n.0  && i[1] <= n.0+n.1 {
                if i[1]+i[2] <= n.0+n.1{
                    ranges.push((i[0], i[2]));
                } else {
                    ranges.push((i[0], n.0-i[1]+i[2]))
                }
            } else {
                ranges.push(n);
            }
        }
        ranges
    }
}


fn parse(s: &str) -> (Vec<i64>, Vec<Map>) {
    let first_line = s.lines().collect::<Vec<&str>>()[0];
    let mut seed_list = Vec::new();
    for seed in first_line.strip_prefix("seeds: ").unwrap().split(" ") {
        seed_list.push(seed.parse().unwrap());
    }
    let mut maps = Vec::new();
    let mut cur_map = Map::new();
    for l in s.lines() {
        if l.starts_with("seeds") {continue;}
        if l.ends_with("map:") {continue;}
        if l.is_empty() {
            if cur_map.items.len() > 0 {
                maps.push(cur_map.clone());
            }
            cur_map = Map::new();
            continue;
        }
        cur_map.items.push(parse_line(l))
    }
    if cur_map.items.len() > 0 {
        maps.push(cur_map.clone());
    }
    (seed_list, maps)
}

fn parse_line(s: &str) -> [i64;3] {
    let s : Vec<&str> = s.split(" ").collect();
    [s[0].parse().unwrap(), s[1].parse().unwrap(), s[2].parse().unwrap()]
}

pub fn chall_1(s : &String) -> i64 {
    let (seeds, maps) = parse(s);
    let mut min = i64::MAX;
    for s in seeds {
        let mut i = s;
        for m in &maps {
            i = m.clone().convert(i);
        }
        if i < min {
            min = i;
        }
    }
    min
}

pub fn chall_2(s : &String) -> i64 {
    let (seeds, maps) = parse(s);
    let mut seed_ranges = Vec::new();
    for i in 0..seeds.len()/2 {
        seed_ranges.push((seeds[i*2], seeds[i*2+1]));
    }
    for m in &maps {
        dbg!(&m, &seed_ranges);
        let mut new_ranges = Vec::new();
        for r in seed_ranges {
            let mut r = m.clone().get_ranges(r);
            dbg!(&m, &r);
            new_ranges.append(&mut r);
        }
        seed_ranges = new_ranges;
    }

    let mut min = i64::MAX;
    for (i, _) in seed_ranges {
        if i < min {
            min = i;
        }
    }
    min
}
