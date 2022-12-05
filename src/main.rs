use std::fs;

pub mod day1;

fn main() {
    //read day 1 input
    let input_1 = fs::read_to_string("input/day1")
        .expect("can't read file 1");
    println!("day 1 challenges :");
    //bug, needs to add 2 blank lines at the end of the file
    //execute day 1 challenge 1
    println!("chall 1: {}", crate::day1::chall_1(&input_1));
    //execute day 1 challenge 2
    println!("chall 2: {}", crate::day1::chall_2(&input_1));

}
