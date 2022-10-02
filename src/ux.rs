use dialoguer::{theme::ColorfulTheme, Input};

pub fn input(input: &str) -> Result<String, std::io::Error> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(input)
        .interact_text()
}
