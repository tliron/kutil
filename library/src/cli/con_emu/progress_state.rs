use std::{
    cmp::*,
    io::{self, Write},
};

//
// ConEmuProgressState
//

/// ConEmu progress state (OSC 9;4).
///
/// [Documentation](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC).
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConEmuProgressState {
    /// Remove progress bar.
    Remove = 0,

    /// Update progress bar with percentage.
    Percent = 1,

    /// Set progress bar to error state.
    Error = 2,

    /// Set progress bar to indeterminate state.
    Indeterminate = 3,

    /// Pause progress bar.
    Pause = 4,
}

impl ConEmuProgressState {
    /// Write to stderr.
    ///
    /// percent is capped at 100.
    pub fn write(&self, percent: u8) {
        write!(&mut io::stderr(), "\x1b]9;4;{};{}\x07", *self as u8, min(percent, 100)).expect("write ConEmu OSC 9;4");
    }
}
