use proconio::input;
use std::cmp;

fn main() {
    input! {
        l:usize,n:usize,
        mut ants:[ usize ; n]
    }

    ants.sort_by(|a, b| a.cmp(b));

    let mut min_time: usize = 0;
    let mut max_time: usize = 0;

    for ant in ants {
        min_time = cmp::max(min_time, cmp::min(ant, l - ant));
        max_time = cmp::max(max_time, cmp::max(ant, l - ant));
    }

    println!("{} {}", min_time, max_time);
}
