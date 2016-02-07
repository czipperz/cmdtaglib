//! Provides the `parse` function which will generate a `CMD` from a `String`.

use help::show_help;
use cmd::CMD::*;
use cmd::CMD;

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
fn parse_cmd(cmd: &String, expected: &str) -> Option<Option<String>> {
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

/// Parses a `String`, using one of `album`, `artist`, `comment`,
/// `genre`, `title`, `track`, `year`, `help`, and `--help` as valid
/// commands.
///
/// The commands `help` and `--help` and any unrecognized commands
/// will cause the program to show help and then exit.  A successful
/// exit will occur when `help` or `--help` are found (error code 0).
/// Unrecognized commands will cause an unsuccessful exit (error code
/// 1).  Showing help requires a program name, hence the argument
/// `progname`.
///
/// The other commands can be used in the fasion `<command>=<value>`
/// (for example `artist=WRLD`).  This will cause the `CMD` to be
/// turned from a get into a set.
///
/// # Examples
///
/// ```
/// assert_eq!(parse(&"artist=WRLD".to_string(), &"".to_string()), SetArtist("WRLD".to_string()));
/// assert_eq!(parse(&"title=Little Too Close".to_string(), &"".to_string()), SetTitle("Little Too Close".to_string()));
/// assert_eq!(parse(&"year".to_string(), &"".to_string()), Year);
/// assert_eq!(parse(&"track".to_string(), &"".to_string()), Track);
/// ```
pub fn parse(str: &String, progname: &String) -> CMD {
    match parse_cmd(str, "album") {
        Some(Some(s)) => SetAlbum(s),
        Some(None) => Album,
        _ => {
            match parse_cmd(str, "artist") {
                Some(Some(s)) => SetArtist(s),
                Some(None) => Artist,
                _ => {
                    match parse_cmd(str, "comment") {
                        Some(Some(s)) => SetComment(s),
                        Some(None) => Comment,
                        _ => {
                            match parse_cmd(str, "genre") {
                                Some(Some(s)) => SetGenre(s),
                                Some(None) => Genre,
                                _ => {
                                    match parse_cmd(str, "title") {
                                        Some(Some(s)) => SetTitle(s),
                                        Some(None) => Title,
                                        _ => {
                                            match parse_cmd(str, "track") {
                                                Some(Some(s)) => SetTrack(s),
                                                Some(None) => Track,
                                                _ => {
                                                    match parse_cmd(str, "year") {
                                                        Some(Some(s)) => SetYear(s),
                                                        Some(None) => Year,
                                                        _ => {
                                                            if str == "help" ||
                                                                str == "--help" {
                                                                println!("Showing help:");
                                                                show_help(&progname, 0);
                                                            } else {
                                                                println!("Unrecognized \
                                                                            command: {}",
                                                                            str);
                                                                show_help(&progname, 1);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_cmd_test() {
        use super::parse_cmd;
        assert_eq!(parse_cmd(&"artist=WRLD".to_string(), "artist"),
                   Some(Some("WRLD".to_string())));
        assert_eq!(parse_cmd(&"artist".to_string(), "artist"), Some(None));
        assert_eq!(parse_cmd(&"random_arg".to_string(), "artist"), None);
    }

    #[test]
    fn parse_test() {
        use super::parse;
        use super::super::cmd::CMD::*;
        assert_eq!(parse(&"artist=WRLD".to_string(), &"".to_string()),
                   SetArtist("WRLD".to_string()));
        assert_eq!(parse(&"title=WRLD".to_string(), &"".to_string()),
                   SetTitle("WRLD".to_string()));
        assert_eq!(parse(&"year".to_string(), &"".to_string()),
                   Year);
        assert_eq!(parse(&"track".to_string(), &"".to_string()),
                   Track);
    }
}
