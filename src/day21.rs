use std::collections::HashMap;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Sub,
    Div,
    None,
}

#[derive(Clone)]
#[derive(Debug)]
struct Monkey {
    name: String,
    operation: Option<(String, Op, String)>,
    number: Option<i64>,
}

impl Monkey {
    fn new() -> Self {
        Monkey{
            name: String::new(),
            operation: None,
            number: None,
        }
    }
}

fn parse_line(s: &str) -> Monkey {
    let mut m = Monkey::new();
    let s = s.split_once(": ").unwrap();
    m.name = String::from(s.0);
    match s.1.parse::<i64>() {
        Ok(k) => m.number = Some(k),
        Err(_) => {
            let mut s = s.1.split(" ");
            let mut op = (String::new(), Op::Add, String::new());
            op.0 = String::from(s.next().unwrap());
            op.1 = match s.next().unwrap().as_bytes()[0] as char {
                '+' => Op::Add,
                '*' => Op::Mul,
                '/' => Op::Div,
                '-' => Op::Sub,
                _ => Op::None,
            };
            op.2 = String::from(s.next().unwrap());
            m.operation = Some(op);
        }
    }
    m
}

fn calc_monkey(name: &str, monkeys: &mut HashMap<String, Monkey>) -> i64 {
    let m = monkeys[name].clone();
    if m.operation == None {
        m.number.unwrap()
    } else {
        let n1 = calc_monkey(&m.operation.clone().unwrap().0, monkeys);
        let n2 = calc_monkey(&m.operation.clone().unwrap().2, monkeys);
        let n = match m.operation.unwrap().1 {
            Op::Add => n1 + n2,
            Op::Mul => n1 * n2,
            Op::Div => n1 / n2,
            Op::Sub => n1 - n2,
            Op::None => {panic!("bruh");},
        };
        monkeys.get_mut(name).unwrap().number = Some(n);
        monkeys.get_mut(name).unwrap().operation = None;
        monkeys[name].number.unwrap()
    }
}

pub fn chall_1(s: &str) -> i64 {
    let mut monkeys = HashMap::new();
    for l in s.lines() {
        let m = parse_line(l);
        monkeys.insert(m.name.clone(), m);
    }
    calc_monkey("root", &mut monkeys)
}

fn has_humn(name: &str,monkeys: &HashMap<String, Monkey>) -> bool {
    if monkeys[name].operation == None {
        name == "humn"
    } else {
        has_humn(&monkeys[name].operation.clone().unwrap().0, monkeys) ||
        has_humn(&monkeys[name].operation.clone().unwrap().2, monkeys)
    }
}

fn invert_op(v:i64, name: &str,monkeys: &mut HashMap<String, Monkey>) -> i64 {
    if name == "humn" {return v;}
    let cur = monkeys[name].operation.clone().unwrap();
    let mut hm_l = false;
    if has_humn(&cur.0.clone(), &monkeys) {
        hm_l = true;
        monkeys.get_mut(name).unwrap().operation = Some((
            cur.2.clone(), cur.1, cur.0.clone()
        ))
    }
    let left = calc_monkey(&monkeys[name].operation.as_ref().unwrap().0.clone(), monkeys);

    let value;
    match cur.1 {
        Op::Add => value = v - left,
        Op::Mul => value = v / left,
        Op::Div => if hm_l {value = left * v} else {value = left / v},
        Op::Sub => if hm_l {value = left + v} else {value = left - v},
        _ => {panic!("bruh again");},
    }

    invert_op(value, &monkeys[name].operation.as_ref().unwrap().2.clone(), monkeys)
}

pub fn chall_2(s: &str) -> i64 {
    let mut monkeys = HashMap::new();
    for l in s.lines() {
        let m = parse_line(l);
        monkeys.insert(m.name.clone(), m);
    }
    let root = monkeys["root"].operation.as_ref().unwrap();
    if has_humn(&root.0.clone(), &monkeys) {
        monkeys.get_mut("root").unwrap().operation = Some((
            root.2.clone(),
            root.1,
            root.0.clone()
        ))
    }
    let left = calc_monkey(&monkeys["root"].operation.as_ref().unwrap().0.clone(), &mut monkeys);
    invert_op(left, &monkeys["root"].operation.as_ref().unwrap().2.clone(), &mut monkeys)
}
