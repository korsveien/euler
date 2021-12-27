pub fn problem_001() -> u32 {
    let mut sum = 0;
    let mut i = 0;

    while i < 1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }
    sum
}

pub fn problem_002() -> u32 {
    let mut previous = 0;
    let mut next = 1;
    let mut sum = 0;

    while next < 4_000_000 {
        if next % 2 == 0 {
            sum += next;
        }
        let tmp = next;
        next = previous + next;
        previous = tmp;
    }
    sum
}

// Trial division:
// 1. Find the smallest prime number that is a divisor without remainder
// 2. Add this prime to a collection and use it to divide our number to find a new number
// 3. Find the smalles prime number that is a divisor without remainder for this new number
// 4. Keep doing this until we have a divisor that is higher than the square root of the original
//    number (no prime factors can be higher than the square root)
pub fn problem_003() -> u64 {
    let mut number: u64 = 600_851_475_143;
    let mut primes = Vec::new();
    let mut divisor = 2;

    while 600_851_475_143 >= divisor * divisor {
        if number % divisor == 0 && is_prime(divisor) {
            primes.push(divisor);
            number = number / divisor;
            divisor = 2;
        } else {
            if divisor == 2 {
                divisor += 1;
            } else {
                // optimalization: skip even numbers since all prime numbers are odd
                divisor += 2;
            }
        }
    }

    *primes.last().unwrap()
}

fn is_prime(number: u64) -> bool {
    if number == 1 {
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
    fn problem_001_solution() {
        let result = problem_001();
        assert_eq!(result, 233168);
    }

    #[test]
    fn problem_002_solution() {
        let result = problem_002();
        assert_eq!(result, 4613732);
    }

    // The prime factors of 13195 are 5, 7, 13 and 29.
    // What is the largest prime factor of the number 600851475143 ?
    #[test]
    fn problem_003_solution() {
        let result = problem_003();
        assert_eq!(result, 6857);
    }
}
