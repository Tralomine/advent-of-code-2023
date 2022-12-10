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

    let input_6 = fs::read_to_string("input/day6").expect("can't read file 6");
    println!("day 6 challenges :");
    println!("\tchall 1: {}", day6::chall_1(&input_6));
    println!("\tchall 2: {}", day6::chall_2(&input_6));

    let input_7 = fs::read_to_string("input/day7").expect("can't read file 7");
    println!("day 7 challenges :");
    println!("\tchall 1: {}", day7::chall_1(&input_7));
    println!("\tchall 2: {}", day7::chall_2(&input_7));

    let input_8 = fs::read_to_string("input/day8").expect("can't read file 8");
    println!("day 8 challenges :");
    println!("\tchall 1: {}", day8::chall_1(&input_8));
    println!("\tchall 2: {}", day8::chall_2(&input_8));

    let input_9 = fs::read_to_string("input/day9").expect("can't read file 9");
    println!("day 9 challenges :");
    println!("\tchall 1: {}", day9::chall_1(&input_9));
    println!("\tchall 2: {}", day9::chall_2(&input_9));

    let input_10 = fs::read_to_string("input/day10").expect("can't read file 10");
    println!("day 10 challenges :");
    println!("\tchall 1: {}", day10::chall_1(&input_10));
    println!("\tchall 2: ");
    day10::chall_2(&input_10);

}
