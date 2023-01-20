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

    #[test]
    fn test_find_lower_range() {
        assert_eq!(Some(1), find_lower_range_start(&[-1, 0, 1]));
    }

    #[test]
    fn test_find_lower_range_2() {
        assert_eq!(Some(3), find_lower_range_start(&[-1, -1, -1]));
    }

    #[test]
    fn test_find_lower_range_3() {
        assert_eq!(None, find_lower_range_start(&[1, 1, 1]));
    }

    #[test]
    fn test_find_lower_range_4() {
        assert_eq!(Some(0), find_lower_range_start(&[0]));
    }

    #[test]
    fn test_find_lower_range_5() {
        assert_eq!(Some(3), find_lower_range_start(&[-1, -1, -1, 0]));
    }

    #[test]
    fn test_find_lower_range_6() {
        assert_eq!(Some(1), find_lower_range_start(&[-1, 0, 1, 1]));
    }

    #[test]
    fn test_find_lower_range_7() {
        assert_eq!(Some(1), find_lower_range_start(&[-1, 0, 0, 1]));
    }

    #[test]
    fn test_find_lower_range_8() {
        assert_eq!(Some(2), find_lower_range_start(&[-1, -1, 0, 1]));
    }
}
