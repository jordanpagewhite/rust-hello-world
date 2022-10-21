/// Find the largest prime factor of `n`.
///
/// # Arguments
///
/// * `n` - A positive integer.
///
/// # Examples
///
/// ```
/// use utils::problem3::problem3;
///
/// mod utils;
///
/// fn main() {
///   problem3(&mut 600851475143);
/// }
/// ```
pub fn problem3(n: &mut i64) {
    let mut factors: Vec<i64> = Vec::new();
    let mut d = 2;

    while *n > 1 {
        while *n % d == 0 {
            factors.push(d);
            *n /= d;
        }
        d += 1;
        if d * d > *n {
            if *n > 1 {
                factors.push(*n)
            }
            break;
        }
    }

    println!("Factors: {:?}", factors);
    println!("Answer: {:?}", factors.pop().unwrap());
}
