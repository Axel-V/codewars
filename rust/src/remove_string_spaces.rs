fn no_space(x: String) -> String {
    x.replace(" ", "")
}

#[test]
fn test_no_space() {
    assert_eq!(no_space("Hello World".to_string()), "HelloWorld");
    assert_eq!(no_space("Rust is fun".to_string()), "Rustisfun");
    assert_eq!(no_space("".to_string()), "");
    assert_eq!("8j8mBliB8gimjB8B8jlB", no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
    assert_eq!("88Bifk8hB8BB8BBBB888chl8BhBfd", no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string()));
    assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
    assert_eq!("jfBmgklf8hg88lbe8", no_space("jfBm  gk lf8hg  88lbe8 ".to_string()));
    assert_eq!("8jaam", no_space("8j aam".to_string()));
}
