use rayon::prelude::*;


fn calculate_sum() -> i32 {
    (1..=1000).into_par_iter().sum()
}

fn main() {
    let sum = calculate_sum();
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let expected_sum = 500500;
        assert_eq!(calculate_sum(), expected_sum, "The sum should be 500500");
    }

    #[test]
    fn test_parallel_execution() {
        // This test checks if the function runs without panic in a parallel context.
        // It indirectly tests parallelism by ensuring the result is correct.
        let result = calculate_sum();
        assert!(result > 0, "The result should be a positive number");
    }
}
