use std::time::Instant;


#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
struct Blueprint {
    id: usize,
    ore_price: usize,   //ore
    clay_price: usize,  //ore
    obsi_price: (usize, usize), //ore, clay
    geode_price: (usize, usize),//ore, obsi
}

fn parse(s: &str) -> Blueprint {
    let s = s.strip_prefix("Blueprint ").unwrap();
    let (id, s) = s.split_once(": Each ore robot costs ").unwrap();
    let id = id.parse::<usize>().unwrap();
    let (ore_price, s) = s.split_once(" ore. Each clay robot costs ").unwrap();
    let ore_price = ore_price.parse::<usize>().unwrap();
    let (clay_price, s) = s.split_once(" ore. Each obsidian robot costs ").unwrap();
    let clay_price = clay_price.parse::<usize>().unwrap();
    let (obsi_price, s) = s.split_once(" clay. Each geode robot costs ").unwrap();
    let obsi_price = obsi_price.split_once(" ore and ").unwrap();
    let obsi_price = (obsi_price.0.parse::<usize>().unwrap(), obsi_price.1.parse::<usize>().unwrap());
    let s = s.strip_suffix(" obsidian.").unwrap();
    let geode_price = s.split_once(" ore and ").unwrap();
    let geode_price = (geode_price.0.parse::<usize>().unwrap(), geode_price.1.parse::<usize>().unwrap());

    Blueprint{id, ore_price, clay_price, obsi_price, geode_price}
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Factory {
    bp: Blueprint,
    resources: (usize, usize, usize),   //ore, clay, obsi
    robots: (usize, usize, usize, usize),//ore, clay, obsi and geodes
    max_resources: (usize, usize, usize),
}

fn factory_cycle(f: &Factory, time_left: usize) -> usize {
    if time_left == 0 {return 0;}
    let mut f = *f;
    let mut new_robots = (false, false, false, false);
    //try to build robots if we can, but not more than what we possibly need
    if f.bp.ore_price <= f.resources.0 {
        if f.robots.0 < f.max_resources.0 {
            new_robots.0 = true;
        }
    }
    if f.bp.clay_price <= f.resources.0 {
        if f.robots.1 < f.max_resources.1 {
            new_robots.1 = true;
        }
    }
    if f.bp.obsi_price.0 <= f.resources.0 && f.bp.obsi_price.1 <= f.resources.1 {
        if f.robots.2 < f.max_resources.1 {
            new_robots.2 = true;
        }
    }
    if f.bp.geode_price.0 <= f.resources.0 && f.bp.geode_price.1 <= f.resources.2 {
        new_robots.3 = true;
    }

    f.resources.0 += f.robots.0;
    f.resources.1 += f.robots.1;
    f.resources.2 += f.robots.2;

    let mut max_geodes = 0;

    if time_left > 12 {
        for i in 0..32-time_left {
            if i%8 == 0 {
                print!("|");
            } else {
                print!("-");
            }
        }
        println!("{:?}", f);
    }

    //if we can construct a geode robot each turn lets just do that, right
    if !(f.robots.0 >= f.bp.geode_price.0 && f.robots.2 >= f.bp.geode_price.1) {
        if new_robots.0 {
            f.robots.0 += 1;
            f.resources.0 -= f.bp.ore_price;
            max_geodes = std::cmp::max(max_geodes, factory_cycle(&f, time_left-1));
            f.resources.0 += f.bp.ore_price;
            f.robots.0 -= 1;
        }
        if new_robots.1 {
            f.robots.1 += 1;
            f.resources.0 -= f.bp.clay_price;
            max_geodes = std::cmp::max(max_geodes, factory_cycle(&f, time_left-1));
            f.resources.0 += f.bp.clay_price;
            f.robots.1 -= 1;
        }
        if new_robots.2 {
            f.robots.2 += 1;
            f.resources.0 -= f.bp.obsi_price.0;
            f.resources.1 -= f.bp.obsi_price.1;
            max_geodes = std::cmp::max(max_geodes, factory_cycle(&f, time_left-1));
            f.resources.0 += f.bp.obsi_price.0;
            f.resources.1 += f.bp.obsi_price.1;
            f.robots.2 -= 1;
        }
        if f.resources.0 < f.max_resources.0 || f.resources.1 < f.max_resources.1 || f.resources.2 < f.max_resources.2 {
            max_geodes = std::cmp::max(max_geodes, factory_cycle(&f, time_left-1));
        }
    } else {
        return f.robots.3 + (time_left * (time_left+1))/2;
    }

    if new_robots.3 {
        f.robots.3 += 1;
        f.resources.0 -= f.bp.geode_price.0;
        f.resources.2 -= f.bp.geode_price.1;
        max_geodes = std::cmp::max(max_geodes, factory_cycle(&f, time_left-1));
        f.resources.0 += f.bp.geode_price.0;
        f.resources.2 += f.bp.geode_price.1;
        f.robots.3 -= 1;
    }


    // f.robots.0 += new_robots.0;
    // f.robots.1 += new_robots.1;
    // f.robots.2 += new_robots.2;
    // f.robots.3 += new_robots.3;
    max_geodes + f.robots.3
}

pub fn chall_1(s: &str) -> usize {
    let mut blueprints = Vec::new();
    for l in s.lines() {
        blueprints.push(parse(l));
    }
    let mut total = 0;
    let mut clock = Instant::now();
    for b in blueprints {
        let f = Factory{
            bp: b,
            resources: (0, 0, 0),
            robots: (1, 0, 0, 0),
            max_resources: (std::cmp::max(std::cmp::max(b.ore_price-1, b.clay_price), std::cmp::max(b.obsi_price.0, b.geode_price.0)), b.obsi_price.1, b.geode_price.1),
        };
        let result = factory_cycle(&f, 24) * b.id;
        println!("factory {}: {result} in {:?}", b.id, clock.elapsed());
        clock = Instant::now();
        total += result;
    }
    total
}

pub fn chall_2(s: &str) -> usize {
    let mut blueprints = Vec::new();
    for l in s.lines() {
        blueprints.push(parse(l));
    }
    let mut total = 1;
    let mut clock = Instant::now();
    for i in 0..3 {
        if i > blueprints.len() {break;}    //example data
        let b = blueprints[i];
        let f = Factory{
            bp: b,
            resources: (0, 0, 0),
            robots: (1, 0, 0, 0),
            max_resources: (std::cmp::max(std::cmp::max(b.ore_price-1, b.clay_price), std::cmp::max(b.obsi_price.0, b.geode_price.0)), b.obsi_price.1, b.geode_price.1),
        };
        let result = factory_cycle(&f, 32);
        println!("factory {}: {result} in {:?}", b.id, clock.elapsed());
        clock = Instant::now();
        total *= result;
    }
    total
}
