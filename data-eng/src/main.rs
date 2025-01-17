use rayon::prelude::*;

fn main() {
    println!("Hello, world!");
    println!("Sum of squares: {}", sum_of_squares(vec![1, 2, 3, 4, 5]));
}

fn sum_of_squares(input: Vec<i32>) -> i32 {
    input.par_iter().map(|&x| x * x).sum()
}
