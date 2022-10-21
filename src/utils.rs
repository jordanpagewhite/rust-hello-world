pub mod problem1;
pub mod problem10;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem9;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Sieve of Eratosthenes implementation.
///
/// Borrowed from/heavily inspired by: <https://github.com/aisrael/sieve-of-eratosthenes/blob/master/sieve.rs>
pub fn sieve(primes: &mut Vec<i64>, factor: i64) {
    for i in 0..primes.len() {
        if primes[i] != 0 && primes[i] != factor && primes[i] % factor == 0 {
            primes[i] = 0;
        }
    }
}

/// Finds the square of sums 1..n.
///
/// # Examples
/// ```
/// square_of_sums(10) == (1+2+3+4+5+6+7+8+9+10)^2 == 3025
/// ```
pub fn square_of_sums(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..n + 1 {
        sum += i;
    }

    sum * sum
}

/// Finds the sum of squares of natural numbers up to n.
///
/// # Examples
/// ```
/// sum_of_squares(10) == 1^2+2^2+3^2+4^2+5^2+6^2+7^2+8^2+9^2+10^2 == 385
/// ```
pub fn sum_of_squares(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..n + 1 {
        sum += i.pow(2);
    }

    sum
}

/// Takes a positive integer and returns its palindrome number.
///
/// # Examples
///
/// ```
/// reverse_int(12345) == 54321
/// ```
pub fn reverse_int(i: i32) -> i32 {
    let str: String = format!("{:?}", i);
    let str_rev = str.chars().rev().collect::<String>();
    str_rev.parse().unwrap()
}

/// Finds the greatest common divisor of positive integers a and b.
///
/// # Examples
///
/// ```
/// gcd(8, 12) == 4
/// ```
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Loads and returns a File from its relative path.
///
/// # Arguments
///
/// * `relative_path` - (String) A relative path to the file.
pub fn get_file_from_path(relative_path: String) -> File {
    let basepath = env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR is not set");
    let filepath_str = format!("{}{}", basepath, relative_path);
    let filepath = Path::new(&filepath_str);
    File::open(filepath).expect("Can't open file!")
}

/// Reads and returns the contents of a file as a String.
///
/// # Arguments
///
/// * `file` - (&mut File) A file to read string data from.
pub fn get_data_from_file(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read to line.");
    contents
}

/// Transforms string data into a Vec<i32> where each line becomes an element.
///
/// # Arguments
///
/// * `data_str` - (String) A string to convert into a Vec<i32>.
pub fn transform_data(data_str: String) -> Vec<i32> {
    let mut vec_ints = Vec::<i32>::new();
    for line in data_str.lines() {
        vec_ints.push(line.parse::<i32>().unwrap());
    }

    vec_ints
}

/// Transforms string data into a Vec<i64> where each character becomes an element.
///
/// # Arguments
///
/// * `data_str` - (String) A string to convert into a Vec<i64>.
pub fn transform_char_data(data_str: String) -> Vec<i64> {
    let mut vec_ints = Vec::<i64>::new();
    for c in data_str.chars() {
        vec_ints.push(c.to_digit(10).unwrap() as i64);
    }

    vec_ints
}

/// Determines if a positive integer is prime.
///
/// # Arguments
///
/// * `n` - A positive integer.
///
/// # Examples
///
/// ```
/// is_prime(13) == true
/// is_prime(4) == false
/// ```
pub fn is_prime(n: i64) -> bool {
    if n > 1 {
        let upper_bound = (n as f64).sqrt() as i64 + 1;

        for i in 2..upper_bound {
            if n % i == 0 {
                return false;
            }
        }

        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {}

    #[test]
    fn test_square_of_sums() {
        // Positive
        assert_eq!(square_of_sums(10), 3025);
        assert_eq!(square_of_sums(1), 1);
    }

    #[test]
    fn test_sum_of_squares() {
        // Positive
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(sum_of_squares(1), 1);
    }

    #[test]
    fn test_reverse_int() {
        // Positive
        assert_eq!(reverse_int(12345), 54321);
        assert_eq!(reverse_int(1), 1);
        assert_eq!(reverse_int(0), 0);
        // Negative
        assert_ne!(reverse_int(12345), 12345);
    }

    #[test]
    fn test_gcd() {
        // Positive
        assert_eq!(gcd(1, 100), 1);
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(128, 96), 32);
        // Negative
        assert_ne!(gcd(8, 12), 2);
        assert_ne!(gcd(8, 12), 8);
    }

    #[test]
    fn test_get_file_from_path() {}

    #[test]
    fn test_get_data_from_file() {}

    #[test]
    fn test_transform_data() {
        let mut transformed_data = Vec::<i32>::new();
        transformed_data.push('1'.to_digit(10).unwrap() as i32);
        transformed_data.push('2'.to_digit(10).unwrap() as i32);
        transformed_data.push('3'.to_digit(10).unwrap() as i32);
        transformed_data.push('4'.to_digit(10).unwrap() as i32);
        transformed_data.push('5'.to_digit(10).unwrap() as i32);
        // Positive
        assert_eq!(
            transform_data("1\n2\n3\n4\n5".to_string()),
            transformed_data
        );
    }

    #[test]
    fn test_transform_char_data() {
        let mut transformed_data = Vec::<i64>::new();
        transformed_data.push('1'.to_digit(10).unwrap() as i64);
        transformed_data.push('2'.to_digit(10).unwrap() as i64);
        transformed_data.push('3'.to_digit(10).unwrap() as i64);
        transformed_data.push('4'.to_digit(10).unwrap() as i64);
        transformed_data.push('5'.to_digit(10).unwrap() as i64);
        // Positive
        assert_eq!(transform_char_data("12345".to_string()), transformed_data);
    }

    #[test]
    fn test_is_prime() {
        // Positive
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        // Negative
        assert!(!is_prime(-1));
        assert!(!is_prime(0));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(10));
        assert!(!is_prime(100));
        assert!(!is_prime(40));
        assert!(!is_prime(25));
    }
}
