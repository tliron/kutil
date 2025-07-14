use {
    anstream::stderr,
    std::{fs::*, io::*, path::*},
    time::{format_description::*, macros::*},
    tracing::subscriber::*,
    tracing_subscriber::{filter::*, fmt::time::*, prelude::*, *},
};

// RFC 3339 with subseconds
// Or ISO 8601 with fewer subsecond digits
// See: https://time-rs.github.io/book/api/well-known-format-descriptions.html
const TIME_FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour]:[offset_minute]"
);

/// Initialize a tracing subscriber.
///
/// If `path` is [None] will use colorized stderr.
///
/// * 0: no tracing subscriber.
/// * 1: [ERROR](tracing::Level::ERROR)
/// * 2: [WARN](tracing::Level::WARN)
/// * 3: [INFO](tracing::Level::INFO)
/// * 4: [DEBUG](tracing::Level::DEBUG)
/// * >=5: [TRACE](tracing::Level::TRACE)
pub fn initialize_tracing(verbosity: u8, path: Option<&PathBuf>) -> Result<()> {
    if verbosity == 0 {
        return Ok(());
    }

    let level = verbosity_to_level(verbosity);
    let timer = LocalTime::new(TIME_FORMAT);

    let builder = fmt().with_max_level(level).with_timer(timer);

    match path {
        Some(path) => {
            let file = OpenOptions::new().write(true).create(true).append(true).open(path)?;
            builder.with_writer(file).with_ansi(false).init();
        }

        None => builder.with_writer(stderr).init(),
    };

    Ok(())
}

/// Initialize a tracing subscriber for journald.
///
/// * 0: no tracing subscriber.
/// * 1: [ERROR](tracing::Level::ERROR)
/// * 2: [WARN](tracing::Level::WARN)
/// * 3: [INFO](tracing::Level::INFO)
/// * 4: [DEBUG](tracing::Level::DEBUG)
/// * >=5: [TRACE](tracing::Level::TRACE)
pub fn initialize_tracing_journald(verbosity: u8) -> Result<()> {
    if verbosity == 0 {
        return Ok(());
    }

    let level = verbosity_to_level(verbosity);

    let layer = tracing_journald::layer()?;
    let subscriber = registry().with(LevelFilter::from_level(level)).with(layer);
    set_global_default(subscriber).map_err(Error::other)
}

fn verbosity_to_level(verbosity: u8) -> tracing::Level {
    match verbosity {
        1 => tracing::Level::ERROR,
        2 => tracing::Level::WARN,
        3 => tracing::Level::INFO,
        4 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    }
}
