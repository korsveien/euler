// The sum of the squares of the first ten natural numbers is: 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is: (1+2+...+10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is:
// 3025 - 385 = 2640

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

#![allow(unused)]
fn solution() -> u32 {
    square_of_the_sum(100) - sum_of_the_squares(100)
}

fn sum_of_the_squares(limit: u32) -> u32 {
    (1..=limit).map(|x| x.pow(2)).sum()
}

fn square_of_the_sum(limit: u32) -> u32 {
    (1..=limit).sum::<u32>().pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_sum_of_the_squares_of_the_first_hundred_natural_numbers() {
        let result = sum_of_the_squares(100);
        assert_eq!(result, 338350);
    }

    #[test]
    fn should_calculate_square_of_the_sum_of_the_first_hundred_natural_numbers() {
        let result = square_of_the_sum(100);
        assert_eq!(result, 25502500);
    }

    #[test]
    fn should_calculate_the_difference() {
        let result = solution();
        assert_eq!(result, 25164150);
    }
}
