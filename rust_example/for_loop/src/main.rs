use std::time::Instant;
use std::io;

fn main() {
    // Prompting the user to enter the size of the 2D array
    println!("Enter the size for the 2D array:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = input.trim().parse().expect("Please type a number!");

    // Creating a 2D vector filled with zeroes based on user input
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; size]; size];

    // filling the matrix with sample data
    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = (i*j) as i32;
        }
    }

    // Iterating row-major order
    let start_row_major = Instant::now();
    for i in 0..size {
        for j in 0..size {
            let _ = matrix[i][j];
        }
    }
    let duration_row_major = start_row_major.elapsed();

    // Iterating column-major order
    let start_col_major = Instant::now();
    for i in 0..size {
        for j in 0..size {
            let _ = matrix[j][i];
        }
    }
    let duration_col_major = start_col_major.elapsed();

    println!("Time taken for row-major order: {:?}", duration_row_major);
    println!("Time taken for column-major order: {:?}", duration_col_major);
    

}
