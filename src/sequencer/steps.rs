// Step grid data
// Step grid utilities and types

pub type Pattern = Vec<Vec<bool>>; // [track][step]

pub fn empty_pattern(tracks: usize, steps: usize) -> Pattern {
    vec![vec![false; steps]; tracks]
}
