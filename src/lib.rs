// ----------------------------------------------------------------------------
/// 'is_prime_all'
///
/// Checks for primality by checking every value less than num
///
/// # arguments
///
/// * `num` - number to check primality
///
/// # return
///
/// * `true` if `num` is prime, otherwise `false`
///
/// ```
/// # use time_complexity::is_prime_all;
/// let prime = 179;
/// assert!(is_prime_all(prime));
///
/// let composite = 180;
/// assert!(!is_prime_all(composite));
/// ```
// ----------------------------------------------------------------------------
pub fn is_prime_all(num: i64) -> bool {
    // Check for divisors of num
    for i in 2..num {
        if num % i == 0 {
            // Divisor other than 1 or num means num is not prime
            return false;
        }
    }
    // No other divisors found means num is prime
    return true;
}

// ----------------------------------------------------------------------------
/// 'is_prime_half'
///
/// Checks for primality by checking every value less than num/2
///
/// # arguments
///
/// * `num` - number to check primality
///
/// # return
///
/// * `true` if `num` is prime, otherwise `false`
///
/// ```
/// # use time_complexity::is_prime_half;
/// let prime = 179;
/// assert!(is_prime_half(prime));
///
/// let composite = 180;
/// assert!(!is_prime_half(composite));
/// ```
// ----------------------------------------------------------------------------
pub fn is_prime_half(num: i64) -> bool {
    // Check for divisors of num
    for i in 2..num / 2 {
        if num % i == 0 {
            // Divisor other than 1 or num means num is not prime
            return false;
        }
    }
    // No other divisors found means num is prime
    return true;
}

// ----------------------------------------------------------------------------
/// 'is_prime_sqrt'
///
/// Checks for primality by checking every value less than or equal to sqrt(num)
///
/// # arguments
///
/// * `num` - number to check primality
///
/// # return
///
/// * `true` if `num` is prime, otherwise `false`
///
/// ```
/// # use time_complexity::is_prime_sqrt;
/// let prime = 179;
/// assert!(is_prime_sqrt(prime));
///
/// let composite = 180;
/// assert!(!is_prime_sqrt(composite));
/// ```
// ----------------------------------------------------------------------------
pub fn is_prime_sqrt(num: i64) -> bool {
    // Check for divisors of num
    for i in 2..=(num as f64).sqrt() as i64 {
        if num % i == 0 {
            // Divisor other than 1 or num means num is not prime
            return false;
        }
    }
    // No other divisors found means num is prime
    return true;
}

// ----------------------------------------------------------------------------
/// 'is_prime'
///
/// Checks for primality by checking every value less than or equal to sqrt(num)
///
/// # arguments
///
/// * `num` - number to check primality
///
/// # return
///
/// * `true` if `num` is prime, otherwise `false`
///
/// ```
/// # use time_complexity::is_prime;
/// let prime = 179;
/// assert!(is_prime(prime));
///
/// let composite = 180;
/// assert!(!is_prime(composite));
/// ```
// ----------------------------------------------------------------------------
pub fn is_prime(num: i64) -> bool {
    (2..=(num as f64).sqrt() as i64).all(|i| num % i != 0)
}

// ----------------------------------------------------------------------------
/// 'sum_prime'
///
/// Computes the sum of all primes in the range [1, num], inclusive
///
/// # arguments
///
/// * `num`      - upper bound of range
/// * 'is_prime' - primality checking function
///
/// # return
///
/// * sum of primes in range from 1 to `num`
///
/// ```
/// # use time_complexity::{is_prime, sum_primes};
/// let prime = 179;
/// assert_eq!(3_266, sum_primes(prime, is_prime));
///
/// let composite = 180;
/// assert_eq!(3_266, sum_primes(composite, is_prime));
/// ```
// ----------------------------------------------------------------------------
pub fn sum_primes<F>(num: i64, is_prime: F) -> i64
where
    F: Fn(i64) -> bool,
{
    (2..=num).fold(0, |acc, i| if is_prime(i) { acc + i } else { acc })
} // ----------------------------------------------------------------------------
/// 'sum_prime_sieve'
///
/// Computes the sum of all primes in the range [1, num], inclusive with a prime
///   number sieve
///
/// # arguments
///
/// * `num` - upper bound of range
///
/// # return
///
/// * sum of primes in range from 1 to `num`
///
/// ```
/// # use time_complexity::sum_primes_sieve;
/// let prime = 179;
/// assert_eq!(3_266, sum_primes_sieve(prime));
///
/// let composite = 180;
/// assert_eq!(3_266, sum_primes_sieve(composite));
/// ```
// ----------------------------------------------------------------------------
pub fn sum_primes_sieve(num: i64) -> i64 {
    // Check for early return
    if num <= 1 {
        return 0;
    };

    // Boolean array of odd numbers
    let upper = (num - 1) / 2;
    let mut sieve = vec![true; upper as usize];

    // Initialize sum
    let mut sum = 2;

    // Sieve array
    let sqrt_upper = (upper as f64).sqrt() as i64;
    assert!((sqrt_upper as usize) < sieve.len());
    for i in 0..sqrt_upper {
        if sieve[i as usize] {
            let prime = 2 * i + 3; // Note: formula comes from only tracking odd values
            sum += prime;

            let prime_squared_index = 2 * i * i + 6 * i + 3;
            for j in (prime_squared_index..upper).step_by(prime as usize) {
                sieve[j as usize] = false;
            }
        }
    }

    // Count remaining primes in sum
    sum + sieve
        .iter()
        .enumerate()
        .skip(sqrt_upper as usize)
        .fold(
            0,
            |acc, (i, &prime)| if prime { acc + 2 * i + 3 } else { acc },
        ) as i64
}

// ----------------------------------------------------------------------------
