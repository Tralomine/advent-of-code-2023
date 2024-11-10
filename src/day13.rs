#[derive(PartialEq)]
enum ElemType {
    Int,
    List,
}
struct Elem {
    list: Vec<Elem>,
    int: i64,
    t: ElemType,
}

impl Elem {
    fn new_int(int: i64) -> Elem {
        Elem{list: Vec::new(), int, t: ElemType::Int}
    }
    fn new_list() -> Elem {
        Elem{list: Vec::new(), int: 0, t: ElemType::List}
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        !(self < other) && !(self > other)
    }
}

impl Eq for Elem {}

impl std::fmt::Debug for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.t == ElemType::Int {
            write!(f, "{}", &self.int)
        } else {
            f.debug_list().entries(&self.list).finish()
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            Some(std::cmp::Ordering::Less)
        } else if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if self.t == ElemType::Int && other.t == ElemType::Int {
            self.int < other.int
        } else if self.t == ElemType::List && other.t == ElemType::Int {
            let mut new = Elem::new_list();
            new.list.push(Elem::new_int(other.int));
            self < &new
        } else if self.t == ElemType::Int && other.t == ElemType::List {
            let mut new = Elem::new_list();
            new.list.push(Elem::new_int(self.int));
            &new < other
        } else {
            for i in 0..self.list.len() {
                if other.list.get(i) == None {
                    return false;   //right is shorter
                }
                if self.list[i] < other.list[i] {
                    return true;
                } else if other.list[i] < self.list[i] {
                    return false;
                }
            }
            if other.list.len() > self.list.len() {
                true    //left is shorter, so comes before
            } else {
                false   //lists are equal
            }
        }
    }
    fn le(&self, other: &Self) -> bool {
        self < other && self == other
    }
    fn gt(&self, other: &Self) -> bool {
        other < self
    }
    fn ge(&self, other: &Self) -> bool {
        self > other && self == other
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(s: &str) -> Elem {
    let mut stack: Vec<Elem> = Vec::new();
    let mut curvalue = -1;
    for c in s.as_bytes() {
        if *c as char == '[' {
            stack.push(Elem::new_list());
        } else if *c as char == ']' {
            if curvalue > -1 {
                let len = stack.len();
                stack[len-1].list.push(Elem::new_int(curvalue));
            }
            curvalue = -1;
            let last = stack.pop().unwrap();
            if stack.len() == 0 {
                return last;
            } else {
                if stack.last().unwrap().t == ElemType::List {
                    let len = stack.len();
                    stack[len-1].list.push(last);
                }
            }
        } else if *c as char == ',' {
            if curvalue > -1 {
                let len = stack.len();
                stack[len-1].list.push(Elem::new_int(curvalue));
            }
            curvalue = -1;
        } else {
            if curvalue < 0 {
                curvalue = (*c - '0' as u8) as i64;
            } else {
                curvalue *= 10;
                curvalue += (*c - '0' as u8) as i64;
            }
        }
    }
    Elem::new_int(0)
}

pub fn chall_1(s: &str) -> usize {
    let mut l = s.lines();
    let mut ordered = 0;
    let mut i = 1;
    loop {
        let first = parse(l.next().unwrap());
        let second = parse(l.next().unwrap());
        if first < second {
            ordered += i;
        }
        if let None = l.next() {
            break ordered
        }
        i += 1;
    }
}

pub fn chall_2(s: &str) -> i64 {
    let mut lines = Vec::new();
    for l in s.lines() {
        if l.len() > 0 {
            lines.push(parse(l));
        }
    }
    let div1 = parse("[[2]]");
    let div2 = parse("[[6]]");
    lines.push(div1);
    lines.push(div2);
    let div1 = parse("[[2]]");
    let div2 = parse("[[6]]");
    lines.sort();
    let mut pos1 = 1;
    let mut pos2 = 1;
    for (i, l) in lines.iter().enumerate() {
        if *l == div1 {
            pos1 = i as i64+1
        }
        if *l == div2 {
            pos2 = i as i64 + 1
        }
    }
    pos1 * pos2
}
