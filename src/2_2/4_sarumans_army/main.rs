use proconio::input;

fn main() {
    input! {
        n:usize,
        r:usize,
        x:[usize;n]
    }

    let mut x_vec: Vec<usize> = Vec::<usize>::from(x);

    x_vec.sort_by(|a, b| a.cmp(&b));

    let mut ans: usize = 0;
    let mut iter_num: usize = 0;
    while iter_num < n {
        let start = x_vec[iter_num];
        iter_num += 1;

        while iter_num < n && start + r < x_vec[iter_num] {
            iter_num += 1;
        }

        let new_point: usize = x_vec[iter_num - 1];
        while iter_num < n && x_vec[iter_num] <= new_point + r {
            iter_num += 1;
        }
        ans += 1;
    }
    println!("{}", ans);
}
