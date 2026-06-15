pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut set = Vec::new();
    let rows = input.len();
    let cols = input[0].len();
    for row in 0..rows  {
        for col in 0..cols {
            let curr = input[row][col];
            let c_col = input.iter()
                .map(|row| *row.get(col).unwrap())
                .collect::<Vec<u64>>();
            let c_row = input[row].clone();
            if  curr == *c_row.iter().max().unwrap() &&
                curr == *c_col.iter().min().unwrap() {
                set.push((row,col));
            }
        }
    }
    set
}
