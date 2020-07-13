use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

lazy_static! {
    static ref APOSTROPHE_VARIATION: Regex = Regex::new("’").unwrap();
}

#[derive(Clone, Debug)]
pub struct MsgEntry {
    pub id: String,
    pub value: String,
}

pub fn load_file(file_path: impl AsRef<Path>) -> Vec<MsgEntry> {
    let mut entries = Vec::new();
    let mut current_id = None;
    let file = BufReader::new(File::open(file_path).expect("Failed to open gettext file"));

    for line in file.lines() {
        let line = line.expect("Failed to read from gettext file");
        let line = line.trim();

        if let Some(msg_id) = parse_line(line, "msgid \"", "\"") {
            current_id = Some(normalize(msg_id));
        } else {
            if let Some(value) = parse_line(line, "msgstr \"", "\"").map(String::from) {
                if let Some(id) = current_id.take() {
                    entries.push(MsgEntry { id, value });
                }
            }

            current_id = None;
        }
    }

    entries
}

fn parse_line<'l>(line: &'l str, prefix: &str, suffix: &str) -> Option<&'l str> {
    if line.starts_with(prefix) && line.ends_with(suffix) {
        let start = prefix.len();
        let end = line.len() - suffix.len();

        Some(&line[start..end])
    } else {
        None
    }
}

fn normalize(string: &str) -> String {
    APOSTROPHE_VARIATION.replace_all(&string, "'").into_owned()
}
