fn fake_bin(input: &str) -> String {
    // Should transform the string input into a fake binary,
    // Any number above or equal to 5 should be replaced with 1,
    // Any number below 5 should be replaced with 0.

    let mut result = String::new();
    for c in input.chars() {
        if c >= '5' {
            result.push('1');
        } else {
            result.push('0');
        }
    }
    return result;
}

#[test]
fn fake_binary_tests() {
    assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
    assert_eq!(fake_bin("509321967506747"), "101000111101101");
    assert_eq!(
        fake_bin("366058562030849490134388085"),
        "011011110000101010000011011"
    );
}
