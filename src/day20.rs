fn parse(s: &str) -> Vec<i64>{
    let mut numbers = Vec::new();
    for l in s.lines() {
        numbers.push(l.parse::<i64>().unwrap());
    }
    numbers
}

pub fn chall_1(s: &str) -> i64 {
    let mut numbers = parse(s);
    let mut indexes = Vec::new();
    let len = numbers.len();
    for i in 0..len {
        indexes.push(i);
    }

    // println!("{:?}", &numbers);
    for i in 0..len {
        let index = {
            let mut index = 0;
            for k in 0..len {
                if indexes[k] == i {
                    index = k;
                    break;
                }
            } index
        };
        let n = numbers.remove(index);
        indexes.remove(index);
        let index = {
            let mut k = n+index as i64;
            if k == 0 || k == len as i64 {k = len as i64 - 1;}
            if k > len as i64 {
                (k%(len as i64 - 1)) as usize
            } else if k < 0 {
                k.rem_euclid(len as i64-1) as usize
            } else {k as usize}
        };
        numbers.insert(index, n);
        indexes.insert(index, i);
        // println!("{:?}", &numbers);
    }

    let pos_zero = {
        let mut i = 0;
        for k in 0..len {
            if numbers[k] == 0 {
                i = k;
                break;
            }
        } i
    };
    numbers[(pos_zero+1000)%len] + numbers[(pos_zero+2000)%len] + numbers[(pos_zero+3000)%len]
}

pub fn chall_2(s: &str) -> i64 {
    let mut numbers = parse(s);
    let mut indexes = Vec::new();
    let len = numbers.len();
    for i in 0..len {
        indexes.push(i);
        numbers[i] *= 811589153;
    }

    // println!("{:?}", &numbers);
    for _ in 0..10 {
        for i in 0..len {
            let index = {
                let mut index = 0;
                for k in 0..len {
                    if indexes[k] == i {
                        index = k;
                        break;
                    }
                } index
            };
            let n = numbers.remove(index);
            indexes.remove(index);
            let index = {
                let mut k = n+index as i64;
                if k == 0 || k == len as i64 {k = len as i64 - 1;}
                if k > len as i64 {
                    (k%(len as i64 - 1)) as usize
                } else if k < 0 {
                    k.rem_euclid(len as i64-1) as usize
                } else {k as usize}
            };
            numbers.insert(index, n);
            indexes.insert(index, i);
        }
        // println!("{:?}", &numbers);
    }

    let pos_zero = {
        let mut i = 0;
        for k in 0..len {
            if numbers[k] == 0 {
                i = k;
                break;
            }
        } i
    };
    numbers[(pos_zero+1000)%len] + numbers[(pos_zero+2000)%len] + numbers[(pos_zero+3000)%len]
}
