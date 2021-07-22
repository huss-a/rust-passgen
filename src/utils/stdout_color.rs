use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
pub struct ColorConfig {
    fg: Color,
    is_bold: bool,
    is_dimmed: bool,
}

impl ColorConfig {
    pub fn new(color: Color) -> Self {
        ColorConfig {
            fg: color,
            is_bold: false,
            is_dimmed: false,
        }
    }

    pub fn fg(&self) -> Color {
        self.fg
    }
    pub fn is_bold(&self) -> bool {
        self.is_bold
    }

    pub fn is_dimmed(&self) -> bool {
        self.is_dimmed
    }

    pub fn set_fg(&mut self, val: Color) -> &mut Self {
        self.fg = val;
        self
    }
    pub fn set_is_bold(&mut self, val: bool) -> &mut Self {
        self.is_bold = val;
        self
    }
    pub fn set_is_dimmed(&mut self, val: bool) -> &mut Self {
        self.is_dimmed = val;
        self
    }
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
/// writeln!(&mut stdout, "Hello!!").unwrap(); // Red and bold output!
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
