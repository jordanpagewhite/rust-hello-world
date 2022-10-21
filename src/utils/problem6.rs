use crate::utils::{square_of_sums, sum_of_squares};

/// Finds the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
///
/// # Examples
///
/// ```
/// use utils::problem6::problem6;
///
/// mod utils;
///
/// fn main() {
///   problem6();
/// }
/// ```
pub fn problem6() {
    let n = 100;
    let answer = square_of_sums(n) - sum_of_squares(n);
    println!("Answer: {}", answer);
}
