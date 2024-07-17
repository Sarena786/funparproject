use super :: normal_mult :: Matrix;
//The idea behind cache optimization is to improve the performance by minimizing cache misses.
/*
In traditional matrix multiplication, accessing matrix elements often results in poor cache utilization 
due to the way memory is accessed in a row-major or column-major order. 
By processing the matrices in smaller sub-blocks,
we can significantly improve cache usage.

*Spatial Locality: By processing smaller blocks of the matrix, the code accesses elements that are close together in memory. 
This improves spatial locality, meaning that once a block of data is loaded into the cache, it is used repeatedly before being evicted

*Temporal Locality: The cache-optimized approach reuses the same small blocks of data multiple times within the cache before moving on to the next block. 
This improves temporal locality, reducing the need to reload the same data from slower main memory multiple times.
*/
pub fn matrix(a: &Matrix, b: &Matrix, block_size: usize) -> Option<Matrix> {
    let a_row = a.len();
    let b_col = b[0].len();
    let a_col = a[0].len();
    let mut result = vec![vec![0; b_col]; a_row];
    //the resulting size of matrix multi is a's row, b's col

    if a_col != b.len() {
        //the condi for multi together
        return None;
    }

    for ii in (0..a_row).step_by(block_size) {
        for jj in (0..b_col).step_by(block_size) {  
            for kk in (0..a_col).step_by(block_size) {
                for i in ii..(ii + block_size).min(a_row) {
                    for j in jj..(jj + block_size).min(b_col) {
                        for k in kk..(kk + block_size).min(a_col) {
                            result[i][j] += a[i][k] * b[k][j];
                        }
                    }
                }
            }
        }
    }

    Some(result)
}

