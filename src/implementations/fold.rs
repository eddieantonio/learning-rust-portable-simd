pub fn problem_1(nums: &[i16]) -> usize {
    let (pos, neg) = nums.iter().fold((0, 0), |(positives, negatives), &n| {
        (positives + (n > 0) as i16, negatives + (n < 0) as i16)
    });

    std::cmp::max(pos, neg) as usize
}
