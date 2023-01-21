pub fn problem_1(nums: &[i16]) -> usize {
    // This works by finding two indices:
    //
    //              numbers:
    // | -3 | -2 | -1 |  0 |  0 |  0 |  0 |  1 |  2 |  3 |
    // |0===|1===|2===|3===|4===|5===|6===|7===|8===|9===| n = 10
    //                 ^                   ^
    //                 a                   b
    //
    // a = First index where there can be no more negative numbers.
    //     In other words, everything to the left satisfies the property:
    //     a[i] < 0.
    // b = First index where there must be positive numbers.
    //     In other words, everything to the left satistifies the property:
    //     a[i] <= 0.
    //
    // The number of negatives is equal to a -- nothing to do there.
    // The number of positives is the area from b to n, or in other words, n - b.
    //
    // slice.partition_point() performs a binary search, probing the given property.
    // If the entire array satisfies the property, then it returns n.
    // The consequence is that, if all a[i] <= 0, then b = n which means n - b == 0,
    // which means there are no positive numbers in the array.
    let n_negatives = nums.partition_point(|&x| x < 0);
    let n_positives = nums.len() - nums.partition_point(|&x| x <= 0);
    std::cmp::max(n_negatives, n_positives)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(1, &[-1, 0, 1] ; "Basic")]
    #[test_case(3, &[-1, -1, -1] ; "All negative 1")]
    #[test_case(3, &[1, 1, 1] ; "All 1")]
    #[test_case(0, &[0] ; "Just zero")]
    #[test_case(3, &[-1, -1, -1, 0] ; "Zero at end")]
    #[test_case(4, &[0, 1, 1, 1, 1] ; "Zero at beginning")]
    #[test_case(2, &[-1, 0, 1, 1] ; "Zero off from middle")]
    #[test_case(1, &[-1, 0, 0, 1] ; "Span of zeros")]
    #[test_case(2, &[-1, -1, 0, 1] ; "Zero off from middle 1")]
    fn test_problem_1(expected: usize, array: &[i16]) {
        assert_eq!(expected, problem_1(array));
    }
}
