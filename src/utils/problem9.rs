/// Finds a Pythagorean triple whose sum is `sum`. Returns the product of this triple.
///
/// # Arguments
///
/// * `sum` - A sum of Pythagorean triple values.
///
/// # Examples
///
/// ```
/// use utils::problem9::problem9;
///
/// mod utils;
///
/// fn main() {
///   problem9(1000);
/// }
/// ```
pub fn problem9(sum: i32) {
    for n in 1..100 {
        for m in 1..100 {
            if m > n {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;

                if a + b + c == sum {
                    println!(
                        "a: {}, b: {}, c: {}, sum: {}, product: {}",
                        a,
                        b,
                        c,
                        a + b + c,
                        a * b * c
                    );
                    return;
                }
            }
        }
    }
}
