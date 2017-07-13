//! Utility methods to use during testing
use std::fmt::Debug;
use std::cmp::PartialEq;
/// Strips all whitespace anywhere within the string. Useful for comparing
/// strings when only caring about content.
fn strip_all_whitespace(string: &str) -> String {
    string.chars().filter(|x| !x.is_whitespace()).collect()
}

/// Asserts the two strings provided have the same non-whitespace content.
pub fn compare_string_content(expected: &str, actual: &str) {
    let expected = strip_all_whitespace(&expected);
    let actual = strip_all_whitespace(&actual);

    assert_eq!(expected, actual);
}

pub fn compare_vec<T: Debug + PartialEq>(expected: Vec<T>, actual: Vec<T>) {
    assert_eq!(expected.len(), actual.len());
    for i in 0..expected.len() {
        assert_eq!(expected[i], actual[i]);
    }
}
