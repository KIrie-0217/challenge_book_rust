use proconio::input;

fn main() {
    input! {
        mut n:usize,
        l:[usize;n]
    }

    let mut l_vec: Vec<usize> = Vec::<usize>::from(l);
    l_vec.sort_by(|a, b| a.cmp(&b));

    let mut ans: usize = 0;
    while n > 1 {
        let mut min_1: usize = 0;
        let mut min_2: usize = 1;
        if l_vec[min_1] > l_vec[min_2] {
            let tmp = min_1;
            min_1 = min_2;
            min_2 = tmp;
        }

        for i in 2..n {
            if l_vec[i] < l_vec[min_1] {
                min_2 = min_1;
                min_1 = i;
            } else if l_vec[i] < l_vec[min_2] {
                min_2 = i;
            }
        }

        let t = l_vec[min_1] + l_vec[min_2];
        ans += t;

        if min_1 == n - 1 {
            let tmp = min_1;
            min_1 = min_2;
            min_2 = tmp;
        }

        l_vec[min_1] = t;
        l_vec[min_2] = l_vec[n - 1];
        n -= 1;
    }

    println!("{}", ans);
}
