pub mod tuning;
pub use tuning::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    RightHanded,
    LeftHanded,
}

// Because it holds Tuning<'a>, must be <'a>
pub struct FretBoardConfig<'a> {
    pub tuning: Tuning<'a>,
    pub fret_count: u8,
    pub orientation: Orientation,
}

impl<'a> FretBoardConfig<'a> {
    pub fn new(tuning: Tuning<'a>, fret_count: u8, orientation: Orientation) -> Self {
        Self {
            tuning,
            fret_count,
            orientation,
        }
    }
}
