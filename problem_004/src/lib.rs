// Problem:
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

#![allow(unused)]

fn find_largest_palindrome() -> u32 {
    let mut palindromes: Vec<u32> = Vec::new();

    for x in 100..1000 {
        for y in (100..1000).rev() {
            let product = x * y;
            if is_palindrome(product.to_string().chars()) {
                palindromes.push(product);
            }
        }
    }
    palindromes.sort_unstable();
    *palindromes.last().unwrap()
}

fn is_palindrome<I>(iterable: I) -> bool
where
    I: IntoIterator,
    I::Item: PartialEq,
    I::IntoIter: DoubleEndedIterator,
{
    let mut iter = iterable.into_iter();
    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if front != back {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_verify_that_string_is_a_palindrome() {
        let number: u32 = 9009;
        let result = is_palindrome(number.to_string().chars());
        assert_eq!(result, true);
    }

    #[test]
    fn should_verify_that_string_is_not_a_palindrome() {
        let number: u32 = 9001;
        let result = is_palindrome(number.to_string().chars());
        assert_eq!(result, false);
    }

    #[test]
    fn should_find_largest_palindrome() {
        let result = find_largest_palindrome();
        assert_eq!(result, 906609);
    }
}
