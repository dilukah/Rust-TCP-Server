use rand::{Rng, distributions::Alphanumeric};
use std::{fs, path::Path};

pub fn load_or_create_token() -> String {
    let path = "config/settings.toml";

    // create folder if missing
    if !Path::new("config").exists() {
        std::fs::create_dir("config").unwrap();
    }

    // load if exists
    if Path::new(path).exists() {
        let content = fs::read_to_string(path).unwrap();
        let config: toml::Value = toml::from_str(&content).unwrap();
        return config["auth"]["token"].as_str().unwrap().to_string();
    }

    // otherwise generate token and save
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let file_content = format!("[auth]\ntoken=\"{}\"", token);
    fs::write(path, file_content).unwrap();

    println!("No token found — generated new config/settings.toml");
    println!("Token: {}", token);

    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_generated_is_32_chars() {
        let token = load_or_create_token();
        assert_eq!(token.len(), 32);
    }
}