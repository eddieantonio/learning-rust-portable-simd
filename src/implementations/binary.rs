pub fn problem_1(nums: &[i16]) -> usize {
    let n_positives = bisect_right(nums, 0).len();
    let n_negatives = bisect_left(nums, 0).len();

    std::cmp::max(n_positives, n_negatives)
}

/// Return a subslice of everything to that is less than `x`.
fn bisect_left<T: std::cmp::PartialOrd>(slice: &[T], x: T) -> &[T] {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if slice[mid] < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    &slice[..low]
}

/// Return a subslice of everything such that `x < a[i]` for all `i`.
fn bisect_right<T: std::cmp::PartialOrd>(slice: &[T], x: T) -> &[T] {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if x < slice[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    &slice[high..]
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
