use super::progress_state::*;

use std::sync::atomic::*;

//
// ConEmuProgressStateTracker
//

/// ConEmu progress state tracker (OSC 9;4).
///
/// [Documentation](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC).
///
/// Will send [ConEmuProgressState::Remove] on [Drop], but you may want to make sure that it is
/// sent when your program exits abnormally. For example:
///
/// ```
/// kutil::std::exit::on_exit(|| ConEmuProgressState::Remove.write(0));
/// ```
#[derive(Debug, Default)]
pub struct ConEmuProgressStateTracker {
    /// Current progress. Must be <= size.
    pub progress: AtomicU64,

    /// Size.
    pub size: AtomicU64,
}

impl ConEmuProgressStateTracker {
    /// Start.
    pub fn start(&self, size: u64) {
        self.size.store(size, Ordering::Relaxed);
        ConEmuProgressState::Percent.write(0);
    }

    /// Add to and update current progress.
    pub fn add(&self, count: u64) {
        self.progress.fetch_add(count, Ordering::Relaxed);
        self.update();
    }

    /// Update current progress.
    pub fn update(&self) {
        let progress: u64 = self.progress.load(Ordering::Relaxed);
        let size: u64 = self.size.load(Ordering::Relaxed);
        let percent = progress * 100 / size;
        ConEmuProgressState::Percent.write(percent as u8);
    }
}

impl Drop for ConEmuProgressStateTracker {
    fn drop(&mut self) {
        ConEmuProgressState::Remove.write(0);
    }
}
