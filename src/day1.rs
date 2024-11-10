pub fn chall_1(s : &String) -> i32 {
    let mut sum = 0;
    for line in s.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                sum += 10 * (c.to_digit(10).unwrap() as i32);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                sum += c.to_digit(10).unwrap() as i32;
                break;
            }
        }
    }
    sum
}

pub fn chall_2(s : &String) -> i32 {
    let mut sum = 0;
    for line in s.lines() {
        let line = line.replace("one", "o1e");
        let line = line.replace("two", "t2o");
        let line = line.replace("three", "t3e");
        let line = line.replace("four", "4");
        let line = line.replace("five", "5e");
        let line = line.replace("six", "6");
        let line = line.replace("seven", "7");
        let line = line.replace("eight", "e8t");
        let line = line.replace("nine", "9e");
        for c in line.chars() {
            if c.is_ascii_digit() {
                sum += 10 * (c.to_digit(10).unwrap() as i32);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                sum += c.to_digit(10).unwrap() as i32;
                break;
            }
        }
    }
    sum
}