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

pub fn create_pass(length: u32, has_nums: bool, has_symbols: bool, filename: &String) -> String {
    let mut chars = ALPHA.to_string();
    if has_nums {
        chars.push_str(NUMS);
    }
    if has_symbols {
        chars.push_str(SPECIAL);
    }

    gen_pass(length, chars, filename)
}

fn gen_pass(length: u32, chars: String, filename: &String) -> String {
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

fn save_pass(filename: &String, password: String) {
    match fs::write(filename, password) {
        Ok(()) => (),
        Err(e) => {
            let mut stdout = std_color(&ColorConfig {
                fg: Color::Red,
                is_bold: true,
                is_dimmed: false,
            });
            writeln!(&mut stdout, "Error while writing to file: {}", e);

            std::process::exit(1);
        }
    };
}
