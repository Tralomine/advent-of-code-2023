use std::fs;
// use std::time;
// let now = time::Instant::now();
// dbg!(now.elapsed());

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {

    let input_1 = fs::read_to_string("input/day1.input").expect("can't read file 1");
    println!("day 1 challenges :");
    println!("\tchall 1: {}", day1::chall_1(&input_1));
    println!("\tchall 2: {}", day1::chall_2(&input_1));

    let input_2 = fs::read_to_string("input/day2.input").expect("can't read file 2");
    println!("day 2 challenges :");
    println!("\tchall 1: {}", day2::chall_1(&input_2));
    println!("\tchall 2: {}", day2::chall_2(&input_2));

    let input_3 = fs::read_to_string("input/day3.input").expect("can't read file 3");
    println!("day 3 challenges :");
    println!("\tchall 1: {}", day3::chall_1(&input_3));
    println!("\tchall 2: {}", day3::chall_2(&input_3));

    let input_4 = fs::read_to_string("input/day4.input").expect("can't read file 4");
    println!("day 4 challenges :");
    println!("\tchall 1: {}", day4::chall_1(&input_4));
    println!("\tchall 2: {}", day4::chall_2(&input_4));

    let input_5 = fs::read_to_string("input/day5.input").expect("can't read file 5");
    println!("day 5 challenges :");
    println!("\tchall 1: {}", day5::chall_1(&input_5));
    println!("\tchall 2: {}", day5::chall_2(&input_5));

    let input_6 = fs::read_to_string("input/day6.input").expect("can't read file 6");
    println!("day 6 challenges :");
    println!("\tchall 1: {}", day6::chall_1(&input_6));
    println!("\tchall 2: {}", day6::chall_2(&input_6));

    let input_7 = fs::read_to_string("input/day7.input").expect("can't read file 7");
    println!("day 7 challenges :");
    println!("\tchall 1: {}", day7::chall_1(&input_7));
    println!("\tchall 2: {}", day7::chall_2(&input_7));

    let input_8 = fs::read_to_string("input/day8.input").expect("can't read file 8");
    println!("day 8 challenges :");
    println!("\tchall 1: {}", day8::chall_1(&input_8));
    println!("\tchall 2: {}", day8::chall_2(&input_8));

    let input_9 = fs::read_to_string("input/day9.input").expect("can't read file 9");
    println!("day 9 challenges :");
    println!("\tchall 1: {}", day9::chall_1(&input_9));
    println!("\tchall 2: {}", day9::chall_2(&input_9));

    let input_10 = fs::read_to_string("input/day10.input").expect("can't read file 10");
    println!("day 10 challenges :");
    println!("\tchall 1: {}", day10::chall_1(&input_10));
    println!("\tchall 2: ");
    day10::chall_2(&input_10);

    let input_11 = fs::read_to_string("input/day11.input").expect("can't read file 11");
    println!("day 11 challenges :");
    println!("\tchall 1: {}", day11::chall_1(&input_11));
    println!("\tchall 2: {}", day11::chall_2(&input_11));

}
