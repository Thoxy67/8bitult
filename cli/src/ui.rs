use colored::Colorize;
use heigtbitult::keyboard::KeyCode;
use tabled::{
    settings::{object::Rows, themes::Colorization, Alignment, Color, Style},
    Table, Tabled,
};

#[derive(Tabled)]
pub struct KeyBindingRow {
    #[tabled(rename = "Touche")]
    pub button: String,
    #[tabled(rename = "1")]
    pub key1: String,
    #[tabled(rename = "2")]
    pub key2: String,
    #[tabled(rename = "3")]
    pub key3: String,
    #[tabled(rename = "4")]
    pub key4: String,
}

pub fn print_section_header(text: &str) {
    println!("\n{}", "━".repeat(50).bright_black());
    println!("{}", text.bold());
    println!("{}", "━".repeat(50).bright_black());
}

pub fn print_step(text: &str) {
    println!("\n{} {}", "→".bright_cyan(), text);
}

pub fn print_success(text: &str) {
    println!("{} {}", "✓".green(), text.green());
}

pub fn print_warning(text: &str) {
    println!("{} {}", "!".yellow(), text.yellow());
}

pub fn print_bindings(bindings: &[[u8; 4]], button_names: &[&str]) {
    let rows: Vec<KeyBindingRow> = bindings
        .iter()
        .enumerate()
        .map(|(i, binding)| {
            let key_names: Vec<String> = binding
                .iter()
                .map(|&key| match KeyCode::from_u8(key) {
                    Some(KeyCode::None) => "-".to_string(), // Si c'est une touche Null, on affiche "-"
                    Some(k) => k.name(), // Si c'est une touche connue, on affiche son nom
                    None => format!("key({:02x})", key), // Si c'est une touche inconnue, on affiche son code
                })
                .collect();
            KeyBindingRow {
                button: button_names[i].to_string(),
                key1: key_names[0].clone(),
                key2: key_names[1].clone(),
                key3: key_names[2].clone(),
                key4: key_names[3].clone(),
            }
        })
        .collect();

    let mut table = Table::new(rows);
    table
        .with(Style::modern())
        .with(Colorization::columns([
            Color::FG_BRIGHT_WHITE,
            Color::FG_RED,
            Color::FG_GREEN,
            Color::FG_BLUE,
            Color::FG_YELLOW,
        ]))
        .modify(Rows::first(), Alignment::center());

    println!("\n{}", table);
}
