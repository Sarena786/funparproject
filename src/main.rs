use rand::Rng;
use std::time::Instant;

mod normal_mult;
mod cache_optimized;
mod simd;
mod pure_thread;

pub type Matrix = Vec<Vec<i32>>;

fn generate_random_matrix(n: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(0..100);
        }
    }
    matrix
}

fn main() {
    
    let n = 2000; // Size of the matrix
    let a = generate_random_matrix(n);
    let b = generate_random_matrix(n);

    // Normal multiplication
    let start = Instant::now();
    let _ = normal_mult::matrix(&a, &b);
    let duration = start.elapsed();
    println!("Normal multiplication took: {:?}", duration);

    // Cache-optimized multiplication
    let block_size = 64;
    let start = Instant::now();
    let _ = cache_optimized::matrix(&a, &b, block_size);
    let duration = start.elapsed();
    println!("Cache-optimized multiplication took: {:?}", duration);

    // Threaded multiplication
    let start = Instant::now();
    let _ = pure_thread::matrix(&a, &b);
    let duration = start.elapsed();
    println!("Pure threads took: {:?}", duration);

    // SIMD multiplication
    let start = Instant::now();
    let _ = simd::matrix(&a, &b);
    let duration = start.elapsed();
    println!("SIMD multiplication took: {:?}", duration);
}
