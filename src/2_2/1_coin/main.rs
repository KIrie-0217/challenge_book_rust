use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        mut target:usize,
        coins:[usize;6]
    }
    let coins_val: Vec<usize> = vec![1, 5, 10, 50, 100, 500];

    let mut ans: usize = 0;
    for (i, coin) in coins.iter().enumerate().rev() {
        let num_coin = min(target / coins_val[i], coin.clone());
        target -= coins_val[i] * num_coin;
        ans += num_coin;
    }

    println!("{}", ans);
}
