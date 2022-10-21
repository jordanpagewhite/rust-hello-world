/// By considering the terms in the Fibonacci sequence whose values do not
/// exceed `limit`, find the sum of the even-valued terms.
///
/// # Arguments
///
/// * `limit` - Sum even-valued Fibonacci values up until this limit.
///
/// # Examples
///
/// ```
/// use utils::problem2::problem2;
///
/// mod utils;
///
/// fn main() {
///   problem2(4_000_000);
/// }
/// ```
pub fn problem2(limit: i32) -> i32 {
    let mut n = 1;
    let mut m = 2;
    let mut sum = 0;

    while m < limit {
        if m % 2 == 0 {
            sum += m;
        }
        let temp = n;
        n = m;
        m += temp;
    }

    sum
}
