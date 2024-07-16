use std :: thread;
use super :: normal_mult :: Matrix;

/*
this method make use of central idea from normal_mult method, which is row * line
and the concurrent part takes place when each dot product happens
which means once we check the condition for whether they are valid matrices to be multiplied together
we can do dot product at a time and get the result immediately
*/
pub fn matrix(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    let a_row = a.len();
    let b_col = b[0].len();
    let a_col = a[0].len();
    //create same var as src/normal_mult does, use this to store result from each dot product
    let mut result = vec![vec![0; b_col]; a_row];

    if a_col != b.len() {
        //check whether it satisfify the condition for matrices to multiplied together
        return None;
    }

    //total collection of all threads
    //later will call handle.join() to wait for all threads to complete
    let mut handles = vec![];
    
    for i in 0..a_row {
        //to prevent from changing ori matrices, we need to clone
        let a = a.clone();
        let b = b.clone();

        let mut result_row = result[i].clone();
        let handle = thread :: spawn(move || {
            //thread :: spawn create a thread to do the thing inside of brakets
            for j in 0..b_col {
                result_row[j] = (0..a_col).map(|k| a[i][k] * b[k][j]).sum();
            }
            result_row
        });

        //push the thread into the total collections
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        //.into_iter() will transfer the ownership compared to .iter(),
        // which generates the referens to each elt of one collection
        result[i] = handle.join().unwrap();
    }

    Some(result)
}
