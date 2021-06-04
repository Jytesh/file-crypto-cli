use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};

pub fn bool(input: &str) -> Result<bool, std::io::Error> {
    Confirm::with_theme(&ColorfulTheme::default())
        .default(true)
        .with_prompt(input)
        .interact()
}

pub fn input(input: &str) -> Result<String, std::io::Error> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(input)
        .interact_text()
}

pub fn input_with_default(input: &str, def: &str) -> Result<String, std::io::Error> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .default(def.to_string())
        .show_default(true)
        .with_prompt(input)
        .interact_text()
}

pub fn pass(input: &str) -> Result<String, std::io::Error> {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt(input)
        .allow_empty_password(false)
        .interact()
}
