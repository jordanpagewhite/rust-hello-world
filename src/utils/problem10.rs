use crate::utils::is_prime;

/// Finds the sum of all primes below `upper_bound`.
///
/// # Arguments
///
/// * `upper_bound` - A positive integer, strict upper bound.
///
/// # Examples
///
/// ```
/// use utils::problem10::problem10;
///
/// mod utils;
///
/// fn main() {
///   problem10(10); // 2 + 3 + 5 + 7 = 17.
///   problem10(100); // 1060.
/// }
/// ```
pub fn problem10(upper_bound: i64) {
    let mut sum = 0;

    for i in 2..=upper_bound {
        if is_prime(i) {
            sum += i;
        }
    }

    println!("Sum: {}", sum);
}
