/// Finds the smallest positive number that is evenly divisible by all of the numbers from 1 to 20.
///
/// # Examples
///
/// ```
/// use utils::problem5::problem5;
///
/// mod utils;
///
/// fn main() {
///   problem5();
/// }
/// ```
pub fn problem5() {
    let mut n = 20;

    loop {
        for i in 2..21 {
            if n % i != 0 {
                n += 2;
                break;
            } else if i == 20 {
                println!("Answer: {}", n);
                return;
            }
        }
    }
}
