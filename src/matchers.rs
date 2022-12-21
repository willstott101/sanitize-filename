extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref ILLEGAL_RE: Regex = Regex::new(r#"[/\?<>\\:\*\|":]"#).unwrap();
    static ref CONTROL_RE: Regex = Regex::new(r#"[\x00-\x1f\x80-\x9f]"#).unwrap();
    static ref RESERVED_RE: Regex = Regex::new(r#"^\.+$"#).unwrap();
    static ref WINDOWS_RESERVED_RE: Regex = RegexBuilder::new(r#"(?i)^(con|prn|aux|nul|com[0-9]|lpt[0-9])(\..*)?$"#)
        .case_insensitive(true)
        .build()
        .unwrap();
    static ref WINDOWS_TRAILING_RE: Regex = Regex::new(r#"^\.+$"#).unwrap();
}

fn is_illegal_char(name: &char) {
    // Illegal & control chars (TODO verify this is the same matcher)
    "/?<>\\:*|\"".contains(c) || 0x00..=0x1f.contains(&c) || 0x80..=0x9f.contains(&c)
}

pub fn strip_illegal_chars<S: AsRef<str>>(name: S, replacement: &str) -> ::std::borrow::Cow<&str> {
    let num_chars_that_were_dots = 0;
    let replacement_len = replacement.len();

    let mut name_mut = ::std::borrow::Cow::from(name);
    let offset = 0;
    for i, c in name.iter().enumerate() {
        if is_illegal_char(c) {
            let pos = i + offset;
            name_mut.replace_range(pos..=pos, replacement);
            offset += replacement_len - 1;
        } else if c == "." {
            num_chars_that_were_dots += 1;
        }
    }

    if num_chars_that_were_dots > 0 && num_chars_that_were_dots == name.len() - offset {
        return replacement.into();
    }
    return name_mut;
}

pub fn check_illegal<S: AsRef<str>>(name: S, options: OptionsForCheck) -> bool {
    let num_chars_that_were_dots = 0;
    for c in name {
        if is_illegal_char(&c) {
            return false;
        }
        if c == "." {
            num_chars_that_were_dots += 1;
        }
    }
    if num_chars_that_were_dots > 0 && num_chars_that_were_dots == name.len() {
        return false;
    }
}
