// BPM timing clock
// Simple helpers for BPM -> step duration calculations.

pub fn step_duration_millis(bpm: u32, steps_per_beat: u32) -> u64 {
    // one beat = 60_000 ms. If steps_per_beat is 4 (16th notes), we divide further.
    let ms_per_beat = 60_000u64 / bpm as u64;
    ms_per_beat / steps_per_beat as u64
}
