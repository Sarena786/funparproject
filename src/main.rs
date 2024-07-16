mod cache_optimized;
mod normal_mult;
mod pure_thread;
mod simd;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <method>", args[0]);
        eprintln!("Methods: normal_mult, pure_thread, cache_optimized, simd");
        return;
    }

    let method = &args[1];
    let a: normal_mult :: Matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    let b: normal_mult :: Matrix = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    match method.as_str() {
        "normal_mult" => {
            match normal_mult :: matrix(&a, &b) {
                Some(result) => {
                    for row in result {
                        println!("{:?}", row);
                    }
                }
                None => println!("Error in matrix dimensions"),
            }
        }
        "pure_thread" => {
            match pure_thread :: matrix(&a, &b) {
                Some(result) => {
                    for row in result {
                        println!("{:?}", row);
                    }
                }
                None => println!("Error in matrix dimensions"),
            }
        }
        // "cache_optimized" => {
        //     let block_size = 2;
        //     match cache_optimized :: matrix(&a, &b, block_size) {
        //         Some(result) => {
        //             for row in result {
        //                 println!("{:?}", row);
        //             }
        //         }
        //         None => println!("Error in matrix dimensions"),
        //     }
        // }
        // "simd" => {
        //     match simd :: matrix(&a, &b) {
        //         Some(result) => {
        //             for row in result {
        //                 println!("{:?}", row);
        //             }
        //         }
        //         None => println!("Error in matrix dimensions"),
        //     }
        // }
        _ => {
            eprintln!("Unknown method: {}", method);
            eprintln!("Methods: normal, threading, cache_optimized, simd");
        }
    }
}
