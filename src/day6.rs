// use std::collections::HashMap;

//O(n²), *bad*
fn has_duplicates(s : &[u8]) -> bool {
    for i in 0..s.len() {
        for j in i+1..s.len() {
            if s[i] == s[j] {
                return true;
            }
        }
    }
    return false;
}

//O(n), but hashmap by default use a safe but slow af hash function
// fn has_duplicates(s : &[u8]) -> bool {
//     let mut map = HashMap::new();
//     for c in s {
//         if !map.get(c).copied().unwrap_or(false) {
//             map.insert(c, true);
//         } else {
//             return true;
//         }
//     }
//     return false;
// }

pub fn chall_1(s : &String) -> usize {
    let s = s.as_bytes();
    for i in 0..s.len()-4 {
        if !has_duplicates(&s[i..i+4]) {
            return i+4;
        }
    }
    0
}

pub fn chall_2(s : &String) -> usize {
    let s = s.as_bytes();
    for i in 0..s.len()-14 {
        if !has_duplicates(&s[i..i+14]) {
            return i+14;
        }
    }
    0
}