//! Provides the `parse_cmd(&String, &str)` function.

/// Parses a `String` based on the expected `str`.  This will return
/// `None` only if `expected` doesn't match `cmd`.  It will return
/// `Some(None)` if it parses but there isn't assignment.  It will
/// return `Some(Some(String))`if it parses and there is an assignment
/// statement.
///
/// # Examples
///
/// ```
/// assert_eq!(parse_cmd(&"artist=WRLD".to_string(), "artist"), Some(Some("WRLD".to_string())));
/// assert_eq!(parse_cmd(&"artist".to_string(), "artist"), Some(None));
/// assert_eq!(parse_cmd(&"random_arg".to_string(), "artist"), None);
/// ```
pub fn parse_cmd(cmd: &String, expected: &str) -> Option<Option<String>> {
    let mut ci = cmd.chars();
    let mut ei = expected.chars();
    loop {
        match (ci.next(), ei.next()) {
            (None, None) => return Some(None),
            (Some('='), None) => return Some(Some(ci.collect())),
            (Some(c), Some(e)) => {
                if c == e {
                    continue;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cmd_test() {
        assert_eq!(parse_cmd(&"artist=WRLD".to_string(), "artist"),
                   Some(Some("WRLD".to_string())));
        assert_eq!(parse_cmd(&"artist".to_string(), "artist"), Some(None));
        assert_eq!(parse_cmd(&"random_arg".to_string(), "artist"), None);
    }
}
