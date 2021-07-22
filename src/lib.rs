static ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMS: &str = "0123456789";
static SPECIAL: &str = "!@#$%^&*_-+=";

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
    pub filename: String,
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

fn gen_pass(length: u32, chars: String, filename: String) -> String {
    let mut password = String::new();

    for _ in 0..length {
        let random = rand::thread_rng().gen_range(0..chars.len());
        let char_at_rand_index = &(chars.chars().collect::<Vec<char>>()[random]).to_string();
        password.push_str(char_at_rand_index);
    }

    if filename != "" {
        save_pass(filename, password.clone());
    }

    password
}

fn save_pass(filename: String, password: String) {
    let mut color_config = ColorConfig::new(Color::Magenta);
    color_config.set_is_bold(true);
    let mut stdout = std_color(&color_config);

    if filename != "".to_string() {
        writeln!(&mut stdout, "Saved password to `{}`!", filename).unwrap();
    };

    match fs::write(filename, password) {
        Ok(()) => (),
        Err(e) => {
            color_config.set_fg(Color::Red);
            writeln!(&mut stdout, "Error while writing to file: {}", e).unwrap();

            std::process::exit(1);
        }
    };
}

pub fn run(config: Vec<String>) {
    let (mut has_nums, mut has_symbols, mut length, mut filename) = (true, true, 8, String::new());

    for (index, arg) in config.iter().enumerate() {
        match &arg[..] {
            "--no-nums" | "-nn" => {
                has_nums = false;
            }
            "--no-symbols" | "-ns" => {
                has_symbols = false;
            }
            "--out" | "-o" => {
                filename = config[index + 1].to_string();
            }
            "--len" | "-l" => {
                length = config[index + 1].parse().unwrap();
            }
            "--help" | "-h" => {
                println!("{}", fs::read_to_string("src/help_message.txt").unwrap());
                std::process::exit(1);
            }
            _ => continue,
        };
    }

    let mut color_config = ColorConfig::new(Color::Green);
    color_config.set_is_bold(true);
    let password = create_pass(PassConfig {
        length,
        has_nums,
        has_symbols,
        filename,
    });
    let mut stdout = std_color(&color_config);
    writeln!(&mut stdout, "Generated Password:",).unwrap();
    color_config.set_fg(Color::Blue).set_is_bold(false);
    let mut stdout = std_color(&color_config);
    writeln!(&mut stdout, "{}", password).unwrap();
}
