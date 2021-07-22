use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
pub struct ColorConfig {
    pub fg: Color,
    pub is_bold: bool,
    pub is_dimmed: bool,
}

/// returns a `StandardStream` instance with your `ColorConfig`s
///
/// ### Example
/// ```
/// use minigrep::{std_color, ColorConfig};
/// use termcolor::Color;
/// use std::io::Write;
///
/// let mut stdout = std_color(ColorConfig {
///     fg: Color::Red,
///     is_bold: true,
///     is_dimmed: false,
/// });
///
/// writeln!(&mut stdout, "Hello!!"); // Red and bold output!
/// ```
pub fn std_color(color_config: &ColorConfig) -> StandardStream {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(color_config.fg))
                .set_bold(color_config.is_bold)
                .set_dimmed(color_config.is_dimmed),
        )
        .expect("Failed to change terminal color!");

    stdout
}
