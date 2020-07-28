use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

lazy_static! {
    static ref APOSTROPHE_VARIATION: Regex = Regex::new("’").unwrap();
    static ref PARAMETERS: Regex = Regex::new(r"%\([^)]*\)").unwrap();
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
            if let Some(translation) = parse_line(line, "msgstr \"", "\"") {
                if let Some(id) = current_id.take() {
                    let value = normalize(translation);

                    entries.push(MsgEntry { id, value });
                }
            }

            current_id = None;
        }
    }

    entries
}

pub fn append_to_template(
    file_path: impl AsRef<Path>,
    entries: impl Iterator<Item = MsgEntry>,
) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);

    for entry in entries {
        writeln!(writer)?;
        writeln!(writer, "msgid {:?}", entry.id)?;
        writeln!(writer, "msgstr {:?}", entry.value)?;
    }

    Ok(())
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
    let string = APOSTROPHE_VARIATION.replace_all(&string, "'");
    let string = PARAMETERS.replace_all(&string, "%");

    string.into_owned()
}
