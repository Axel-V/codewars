fn longest(a1: &str, a2: &str) -> String {
    // Take 2 strings s1 and s2 including only letters from a to z.
    // Return a new sorted string, the longest possible,
    // containing distinct letters - each taken only once - coming from s1 or s2.

    // Merge two strings together
    let mut merged = String::from(a1) + a2;
    let mut result = String::new();

    for char in merged.chars() {
        if !result.contains(char) {
            result.push(char);
        }
    }

    // Transform the string into a vector of chars
    let mut result_array: Vec<char> = result.chars().collect();

    // sort the vector
    result_array.sort();

    // transform the vector into a string
    result = result_array.into_iter().collect();

    return result;
}

fn alternate_longest(a1: &str, a2: &str) -> String {
    let mut result: Vec<_> = a1.chars().collect();
    result.extend(a2.chars());
    result.sort();
    result.dedup();
    result.into_iter().collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
