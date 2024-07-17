pub type Matrix = Vec<Vec<i32>>;

pub fn matrix(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    let a_row = a.len();
    let b_col = b[0].len();
    let a_col = a[0].len();
    let mut result = vec![vec![0; b_col]; a_row];

    if a_col != b.len() {
        return None;
    }

    for i in 0..a_row {
        for j in 0..b_col {
            result[i][j] = (0..a_col).map(|k| a[i][k] * b[k][j]).sum();
            //Computes the dot product of the i-th row of a and the j-th column of b.
            //Uses a concise way to perform the dot product using map and sum.
            //This effectively hides one level of iteration but still does the same amount of work.

            //Matrix b is accessed in column-major order, which can lead to cache misses. 
            //When k varies quickly, b[k][j] accesses different rows of b, which are far apart in memory.
        }
    }
    Some(result)
}
