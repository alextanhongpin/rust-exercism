pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_rows: Vec<(usize, usize)> = vec![];
    for (i, rows) in input.iter().enumerate() {
        let mut max_i = 0;
        let mut max_row = u64::min_value();
        for (j, col) in rows.iter().enumerate() {
           if *col > max_row {
               max_i = j;
               max_row = *col;
           }
        }
        max_rows.push((i, max_i));
        for (j, col) in rows.iter().enumerate() {
            if j != max_i && *col == max_row {
                max_rows.push((i, j));
            }
        }
    }

    let col_len = input[0].len();
    let row_len = input.len(); 
    let mut solution: Vec<(usize, usize)> = vec![];
    for i in 0..col_len {
        let mut min_i = 0;
        let mut min_col = u64::max_value(); 
        for j in 0..row_len {
            if input[j][i] < min_col {
                min_col = input[j][i];
                min_i = j;
            }
        }
        solution.extend(max_rows.iter().filter(|(row, col)| *row == min_i && *col == i));
        for j in 0..row_len {
            if input[j][i] == min_col && min_i != j {
               solution.extend(max_rows.iter().filter(|(row, col)| *row == j && *col == i));
            }
        }
    }
    solution
}

