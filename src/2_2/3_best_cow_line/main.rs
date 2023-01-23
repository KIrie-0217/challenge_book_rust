use proconio::input;

fn main() {
    input! {
        n:usize,
        s:String
    }

    let s_vec: Vec<char> = s.as_str().chars().collect();
    let mut ans: Vec<char> = Vec::<char>::new();
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    while left <= right {
        let mut is_left: bool = false;
        let mut iter_count: usize = 0;
        while left + iter_count <= right {
            if s_vec[left + iter_count] < s_vec[right - iter_count] {
                is_left = true;
                break;
            } else if s_vec[left + iter_count] > s_vec[right - iter_count] {
                is_left = false;
                break;
            }
            iter_count += 1;
        }

        if is_left {
            ans.push(s_vec[left]);
            left += 1;
        } else {
            ans.push(s_vec[right]);
            right -= 1;
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
