use clap::ValueEnum;
use std::fmt::Write;

#[derive(Debug, ValueEnum, Clone)]
pub enum OutputFormat {
    SpacedHexdump,
    Hexdump,
    Escaped,
    C,
    Python,
}

impl OutputFormat {
    pub fn format_sequence(&self, what: &[u8]) -> Result<String, std::fmt::Error> {
        match self {
            Self::SpacedHexdump => format_sequence(what, "", "", " ", |x| format!("{x:02X}")),
            Self::Hexdump => format_sequence(what, "", "", "", |x| format!("{x:02X}")),
            Self::Escaped => format_sequence(what, "", "", "", |x| format!("\\x{x:02x}")),
            Self::C => format_sequence(what, "{ ", " }", ", ", |x| format!("0x{x:02x}")),
            Self::Python => format_sequence(what, "b'", "'", "", |x| format!("\\x{x:02x}")),
        }
    }
}

fn format_sequence(
    data: &[u8],
    start: &str,
    end: &str,
    join: &str,
    byte_formatter: impl Fn(u8) -> String,
) -> Result<String, std::fmt::Error> {
    // It can be reasonably expected that the result will be at least 2 characters for each byte
    let mut result = String::with_capacity(2 * data.len());

    result.write_str(start)?;

    let mut iter = data.iter().peekable();

    while let Some(b) = iter.next() {
        result.write_str(&byte_formatter(*b))?;
        if iter.peek().is_some() {
            result.write_str(join)?;
        }
    }

    result.write_str(end)?;

    Ok(result)
}
