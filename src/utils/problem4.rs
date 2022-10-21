use crate::utils::reverse_int;

/// Find the largest palindrome made from the product of two 3-digit numbers
///
/// # Examples
///
/// ```
/// use utils::problem4::problem4;
///
/// mod utils;
///
/// fn main() {
///   problem4();
/// }
/// ```
pub fn problem4() {
    let mut palindrome: i32 = 0;

    for m in (100..1000).rev() {
        for n in (100..1000).rev() {
            if m * n == reverse_int(m * n) && m * n > palindrome {
                palindrome = m * n;
            }
        }
    }

    println!("Answer: {}", palindrome);
}
