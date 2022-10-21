/// Finds the sum of all the multiples of `divisors` below `limit`.
///
/// Ex. if limit == 10 and divisors == (2,4), then 2+4+6+8=20
///
/// # Arguments
///
/// * `limit` - A positive integer to use as an upper bound for finding multiples.
/// * `divisors` - Positive integers to find multiples of up until `limit`.
///
/// # Examples
///
/// ```
/// use utils::problem1::problem1;
///
/// mod utils;
///
/// fn main() {
///   problem1(10, (3, 5));
/// }
/// ```
pub fn problem1(limit: i32, divisors: (i32, i32)) -> i32 {
    let mut sum: i32 = 0;

    for x in 1..limit {
        if x % divisors.0 == 0 || x % divisors.1 == 0 {
            sum += x;
        }
    }

    sum
}
