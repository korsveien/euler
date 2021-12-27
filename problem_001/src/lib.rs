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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_001_solution() {
        let result = problem_001();
        assert_eq!(result, 233168);
    }
}
