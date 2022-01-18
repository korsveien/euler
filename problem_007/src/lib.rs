// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

#![allow(unused)]
fn solution() -> u64 {
    let mut number: u64 = 0;
    let mut primes = Vec::with_capacity(10_001);

    while primes.len() <= 10_000 {
        if is_prime(number) {
            primes.push(number);
        }
        number += 1;
    }
    println!("{:#?}", primes);
    *primes.last().unwrap()
}

fn is_prime(number: u64) -> bool {
    if number == 2 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
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
