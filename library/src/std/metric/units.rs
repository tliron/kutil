use super::super::string::*;

/// Kilo.
pub const KILO: u64 = 1000;

/// Mega.
pub const MEGA: u64 = 1000 * 1000;

/// Giga.
pub const GIGA: u64 = 1000 * 1000 * 1000;

/// Tera.
pub const TERA: u64 = 1000 * 1000 * 1000 * 1000;

/// Kibi.
pub const KIBI: u64 = 1024;

/// Mebi.
pub const MEBI: u64 = 1024 * 1024;

/// Gibi.
pub const GIBI: u64 = 1024 * 1024 * 1024;

/// Tebi.
pub const TEBI: u64 = 1024 * 1024 * 1024 * 1024;

/// Parse unit.
pub fn parse_metric_unit(representation: &str) -> Result<u64, ParseError> {
    match representation.to_lowercase().as_str() {
        "" | "b" | "byte" | "bytes" => Ok(1),
        "kb" | "kilobyte" | "kilobytes" => Ok(KILO),
        "mb" | "megabyte" | "megabytes" => Ok(MEGA),
        "gb" | "gigabyte" | "gigabytes" => Ok(GIGA),
        "tb" | "terabyte" | "terabytes" => Ok(TERA),
        "kib" | "kibibyte" | "kibibytes" => Ok(KIBI),
        "mib" | "mebibyte" | "mebibytes" => Ok(MEBI),
        "gib" | "gibibyte" | "gibibytes" => Ok(GIBI),
        "tib" | "tebibyte" | "tebibytes" => Ok(TEBI),
        _ => return Err(format!("unsupported unit: {}", representation).into()),
    }
}
