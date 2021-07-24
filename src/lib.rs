static ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMS: &str = "0123456789";
static SPECIAL: &str = "!@#$%^&*_-+=,.~";

mod utils {
    pub mod stdout_color;
}

use rand::Rng;
use std::{fs, io::Write};
use termcolor::Color;
pub use utils::stdout_color;
use utils::stdout_color::*;

pub struct PassConfig {
    pub length: u32,
    pub has_nums: bool,
    pub has_symbols: bool,
    pub filename: Option<String>,
}

pub fn create_pass(config: PassConfig) -> String {
    let PassConfig {
        length,
        has_nums,
        has_symbols,
        filename,
    } = config;

    let mut chars = ALPHA.to_string();
    if has_nums {
        chars.push_str(NUMS);
    }
    if has_symbols {
        chars.push_str(SPECIAL);
    }

    gen_pass(length, chars, filename)
}

fn gen_pass(length: u32, chars: String, filename: Option<String>) -> String {
    let mut password = String::new();
    let char_len = chars.len();
    for _ in 0..length {
        let random = rand::thread_rng().gen_range(0..char_len);
        let char_at_rand_index = chars.chars().nth(random).unwrap();
        password.push(char_at_rand_index);
    }

    if let Some(filename) = filename {
        save_pass(Some(filename), password.clone());
    }

    password
}

fn save_pass(filename: Option<String>, password: String) {
    let mut color_config = ColorConfig::new(Color::Magenta);
    color_config.set_is_bold(true);
    let mut stdout = std_color(&color_config);

    if let Some(name) = filename {
        writeln!(&mut stdout, "Saved password to `{}`!", name).unwrap();
        match fs::write(name, password) {
            Ok(()) => (),
            Err(e) => {
                color_config.set_fg(Color::Red);
                writeln!(&mut stdout, "Error while writing to file: {}", e).unwrap();
                std::process::exit(1);
            }
        };
    };
}
