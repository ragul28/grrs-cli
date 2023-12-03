#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs_cli::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}