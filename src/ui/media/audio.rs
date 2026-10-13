// SPDX-License-Identifier: MIT
//
// kan-linux: GStreamer audio playback removed (strata kan branch).
// FFmpeg still handles decode/probe/thumbnails via sandbox helpers; this
// stub keeps the PcmOutput type so the media pipeline runs video-only.

use std::cell::Cell;
use std::time::Duration;

pub(super) struct PcmOutput {
    finished: Cell<bool>,
}

impl PcmOutput {
    /// Audio playback is disabled; construction always succeeds as a no-op.
    pub(super) fn new(_muted: bool, _volume: f64, _lookahead: Duration) -> Result<Self, String> {
        Ok(Self {
            finished: Cell::new(false),
        })
    }

    #[cfg(test)]
    pub(super) fn test_sink(
        _muted: bool,
        _volume: f64,
        _lookahead: Duration,
        _description: &str,
        _clock: Option<&()>,
    ) -> Result<Self, String> {
        Self::new(_muted, _volume, _lookahead)
    }

    pub(super) fn push(&self, _data: Vec<u8>, _timestamp_us: u64) -> Result<(), String> {
        Ok(())
    }

    pub(super) fn has_capacity(&self) -> bool {
        !self.finished.get()
    }

    pub(super) fn play(&self) {}

    pub(super) fn pause(&self) {}

    pub(super) fn position_us(&self) -> Option<u64> {
        None
    }

    pub(super) fn set_audio(&self, _muted: bool, _volume: f64) {}

    pub(super) fn set_fade(&self, _fade: f64) {}

    #[cfg(test)]
    pub(super) fn effective_volume(&self) -> f64 {
        0.0
    }

    pub(super) fn finish(&self) -> Result<(), String> {
        self.finished.set(true);
        Ok(())
    }

    pub(super) fn error(&self) -> Option<String> {
        None
    }
}
