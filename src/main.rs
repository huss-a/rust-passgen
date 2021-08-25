use passgen::*;
use std::{env, io::Write};
use stdout_color::*;
use termcolor::Color;

fn main() {
    let config = env::args().collect::<Vec<String>>();
    let (mut has_nums, mut has_symbols, mut length, mut filename, help_message) =
        (true, true, 8, None, include_str!("help_message.txt"));

    for (index, arg) in config.iter().enumerate() {
        match &arg[..] {
            "--no-nums" | "-nn" => {
                has_nums = false;
            }
            "--no-symbols" | "-ns" => {
                has_symbols = false;
            }
            "--out" | "-o" => {
                filename = Some(config[index + 1].to_string());
            }
            "--len" | "-l" => {
                length = config[index + 1].parse().unwrap();
            }
            "--help" | "-h" => {
                println!("{}", help_message);
                std::process::exit(0);
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
