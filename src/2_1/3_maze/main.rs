use proconio::input;
use std::collections::VecDeque;

static LIMIT: usize = 1_000_000;

fn bfs(
    target: Vec<Vec<bool>>,
    target_length: &mut Vec<Vec<usize>>,
    start_x: usize,
    start_y: usize,
    goal_x: usize,
    goal_y: usize,
    row_size: usize,
    col_size: usize,
) -> usize {
    let mut queue_vec: VecDeque<Vec<usize>> = VecDeque::<Vec<usize>>::new();
    let dx: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0], vec![-1, 0], vec![0, -1]];

    queue_vec.push_front(vec![start_x, start_y]);
    target_length[start_y][start_x] = 0;
    while queue_vec.len() != 0 {
        let tmp: Vec<usize> = queue_vec.pop_front().unwrap();
        if tmp[0] == goal_x && tmp[1] == goal_y {
            break;
        }

        for d in &dx {
            let next_x = tmp[0] as i32 + d[0];
            let next_y = tmp[1] as i32 + d[1];

            if 0 <= next_x && next_x < col_size as i32 && 0 <= next_y && next_y < row_size as i32 {
                if target[next_y as usize][next_x as usize]
                    && target_length[next_y as usize][next_x as usize] == LIMIT
                {
                    target_length[next_y as usize][next_x as usize] =
                        target_length[tmp[1]][tmp[0]] + 1;

                    queue_vec.push_back(vec![next_x as usize, next_y as usize]);
                }
            }
        }
    }

    target_length[goal_y][goal_x]
}

fn main() {
    input! {
        row_size:usize , col_size:usize,
        start_y :usize , start_x:usize,
        goal_y  :usize , goal_x :usize,
        maze: [ String ; row_size]
    }

    let mut maze_bool: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    let mut maze_length: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
    for maze_row in 0..row_size {
        maze_bool.push(vec![]);
        maze_length.push(vec![]);
        for maze_col in 0..col_size {
            if maze[maze_row].chars().nth(maze_col).unwrap() == '#' {
                maze_bool[maze_row].push(false);
            } else {
                maze_bool[maze_row].push(true);
            }
            maze_length[maze_row].push(LIMIT);
        }
    }

    let res = bfs(
        maze_bool,
        &mut maze_length,
        start_x - 1,
        start_y - 1,
        goal_x - 1,
        goal_y - 1,
        row_size,
        col_size,
    );

    println!("{}", res);
}
