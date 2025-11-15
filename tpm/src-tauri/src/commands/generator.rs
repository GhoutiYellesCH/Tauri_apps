use rand::seq::SliceRandom;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_+=[]{}|;:,.<>?/~";

#[derive(serde::Deserialize)]
pub struct GeneratorOptions {
    pub length: u8,
    pub include_symbols: bool,
    pub include_numbers: bool,
    pub include_caps: bool,
}

#[tauri::command]
pub fn generate_strong_password(options: GeneratorOptions) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let mut character_set: Vec<u8> = LOWERCASE.to_vec();

    if options.include_caps {
        character_set.extend_from_slice(UPPERCASE);
    }
    if options.include_numbers {
        character_set.extend_from_slice(NUMBERS);
    }
    if options.include_symbols {
        character_set.extend_from_slice(SYMBOLS);
    }

    if character_set.is_empty() || options.length == 0 {
        return Err("Password length must be greater than 0, and at least one character set must be selected.".to_string());
    }

    let password: String = (0..options.length)
        .map(|_| {
            *character_set.choose(&mut rng).unwrap() as char
        })
        .collect();

    Ok(password)
}