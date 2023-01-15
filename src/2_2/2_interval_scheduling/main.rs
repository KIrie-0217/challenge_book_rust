use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[usize;n],
        t:[usize;n]
    }

    let mut task_pair: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
    for i in 0..n {
        task_pair.push(vec![s[i], t[i]]);
    }

    task_pair.sort_by(|a, b| b[1].cmp(&(a[1])));

    let mut latest_end: usize = 0;
    let mut ans: usize = 0;
    for _ in 0..n {
        let tmp = task_pair.pop().unwrap();
        if latest_end < tmp[0] {
            ans += 1;
            latest_end = tmp[1].clone();
        }
    }

    println!("{}", ans);
}
