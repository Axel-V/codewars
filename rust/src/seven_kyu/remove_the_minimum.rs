fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    // https://www.codewars.com/kata/563cf89eb4747c5fb100001b/train/rust
    // Given an array of integers, remove the smallest value.
    // Do not mutate the original array/list.
    // If there are multiple elements with the same value, remove the one with a lower index.
    // If you get an empty array/list, return an empty array/list.
    //
    // Don't change the order of the elements that are left.

    let mut copy = numbers.to_vec();

    if let Some(min_index) = copy
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .map(|(index, _)| index)
    {
        copy.remove(min_index);
    }

    return copy;
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(
            remove_smallest(a),
            expected,
            "{ERR_MSG} with numbers = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
