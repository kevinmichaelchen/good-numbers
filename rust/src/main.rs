extern crate rayon;
use rayon::prelude::*;
use std::time::{SystemTime};

#[inline]
fn is_good_number(cur_num: &u32) -> bool {
    let mut sum = 0;
    let mut tmp = *cur_num;
    while tmp > 0 {
        let n = tmp;
        tmp /= 10;
        sum += n - tmp * 10;
    }
    (*cur_num % sum) == 0
}

fn parallel_iter() {
    let upper_bound: u32 = 1_000_000_000;
    let start = SystemTime::now();
    println!(
        "There are {:?} good numbers",
        (1..upper_bound + 1)
            .into_par_iter()
            .filter(is_good_number)
            .count()
    );
    let done = SystemTime::now();
    println!("Program duration: {:?}",
             done.duration_since(start).expect("Time went backwards"));
}

fn sequential_iter() {
    let upper_bound: u32 = 1_000_000_000;
    let start = SystemTime::now();
    println!(
        "There are {:?} good numbers",
        (1..upper_bound + 1)
            .into_iter()
            .filter(is_good_number)
            .count()
    );
    let done = SystemTime::now();
    println!("Program duration: {:?}",
             done.duration_since(start).expect("Time went backwards"));
}

fn sequential_for_loop() {
    let upper_bound: u32 = 1_000_000_000;
    let start = SystemTime::now();
    let mut arr : Vec<u32> = vec![];

    for i in 1..=upper_bound {
        if is_good_number(&i) {
            arr.push(i)
        }
    }

    println!(
        "There are {:?} good numbers",
        arr.len()
    );
    let done = SystemTime::now();
    println!("Program duration: {:?}",
             done.duration_since(start).expect("Time went backwards"));
}


fn main() {
//    parallel_iter();
    sequential_iter();
    sequential_for_loop();
}
