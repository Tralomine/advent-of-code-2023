#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: (i64, char, i64),
    divisibility: i64,
    monkey_true: usize,
    monkey_false: usize,
    total_inspected: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: (-1, ' ', -1),
            divisibility: 0,
            monkey_true: 0,
            monkey_false: 0,
            total_inspected: 0,
        }
    }
}

fn parse(s: &str) -> Vec<Monkey> {
    let mut lines = s.split("\n");
    let mut monkeys = Vec::new();
    loop {
        let mut new_monkey = Monkey::new();
        if let None = lines.next() {break}; //Monkey n
        let items = lines.next().unwrap().strip_prefix("  Starting items: ").unwrap();
        let items = items.split(", ");
        for i in items {
            new_monkey.items.push(i.parse::<i64>().unwrap());
        }
        new_monkey.operation = {
            let op = lines.next().unwrap().strip_prefix("  Operation: new = ").unwrap();
            let mut op = op.split(' ');
            let first = if let Ok(n) = op.next().unwrap().parse::<i64>() {n} else {-1};
            let operator = op.next().clone().unwrap().as_bytes()[0] as char;
            let last = if let Ok(n) = op.next().unwrap().parse::<i64>() {n} else {-1};
            (first, operator, last)
        };
        new_monkey.divisibility = lines.next().unwrap()
                .strip_prefix("  Test: divisible by ").unwrap()
                .parse::<i64>().unwrap();
        new_monkey.monkey_true = lines.next().unwrap()
                .strip_prefix("    If true: throw to monkey ").unwrap()
                .parse::<usize>().unwrap();
        new_monkey.monkey_false = lines.next().unwrap()
                .strip_prefix("    If false: throw to monkey ").unwrap()
                .parse::<usize>().unwrap();
        monkeys.push(new_monkey);
        if let None = lines.next() {break};
    }
    monkeys
}

fn round(monkeys: Vec::<Monkey>, worry: bool) -> Vec::<Monkey> {
    let mut monkeys = monkeys;
    let mut modulus = 1;
    for m in &monkeys {
        modulus *= m.divisibility;
    }
    for i in 0..monkeys.len() {
        monkeys[i].items.reverse();
        while monkeys[i].items.len() > 0 {
            let mut item = monkeys[i].items.pop().unwrap();
            item = match monkeys[i].operation.1 {
                '+' => item + if monkeys[i].operation.2 == -1 {item} else {monkeys[i].operation.2},
                '*' => item * if monkeys[i].operation.2 == -1 {item} else {monkeys[i].operation.2},
                _ => item,
            };
            if !worry {
                item = item / 3;
            }
            item = item%modulus;
            if item % monkeys[i].divisibility == 0 {
                let m_true = monkeys[i].monkey_true;
                monkeys[m_true].items.push(item);
            } else {
                let m_false = monkeys[i].monkey_false;
                monkeys[m_false].items.push(item);
            }
            monkeys[i].total_inspected += 1;
        }
    }
    monkeys
}

pub fn chall_1(s : &String) -> usize {
    let mut monkeys = parse(s);
    for _ in 0..20 {
        monkeys = round(monkeys, false);
    }
    let mut max = (0, 0);
    for m in monkeys {
        dbg!(m.total_inspected);
        if max.0 < m.total_inspected {
            max = (m.total_inspected, max.0);
        } else if max.1 < m.total_inspected {
            max.1 = m.total_inspected;
        }
    }
    max.0 * max.1
}

pub fn chall_2(s : &String) -> usize {
    let mut monkeys = parse(s);
    for _ in 0..10000 {
        monkeys = round(monkeys, true);
    }
    let mut max = (0, 0);
    for m in monkeys {
        dbg!(m.total_inspected);
        if max.0 < m.total_inspected {
            max = (m.total_inspected, max.0);
        } else if max.1 < m.total_inspected {
            max.1 = m.total_inspected;
        }
    }
    max.0 * max.1
}