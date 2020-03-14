#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

use std::borrow::Cow;

pub use indexmap;
use indexmap::IndexMap;
use regex::{Regex, RegexBuilder};

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        TagOverwritten(line: usize, tag: String) {
            display(r#"Tag "{}" overwritten at line: {}"#, tag, line)
        }
        QueryWithoutTag(line: usize, query: String) {
            display(r#"Query without tag (line: {}): "{}""#, line, query)
        }
    }
}

#[derive(Debug, PartialEq)]
enum LineType {
    Empty,
    Tag,
    Query,
}

pub fn parse(text: &str) -> Result<IndexMap<String, String>, ParseError> {
    let mut queries = IndexMap::new();

    let mut last_type: Option<LineType> = None;
    let mut last_tag: Option<&str> = None;

    for (idx, line) in remove_multi_line_comments(text).lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let (ty, value) = parse_line(line);
        match ty {
            LineType::Empty => continue,
            LineType::Tag => {
                if last_type.is_some() && last_type.as_ref().unwrap() == &LineType::Tag {
                    return Err(ParseError::TagOverwritten(idx + 1, value.to_owned()));
                }

                last_tag = Some(value);
            }
            LineType::Query => {
                if last_tag.is_none() {
                    return Err(ParseError::QueryWithoutTag(idx + 1, value.to_owned()));
                }

                queries
                    .entry(last_tag.unwrap().to_owned())
                    .and_modify(|x| {
                        *x = format!("{} {}", *x, value);
                    })
                    .or_insert_with(|| value.to_owned());
            }
        };

        last_type = Some(ty);
    }

    Ok(queries)
}

// Inner comments are not allowed.
// Preserve newlines for better error messages.
fn remove_multi_line_comments(text: &str) -> Cow<'_, str> {
    lazy_static! {
        // RegexBuilder::new(r#"(/\*(?:[^*][^/])*?.?\*/)"#)
        static ref RE: Regex = RegexBuilder::new(r#"(/\*.*?\*/)"#)
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    }

    RE.replace_all(text, |caps: &regex::Captures| {
        let mut rep = String::with_capacity(caps[1].len());
        for c in caps[1].chars() {
            let nc = match c {
                '\r' => '\r',
                '\n' => '\n',
                _ => ' ',
            };
            rep.push(nc);
        }
        rep
    })
}

// Remove single-line comment and trim string
fn parse_line(mut line: &str) -> (LineType, &str) {
    lazy_static! {
        static ref RE_TAG: Regex = Regex::new(r#"^\s*--\s*name\s*:\s*(.*?)\s*$"#).unwrap();
    }

    match RE_TAG.captures(line) {
        Some(caps) => (LineType::Tag, caps.get(1).unwrap().as_str()),
        None => {
            if let Some(idx) = line.find("--") {
                line = line.get(0..idx).unwrap();
            };

            line = line.trim();
            if line.is_empty() {
                (LineType::Empty, line)
            } else {
                (LineType::Query, line)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_tag_overwritten() {
        let text = "--name: x\n--name: x";
        assert_eq!(
            parse(text).err(),
            Some(ParseError::TagOverwritten(2, "x".to_owned()))
        );
    }

    #[test]
    fn error_query_without_tag() {
        let text = "SELECT 1;";
        assert_eq!(
            parse(text).err(),
            Some(ParseError::QueryWithoutTag(1, "SELECT 1;".to_owned()))
        );
    }

    #[test]
    fn parse_text() {
        let text = "-- just comment\n--name: x\nselect 2;";
        let mut queries = IndexMap::new();
        queries.insert("x".to_owned(), "select 2;".to_owned());
        assert_eq!(parse(text).ok(), Some(queries));
    }

    #[test]
    fn remove_zero_comments() {
        let text = "123\nabc";
        let result = "123\nabc";
        assert_eq!(remove_multi_line_comments(text), result);
    }

    #[test]
    fn remove_line_comment() {
        let text = "123/*qqq*/ /*123**/ 321";
        let result = "123                 321";
        assert_eq!(remove_multi_line_comments(text), result);
    }

    #[test]
    fn remove_multi_line_comment() {
        let text = "123/*9\nqqq\nz*/321";
        let result = "123   \n   \n   321";
        assert_eq!(remove_multi_line_comments(text), result);
    }

    #[test]
    fn parse_line_with_comment() {
        let line = "33 -- 123";
        let result = (LineType::Query, "33");
        assert_eq!(parse_line(line), result);
    }

    #[test]
    fn parse_line_invalid_tag() {
        let line = "0 -- name: start";
        let result = (LineType::Query, "0");
        assert_eq!(parse_line(line), result);
    }

    #[test]
    fn parse_line_tag() {
        let line = " --  name:start";
        let result = (LineType::Tag, "start");
        assert_eq!(parse_line(line), result);
    }

    #[test]
    fn parse_line_tag_with_space() {
        let line = "-- name: start end ";
        let result = (LineType::Tag, "start end");
        assert_eq!(parse_line(line), result);
    }
}
