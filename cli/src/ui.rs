use colored::Colorize;
use heigtbitult::keyboard::KeyCode;
use tabled::{
    settings::{object::Rows, themes::Colorization, Alignment, Color, Style},
    Table, Tabled,
};

#[derive(Tabled)]
pub struct KeyBindingRow {
    #[tabled(rename = "Button")]
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
        .map(|(i, &binding)| {
            let name = |key: u8| match KeyCode::from_u8(key) {
                Some(KeyCode::None) => "-".to_string(), // unused slot
                Some(k) => k.name(),                    // known key: show its name
                None => format!("key({:02x})", key),    // unknown: show the raw code
            };
            let [k1, k2, k3, k4] = binding;
            KeyBindingRow {
                button: button_names[i].to_string(),
                key1: name(k1),
                key2: name(k2),
                key3: name(k3),
                key4: name(k4),
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
