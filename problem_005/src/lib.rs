// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

#![allow(unused)]

fn find_number() -> u32 {
    let mut number = 2521;
    loop {
        if is_divisible(number) {
            return number;
        }
        number += 1;
    }
}

fn is_divisible(number: u32) -> bool {
    for i in 1..=20 {
        if number % i != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_solution() {
        let result = find_number();
        assert_eq!(result, 232792560);
    }
}
