use proconio::input;

fn dfs(target: &mut Vec<Vec<bool>>, row: usize, col: usize, row_size: usize, col_size: usize) {
    target[row][col] = false;
    for i in -1..=1 {
        for j in -1..=1 {
            let next_row: i32 = i + row as i32;
            let next_col: i32 = j + col as i32;
            if 0 <= next_row
                && next_row < row_size as i32
                && 0 <= next_col
                && next_col < col_size as i32
            {
                if target[next_row as usize][next_col as usize] {
                    dfs(
                        target,
                        next_row as usize,
                        next_col as usize,
                        row_size,
                        col_size,
                    );
                }
            }
        }
    }
}

fn main() {
    input! {
        row_size:usize,
        col_size:usize,
        garden:[String;row_size]
    }

    let mut garden_bool: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    for r in 0..row_size {
        garden_bool.push(vec![]);
        for c in 0..col_size {
            if garden[r].chars().nth(c).unwrap() == '.' {
                garden_bool[r].push(false);
            } else {
                garden_bool[r].push(true);
            };
        }
    }

    let mut res: usize = 0;
    for r in 0..row_size {
        for c in 0..col_size {
            if garden_bool[r][c] {
                dfs(&mut garden_bool, r, c, row_size, col_size);
                res += 1;
            }
        }
    }
    println!("{}", res);
}
