use crate::theory::{MidiNote, Note, note::midi_from_note};

// Since tunings don't change <'a> ...
// Tunings are fixed constants that never change, so there is no reason for heap allocation
// borrow, so data lives in the compiled binary
#[derive(Debug, PartialEq, Eq)]
pub struct Tuning<'a> {
    // Use slices instead of owned String/Vec since tuning data is fixed at runtime
    pub name: &'a str,
    pub strings: &'a [MidiNote],
}

impl<'a> Tuning<'a> {
    // Get the note produce by pressing a specified fret on a specific string
    pub fn note_at_fret(&self, string_index: usize, fret: u8) -> Option<Note> {
        // '?' returns null if out of bounds
        let open_midi = self.strings.get(string_index)?;
        let fretted_midi = open_midi.0 + fret;
        // Get note from midi value of fretted note
        Some(Note::from_midi(fretted_midi))
    }
}

// Standard tuning for an instrument
pub struct StandardTunings;

impl StandardTunings {
    pub fn guitar_standard() -> Tuning<'static> {
        // High to low
        const STRINGS: &[MidiNote] = &[
            midi_from_note(Note::E.value(), 4), // E4
            midi_from_note(Note::B.value(), 3), // B3
            midi_from_note(Note::G.value(), 3), // G3
            midi_from_note(Note::D.value(), 3), // D3
            midi_from_note(Note::A.value(), 2), // A2
            midi_from_note(Note::E.value(), 2), // E2
        ];

        Tuning {
            name: "Standard E",
            strings: STRINGS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guitar_standard_string_count() {
        let tuning = StandardTunings::guitar_standard();
        assert_eq!(tuning.strings.len(), 6);
    }

    #[test]
    fn test_guitar_standard_open_notes() {
        let tuning = StandardTunings::guitar_standard();
        // High to low: E4 B3 G3 D3 A2 E2
        assert_eq!(tuning.strings[0], MidiNote(64)); // E4
        assert_eq!(tuning.strings[5], MidiNote(40)); // E2
    }

    #[test]
    fn test_note_at_fret_open() {
        let tuning = StandardTunings::guitar_standard();
        // Fret 0 = open string, should return the open note
        assert_eq!(tuning.note_at_fret(0, 0), Some(Note::E));
    }

    #[test]
    fn test_note_at_fret_out_of_bounds() {
        let tuning = StandardTunings::guitar_standard();
        assert_eq!(tuning.note_at_fret(99, 0), None);
    }
}
