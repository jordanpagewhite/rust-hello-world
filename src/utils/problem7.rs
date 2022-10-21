use crate::utils::sieve;

/// Finds the 10001st prime number.
///
/// # Examples
///
/// ```
/// use utils::problem7::problem7;
///
/// mod utils;
///
/// fn main() {
///   problem7();
/// }
/// ```
pub fn problem7(n: i64) {
    let mut primes: Vec<i64> = Vec::new();

    for i in 2..=n {
        primes.push(i);
    }

    for i in 0..primes.len() {
        let factor = primes[i];
        if factor != 0 {
            sieve(&mut primes, factor);
        }
    }

    let mut count = 0;

    loop {
        if count >= primes.len() {
            break;
        }
        if primes[count] == 0 {
            primes.remove(count);
            count -= 1;
        }
        count += 1;
    }

    // 10000 index for the 10001th prime (starts at 0).
    println!("Answer: {}", primes[10000]);
}
