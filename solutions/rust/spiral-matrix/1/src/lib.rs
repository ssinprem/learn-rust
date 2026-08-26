pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    if size == 0 {
        return vec![];
    }

    let mut cur = (0_isize, 0_isize);
    matrix[0][0] = 1;

    let mut d = 0;
    let dir: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut num = 2;
    while matrix.iter().any(|r| r.iter().any(|n| n == &0)) {
        let target = (cur.0 + dir[d].0, cur.1 + dir[d].1);
        if target.0 < 0 || target.0 >= size as isize || target.1 < 0 || target.1 >= size as isize {
            d = (d + 1) % 4;
            continue;
        }

        let val_target = matrix[target.0 as usize][target.1 as usize];
        if val_target != 0 {
            d = (d + 1) % 4;
            continue;
        }
        cur = (target.0, target.1);
        matrix[target.0 as usize][target.1 as usize] = num;
        num += 1;
    }
    matrix
}
