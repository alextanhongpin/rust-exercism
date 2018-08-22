pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // PSEUDO-CODE
    // Find the maximum values from each rows
    // Save the positions of the maximum values for each rows
    // Find the minimum positions from each columns
    // Save the positions of the minimum values for each columns
    // Check the intersection between the columns and rows
    let mut max_rows: Vec<(usize, usize)> = vec![];
    for (i, rows) in input.iter().enumerate() {
        let max = rows.iter().max();
        for (j, col) in rows.iter().enumerate() {
           match max {
               Some(max) if col >= max => max_rows.push((i, j)),
               Some(_) => (),
               None => () 
           }
        }
    }

    let col_len = input[0].len();
    let row_len = input.len(); 
    let mut solution: Vec<(usize, usize)> = vec![];
    for i in 0..col_len {
        let min = (0..row_len).map(|j| input[j][i]).min().unwrap();
        for j in 0..row_len {
            if input[j][i] <= min {
                solution.extend(max_rows.iter().filter(|(row, col)| *row == j && *col == i));
            }
        }
    }
    solution
}

