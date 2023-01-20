pub fn problem_1(nums: &[i16]) -> usize {
    // Unable to find the first zero? (because all greater than zero)
    // then there are no negatives:
    let n_negatives = find_lower_range_start(nums).unwrap_or(0);

    // Unable to find the first zero?
    let zeros_end_index = find_upper_range_start(&nums[n_negatives..]);
    let n_positives = nums.len() - (n_negatives + zeros_end_index);

    std::cmp::max(n_positives, n_negatives)
}

fn find_lower_range_start(nums: &[i16]) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len();

    if nums.is_empty() {
        return None;
    }

    if nums[0] > 0 {
        // The first element is greater than zero.  If it's in non-decreasing order, that means it
        // can't contain zero!
        return None;
    }

    if nums[0] == 0 {
        // Avoid looping with this one weird trick!
        return Some(0);
    }

    while high - low > 1 {
        let midpoint = low + (high - low) / 2;
        if nums[midpoint] >= 0 {
            // Eliminate the right-hand side to search in the left.
            high = midpoint;
        } else {
            // Eliminate the left-hand side to search in the right.
            low = midpoint;
        }
    }

    Some(high)
}

// Finds the first number greater than zero
fn find_upper_range_start(nums: &[i16]) -> usize {
    let mut low = 0;
    let mut high = nums.len();

    if nums.is_empty() {
        return 0;
    }

    while high > low {
        let midpoint = low + (high - low) / 2;
        if nums[midpoint] <= 0 {
            // Eliminate the left-hand side to search on the right.
            low = midpoint + 1;
        } else {
            // Eliminate the right-hand side to search on the left.
            high = midpoint;
        }
    }

    low
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Some(1), &[-1, 0, 1] ; "Basic")]
    #[test_case(Some(3), &[-1, -1, -1] ; "All negative 1")]
    #[test_case(None, &[1, 1, 1] ; "All 1")]
    #[test_case(Some(0), &[0] ; "Just zero")]
    #[test_case(Some(3), &[-1, -1, -1, 0] ; "Zero at end")]
    #[test_case(Some(0), &[0, 1, 1, 1, 1] ; "Zero at beginning")]
    #[test_case(Some(1), &[-1, 0, 1, 1] ; "Zero off from middle")]
    #[test_case(Some(1), &[-1, 0, 0, 1] ; "Span of zeros")]
    #[test_case(Some(2), &[-1, -1, 0, 1] ; "Zero off from middle 1")]
    fn test_find_lower_range(expected: Option<usize>, array: &[i16]) {
        assert_eq!(expected, find_lower_range_start(array));
    }

    #[test_case(2, &[-1, 0, 1] ; "Basic")]
    #[test_case(3, &[-1, -1, -1] ; "All negative 1")]
    #[test_case(0, &[1, 1, 1] ; "All 1")]
    #[test_case(1, &[0] ; "Just zero")]
    #[test_case(4, &[-1, -1, -1, 0] ; "Zero at end")]
    #[test_case(1, &[0, 1, 1, 1, 1] ; "Zero at beginning")]
    #[test_case(2, &[-1, 0, 1, 1] ; "Zero off from middle")]
    #[test_case(3, &[-1, 0, 0, 1] ; "Span of zeros")]
    #[test_case(3, &[-1, -1, 0, 1] ; "Zero off from middle 1")]
    fn test_find_upper_range(expected: usize, array: &[i16]) {
        assert_eq!(expected, find_upper_range_start(array));
    }

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
