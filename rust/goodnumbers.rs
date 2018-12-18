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

fn main() {
    let upper_bound: u32 = 1_000_000_000;
    println!(
        "{:?}",
        (1..upper_bound + 1)
            .into_par_iter()
            .filter(is_good_number)
            .count()
    );
}
