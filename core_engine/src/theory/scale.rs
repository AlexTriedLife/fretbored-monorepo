use crate::theory::Note;
use crate::theory::interval::{Interval, calculate_target_note};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
// Major, minor, and pentatonic for now
// TODO: Add modes and different scales
pub enum ScaleType {
    Major,
    NaturalMinor,
    MajorPentatonic,
    MinorPentatonic,
}

impl ScaleType {
    // Returns a static slice containing the intervals of a scale relative to the root note
    pub fn intervals(&self) -> &'static [Interval] {
        match self {
            ScaleType::Major => &[
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MajorSixth,
                Interval::MajorSeventh,
            ],
            ScaleType::NaturalMinor => &[
                Interval::MajorSecond,
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSixth,
                Interval::MinorSeventh,
            ],
            ScaleType::MajorPentatonic => &[
                Interval::MajorSecond,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MajorSixth,
            ],
            ScaleType::MinorPentatonic => &[
                Interval::MinorThird,
                Interval::PerfectFourth,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ],
        }
    }
}

// Returns generated scale as a vector of note enums
pub fn generate_scale(root: Note, scale_type: ScaleType) -> Vec<Note> {
    // Retrieve the interval/step pattern of the scale type
    let intervals = scale_type.intervals();

    // Dynamically allocate capacity (rootnote + number of intervals in scale)
    let mut scale = Vec::with_capacity(intervals.len() + 1);

    // Add the root note to beginning of scale
    scale.push(root);

    // Calculate following notes and add to scale
    // Deref interval
    for &interval in intervals {
        scale.push(calculate_target_note(root, interval));
    }

    scale
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::theory::Note;

    #[test]
    fn test_generate_major_scale() {
        // Generate a C Major scale
        let c_major = generate_scale(Note::C, ScaleType::Major);

        // Expected: C, D, E, F, G, A, B
        assert_eq!(
            c_major,
            vec![
                Note::C,
                Note::D,
                Note::E,
                Note::F,
                Note::G,
                Note::A,
                Note::B,
            ]
        );

        // Generate a A#/Bb Major scale
        let asharp_major = generate_scale(Note::ASharp, ScaleType::Major);

        // Expected: A#, C, D, D#, F, G, A
        assert_eq!(
            asharp_major,
            vec![
                Note::ASharp,
                Note::C,
                Note::D,
                Note::DSharp,
                Note::F,
                Note::G,
                Note::A,
            ]
        );
    }

    #[test]
    fn test_generate_natural_minor_scale() {
        // Generate an A Natural Minor scale
        let a_minor = generate_scale(Note::A, ScaleType::NaturalMinor);

        // Expected: A, B, C, D, E, F, G
        assert_eq!(
            a_minor,
            vec![
                Note::A,
                Note::B,
                Note::C,
                Note::D,
                Note::E,
                Note::F,
                Note::G,
            ]
        );

        // Generate a C# Natural Minor scale
        let csharp_minor = generate_scale(Note::CSharp, ScaleType::NaturalMinor);

        // Expected: C#, D#, E, F#, G#, A, B
        assert_eq!(
            csharp_minor,
            vec![
                Note::CSharp,
                Note::DSharp,
                Note::E,
                Note::FSharp,
                Note::GSharp,
                Note::A,
                Note::B,
            ]
        );
    }
    #[test]
    fn test_generate_major_pentatonic_scale() {
        let c_maj_pent = generate_scale(Note::C, ScaleType::MajorPentatonic);

        // Expected: C, D, E, G, A
        assert_eq!(
            c_maj_pent,
            vec![Note::C, Note::D, Note::E, Note::G, Note::A,]
        );
    }

    #[test]
    fn test_generate_minor_pentatonic_scale() {
        let a_min_pent = generate_scale(Note::A, ScaleType::MinorPentatonic);

        // Expected: A, C, D, E, G
        assert_eq!(
            a_min_pent,
            vec![Note::A, Note::C, Note::D, Note::E, Note::G,]
        );
    }
}
