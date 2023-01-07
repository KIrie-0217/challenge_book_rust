use proconio::input;

fn bin_search(target: usize, check_arr: &[usize]) -> usize {
    let mut right: i32 = check_arr.len() as i32;
    let mut left: i32 = -1;
    let mut mid: i32 = (right + left) / 2;

    while right - left > 1 {
        if check_arr[mid as usize] > target {
            right = mid;
        } else {
            left = mid;
        }
        mid = (right + left) / 2;
    }

    mid as usize
}

fn main() {
    input! {
        n:usize,
        mut m:[usize;n]
    }

    m.sort_by(|a, b| a.cmp(b));

    let mut ans: usize = 0;
    for i in 0..n - 1 {
        let bi = m[i] + m[i + 1];
        let ai_index = bin_search(bi, &m);
        if i + 2 <= ai_index {
            ans = m[i] + m[i + 1] + m[ai_index];
        }
    }

    println!("{}", ans);
}
