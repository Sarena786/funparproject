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
        }
    }
    Some(result)
}
