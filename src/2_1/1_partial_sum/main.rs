use proconio::input;

fn dfs(
    input_arr: &Vec<usize>,
    now_index: usize,
    count_limit: usize,
    sum_tmp: usize,
    target: usize,
) -> bool {
    if now_index == count_limit {
        let res: bool = if sum_tmp == target { true } else { false };
        return res;
    }

    if dfs(&input_arr, now_index + 1, count_limit, sum_tmp, target) {
        return true;
    }

    if dfs(
        &input_arr,
        now_index + 1,
        count_limit,
        sum_tmp + input_arr[now_index].clone(),
        target,
    ) {
        return true;
    }

    return false;
}

fn main() {
    input! {
        n:usize,
        a_vec:[usize;n],
        k:usize
    }

    if dfs(&a_vec, 0, n, 0, k) {
        println!("Yes");
    } else {
        println!("No");
    }
}
