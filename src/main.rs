use std::fs;

pub mod day1;
pub mod day2;

fn main() {
    //read day 1 input
    let input_1 = fs::read_to_string("input/day1")
        .expect("can't read file 1");
    println!("day 1 challenges :");
    //bug, needs to add 2 blank lines at the end of the file
    //execute day 1 challenge 1
    println!("\tchall 1: {}", crate::day1::chall_1(&input_1));
    //execute day 1 challenge 2
    println!("\tchall 2: {}", crate::day1::chall_2(&input_1));

    let input_2 = fs::read_to_string("input/day2").expect("can't read file 2");
    println!("day 2 challenges :");
    println!("\tchall 1: {}", crate::day2::chall_1(&input_2));
    println!("\tchall 2: {}", crate::day2::chall_2(&input_2));
}
