mod android;
mod gettext;

use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

fn main() {
    let resources_dir = Path::new("../src/main/res");
    let strings_file = File::open(resources_dir.join("values/strings.xml"))
        .expect("Failed to open string resources file");
    let mut string_resources: android::StringResources =
        serde_xml_rs::from_reader(strings_file).expect("Failed to read string resources file");

    string_resources.normalize();

    let (known_urls, known_strings): (HashMap<_, _>, _) = string_resources
        .into_iter()
        .map(|string| {
            let android_id = string.name;

            (string.value, android_id)
        })
        .partition(|(string_value, _)| string_value.starts_with("https://mullvad.net/en/"));

    let mut missing_translations = known_strings.clone();

    let locale_dir = Path::new("../../gui/locales");
    let locale_files = fs::read_dir(&locale_dir)
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
            locale,
            known_urls.clone(),
            known_strings.clone(),
            gettext::load_file(&locale_file),
            destination_dir.join("strings.xml"),
            &mut missing_translations,
        );
    }

    println!("Missing translations:");

    for (missing_translation, id) in &missing_translations {
        println!("  {}: {}", id, missing_translation);
    }

    gettext::append_to_template(
        locale_dir.join("messages.pot"),
        missing_translations
            .into_iter()
            .map(|(id, _)| gettext::MsgEntry {
                id,
                value: String::new(),
            }),
    )
    .expect("Failed to append missing translations to message template file");
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
    locale: &str,
    known_urls: HashMap<String, String>,
    mut known_strings: HashMap<String, String>,
    translations: Vec<gettext::MsgEntry>,
    output_path: impl AsRef<Path>,
    missing_translations: &mut HashMap<String, String>,
) {
    let mut localized_resource = android::StringResources::new();

    for translation in translations {
        if let Some(android_key) = known_strings.remove(&translation.id) {
            localized_resource.push(android::StringResource::new(
                android_key,
                &translation.value,
            ));
        }
    }

    if let Some(web_locale) = website_locale(locale) {
        let locale_path = format!("/{}/", web_locale);

        for (url, android_key) in known_urls {
            localized_resource.push(android::StringResource::new(
                android_key,
                &url.replacen("/en/", &locale_path, 1),
            ));
        }
    }

    fs::write(output_path, localized_resource.to_string())
        .expect("Failed to create Android locale file");

    missing_translations.retain(|translation, _| known_strings.contains_key(translation));
}

fn website_locale(locale: &str) -> Option<&str> {
    match locale {
        locale if !locale.contains("-") => Some(locale),
        "zh-TW" => Some("zh-hant"),
        unknown_locale => {
            eprintln!("Unknown locale: {}", unknown_locale);
            None
        }
    }
}
