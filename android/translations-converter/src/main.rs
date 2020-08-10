mod android;
mod gettext;

use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

fn main() {
    let resources_dir = Path::new("../src/main/res");
    let strings_file = File::open(resources_dir.join("values/strings.xml"))
        .expect("Failed to open string resources file");
    let string_resources: android::StringResources =
        serde_xml_rs::from_reader(strings_file).expect("Failed to read string resources file");

    let line_breaks = Regex::new(r"\s*\n\s*").unwrap();

    let known_strings: HashMap<_, _> = string_resources
        .into_iter()
        .map(|string| {
            let android_id = string.name;
            let string_value = line_breaks.replace_all(&string.value, " ").into_owned();

            (string_value, android_id)
        })
        .collect();

    let locale_files = fs::read_dir("../../gui/locales")
        .expect("Failed to open root locale directory")
        .filter_map(|dir_entry_result| dir_entry_result.ok().map(|dir_entry| dir_entry.path()))
        .filter(|dir_entry_path| dir_entry_path.is_dir())
        .map(|dir_path| dir_path.join("messages.po"))
        .filter(|file_path| file_path.exists());

    for locale_file in locale_files {
        let locale = locale_file
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let destination_dir = resources_dir.join(&android_locale_directory(locale));

        if !destination_dir.exists() {
            fs::create_dir(&destination_dir).expect("Failed to create Android locale directory");
        }

        generate_translations(
            known_strings.clone(),
            gettext::load_file(&locale_file),
            destination_dir.join("strings.xml"),
        );
    }
}

fn android_locale_directory(locale: &str) -> String {
    let mut directory = String::from("values-");
    let mut parts = locale.split("-");

    directory.push_str(parts.next().unwrap());

    if let Some(region) = parts.next() {
        directory.push_str("-r");
        directory.push_str(region);
    }

    directory
}

fn generate_translations(
    mut known_strings: HashMap<String, String>,
    translations: Vec<gettext::MsgEntry>,
    output_path: impl AsRef<Path>,
) {
    let mut localized_resource = android::StringResources::new();

    for translation in translations {
        if let Some(android_key) = known_strings.remove(&translation.id) {
            localized_resource.push(android::StringResource {
                name: android_key,
                value: translation.value,
            });
        }
    }

    fs::write(output_path, localized_resource.to_string())
        .expect("Failed to create Android locale file");

    println!("Missing translations:");

    for (missing_translation, id) in known_strings {
        println!("  {}: {}", id, missing_translation);
    }
}
