use crate::utils::{get_data_from_file, get_file_from_path, transform_char_data};

const FILEPATH: &str = "/data/problem8.txt";

/// Finds the `n` adjacent digits with the greatest product. Returns the product.
///
/// # Examples
///
/// ```
/// use utils::problem8::problem8;
///
/// mod utils;
///
/// fn main() {
///   problem8(4); // 9 * 9 * 8 * 9 = 5832.
/// }
/// ```
pub fn problem8(n: i32) {
    let filepath = String::from(FILEPATH);
    let mut file = get_file_from_path(filepath);
    let data_str = get_data_from_file(&mut file);
    let data_vec = transform_char_data(data_str);
    let mut largest_product: i64 = 0;
    let mut index: usize = 0;

    while (index + n as usize) < data_vec.len() {
        let product = data_vec[index..index + n as usize].iter().product();

        if product > largest_product {
            largest_product = product;
        }

        index += 1;
    }

    println!("Answer (product): {}", largest_product);
}
