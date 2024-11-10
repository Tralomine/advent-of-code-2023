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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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

    let input_12 = fs::read_to_string("input/day12.input").expect("can't read file 12");
    println!("day 12 challenges :");
    println!("\tchall 1: {}", day12::chall_1(&input_12));
    println!("\tchall 2: {}", day12::chall_2(&input_12));

    let input_13 = fs::read_to_string("input/day13.input").expect("can't read file 13");
    println!("day 13 challenges :");
    println!("\tchall 1: {}", day13::chall_1(&input_13));
    println!("\tchall 2: {}", day13::chall_2(&input_13));

    let input_14 = fs::read_to_string("input/day14.input").expect("can't read file 14");
    println!("day 14 challenges :");
    println!("\tchall 1: {}", day14::chall_1(&input_14));
    println!("\tchall 2: {}", day14::chall_2(&input_14));

    let input_15 = fs::read_to_string("input/day15.input").expect("can't read file 15");
    println!("day 15 challenges :");
    println!("\tchall 1: {}", day15::chall_1(&input_15));
    println!("\tchall 2: {}", day15::chall_2(&input_15));

    let input_16 = fs::read_to_string("input/day16.input").expect("can't read file 16");
    println!("day 16 challenges :");
    println!("\tchall 1: {}", day16::chall_1(&input_16));
    println!("\tchall 2: {}", day16::chall_2(&input_16));

    let input_17 = fs::read_to_string("input/day17.input").expect("can't read file 17");
    println!("day 17 challenges :");
    println!("\tchall 1: {}", day17::chall_1(&input_17));
    println!("\tchall 2: {}", day17::chall_2(&input_17));

    let input_18 = fs::read_to_string("input/day18.input").expect("can't read file 18");
    println!("day 18 challenges :");
    println!("\tchall 1: {}", day18::chall_1(&input_18));
    println!("\tchall 2: {}", day18::chall_2(&input_18));

    let input_19 = fs::read_to_string("input/day19.input").expect("can't read file 19");
    println!("day 19 challenges :");
    println!("\tchall 1: {}", day19::chall_1(&input_19));
    println!("\tchall 2: {}", day19::chall_2(&input_19));

    let input_20 = fs::read_to_string("input/day20.input").expect("can't read file 20");
    println!("day 20 challenges :");
    println!("\tchall 1: {}", day20::chall_1(&input_20));
    println!("\tchall 2: {}", day20::chall_2(&input_20));

    let input_21 = fs::read_to_string("input/day21.input").expect("can't read file 21");
    println!("day 21 challenges :");
    println!("\tchall 1: {}", day21::chall_1(&input_21));
    println!("\tchall 2: {}", day21::chall_2(&input_21));

    let input_22 = fs::read_to_string("input/day22.input").expect("can't read file 22");
    println!("day 22 challenges :");
    println!("\tchall 1: {}", day22::chall_1(&input_22));
    println!("\tchall 2: {}", day22::chall_2(&input_22));

    let input_23 = fs::read_to_string("input/day23.input").expect("can't read file 23");
    println!("day 23 challenges :");
    println!("\tchall 1: {}", day23::chall_1(&input_23));
    println!("\tchall 2: {}", day23::chall_2(&input_23));

    let input_24 = fs::read_to_string("input/day24.input").expect("can't read file 24");
    println!("day 24 challenges :");
    println!("\tchall 1: {}", day24::chall_1(&input_24));
    println!("\tchall 2: {}", day24::chall_2(&input_24));

    let input_25 = fs::read_to_string("input/day25.input").expect("can't read file 25");
    println!("day 25 challenges :");
    println!("\tchall 1: {}", day25::chall_1(&input_25));

}
