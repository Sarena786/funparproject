use std::arch::x86_64::*;
use std::alloc::{alloc, dealloc, Layout};

pub type Matrix = Vec<Vec<i32>>;

pub fn matrix(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_cols = b[0].len();

    if a_cols != b.len() {
        return None; // Matrix dimensions mismatch
    }

    let mut result = vec![vec![0; b_cols]; a_rows];

    unsafe {
        // Allocate memory for matrix_a and matrix_b
        let layout = Layout::from_size_align(a_rows * a_cols * std::mem::size_of::<i32>(), std::mem::align_of::<i32>()).unwrap();
        let matrix_a = alloc(layout) as *mut i32;
        let matrix_b = alloc(layout) as *mut i32;

        // Initialize matrix_a and transpose matrix_b
        for i in 0..a_rows {
            for j in 0..a_cols {
                *matrix_a.add(i * a_cols + j) = a[i][j];
                *matrix_b.add(j * b_cols + i) = b[i][j]; // Transpose matrix b for better memory access
            }
        }

        // Perform matrix multiplication with SIMD
        for i in 0..a_rows {
            for j in 0..b_cols {
                let mut sum = _mm_setzero_si128();
                for k in (0..a_cols).step_by(4) {
                    let va = _mm_loadu_si128(matrix_a.add(i * a_cols + k) as *const __m128i);
                    let vb = _mm_loadu_si128(matrix_b.add(j * a_cols + k) as *const __m128i);

                    let product = _mm_mullo_epi32(va, vb);
                    sum = _mm_add_epi32(sum, product);
                }

                // Horizontally add the elements of sum
                let mut temp = _mm_extract_epi32(sum, 0) + _mm_extract_epi32(sum, 1) + _mm_extract_epi32(sum, 2) + _mm_extract_epi32(sum, 3);
                temp = temp + (temp << 16);
                temp = temp + (temp << 8);
                result[i][j] = temp >> 24;
            }
        }

        // Deallocate memory
        dealloc(matrix_a as *mut u8, layout);
        dealloc(matrix_b as *mut u8, layout);
    }

    Some(result)
}
