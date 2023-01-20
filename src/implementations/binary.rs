pub fn problem_1(nums: &[i16]) -> i16 {
    0
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
}
