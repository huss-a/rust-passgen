use passgen::{
    create_pass,
    std_color::{std_color, ColorConfig},
};
use std::{env, fs, io::Write};
use termcolor::Color;

#[allow(unused_must_use)]
fn main() {
    let config = env::args().collect::<Vec<String>>();
    let (mut has_nums, mut has_symbols, mut length, mut filename) = (true, true, 8, &String::new());

    for (index, arg) in config.iter().enumerate() {
        match &arg[..] {
            "--no-nums" | "-nn" => {
                has_nums = false;
            }
            "--no-symbols" | "-ns" => {
                has_symbols = false;
            }
            "--out" | "-o" => {
                filename = &config[index + 1];
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

    let mut stdout_color = ColorConfig {
        fg: Color::Green,
        is_bold: true,
        is_dimmed: false,
    };

    let mut stdout = std_color(&stdout_color);
    writeln!(&mut stdout, "Generated Password:",);
    stdout_color.fg = Color::Blue;
    let mut stdout = std_color(&stdout_color);
    writeln!(
        &mut stdout,
        "{}",
        create_pass(length, has_nums, has_symbols, filename)
    );
    stdout_color.fg = Color::Magenta;
    let mut stdout = std_color(&stdout_color);
    if filename != "" {
        writeln!(&mut stdout, "Saved password to `{}`!", filename);
    };
}
