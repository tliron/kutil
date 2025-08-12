use {anstyle::*, clap::builder::*};

/// Styles for Clap.
pub fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Cyan.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Blue.on_default())
}
