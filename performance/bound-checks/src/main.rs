fn main() {
    println!("Hello, world!");
}

fn sum_next_three(idx: usize, nums: &[i32]) -> i32 {
    nums[idx + 1] + nums[idx + 2] + nums[idx + 3]
}

fn sum_next_three_assert_no_const(idx: usize, nums: &[i32]) -> i32 {
    // No const values => No optimizations.
    assert!(nums.len() > 3);
    nums[idx + 1] + nums[idx + 2] + nums[idx + 3]
}

fn sum_next_three_optimized(idx: usize, nums: &[i32]) -> i32 {
    let nums = &nums[idx..];
    // We must use const values here to activate
    // compiler optimizations
    assert!(nums.len() > 3);
    nums[1] + nums[2] + nums[3]
}
