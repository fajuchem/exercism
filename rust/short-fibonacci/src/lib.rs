/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci(n: i32) -> f64 {
    let phi = (1.0 + f64::sqrt(5.0)) / 2.0;
    let asymp = phi.powi(n) / f64::sqrt(5.0);

    asymp.ceil()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn it_works() {
        let fib_val = fibonacci(1000);

        assert_eq!(fib_val, 4.346655768693734e208);
    }
}
