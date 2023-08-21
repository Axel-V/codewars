fn are_you_playing_banjo(name: &str) -> String {
    let first_letter = name.to_lowercase().chars().nth(0).unwrap();
    let mut result = String::new();
    if first_letter == 'r' {
        result.push_str(&format!("{} plays banjo", name));
    } else {
        result.push_str(&format!("{} does not play banjo", name));
    }
    return result;
}

#[test]
fn test_are_you_playing_banjo() {
    assert_eq!(
        are_you_playing_banjo("Martin"),
        "Martin does not play banjo"
    );
    assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
    assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
    assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
}
