use std::fs;
use std::time;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {

    let input_1 = fs::read_to_string("input/day1").expect("can't read file 1");
    println!("day 1 challenges :");
    println!("\tchall 1: {}", day1::chall_1(&input_1));
    println!("\tchall 2: {}", day1::chall_2(&input_1));

    let input_2 = fs::read_to_string("input/day2").expect("can't read file 2");
    println!("day 2 challenges :");
    println!("\tchall 1: {}", day2::chall_1(&input_2));
    println!("\tchall 2: {}", day2::chall_2(&input_2));

    let input_3 = fs::read_to_string("input/day3").expect("can't read file 3");
    println!("day 3 challenges :");
    println!("\tchall 1: {}", day3::chall_1(&input_3));
    println!("\tchall 2: {}", day3::chall_2(&input_3));

    let input_4 = fs::read_to_string("input/day4").expect("can't read file 4");
    println!("day 4 challenges :");
    println!("\tchall 1: {}", day4::chall_1(&input_4));
    println!("\tchall 2: {}", day4::chall_2(&input_4));

    let input_5 = fs::read_to_string("input/day5").expect("can't read file 5");
    println!("day 5 challenges :");
    println!("\tchall 1: {}", day5::chall_1(&input_5));
    println!("\tchall 2: {}", day5::chall_2(&input_5));

    let input_6 = fs::read_to_string("input/day6").expect("can't read file 5");
    println!("day 6 challenges :");
    println!("\tchall 1: {}", day6::chall_1(&input_6));
    let now = time::Instant::now();
    println!("\tchall 2: {}", day6::chall_2(&input_6));
    dbg!(now.elapsed());
}
