// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

#![allow(unused)]
fn solution() -> u64 {
    let mut prime_candidate: u64 = 3;
    let mut primes = Vec::with_capacity(10_001);
    primes.push(2);

    while primes.len() <= 10_000 {
        let is_prime = !primes.iter().any(|&x| prime_candidate % x == 0);

        if is_prime {
            primes.push(prime_candidate);
        }
        prime_candidate += 2;
    }

    *primes.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = solution();
        assert_eq!(result, 104743);
    }
}
