use crate::instrument::registry::TuningRegistry;
use crate::instrument::{FretBoardConfig, Orientation};
use crate::mapping::find_notes_on_fretboard;
use crate::theory::Note;
// Registry should be singleton
use once_cell::sync::Lazy;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*; // brings in #[wasm_bindgen] macro + JsValue

// Built once, lives for the duration of the WASM module
static REGISTRY: Lazy<TuningRegistry> = Lazy::new(TuningRegistry::new);

#[wasm_bindgen]
pub fn get_note_at_fret(
    tuning_key: &str,
    string_index: usize,
    fret: u8,
) -> Result<JsValue, JsValue> {
    let tuning = REGISTRY
        .get(tuning_key)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown tuning: '{}'", tuning_key)))?;

    tuning
        .note_at_fret(string_index, fret)
        .ok_or_else(|| JsValue::from_str("String index out of range"))
        .and_then(|note| to_value(&note).map_err(|e| JsValue::from_str(&e.to_string())))
}

// Let the frontend discover available tunings without hardcoding them
#[wasm_bindgen]
pub fn list_tunings() -> JsValue {
    let keys = REGISTRY.keys();
    to_value(&keys).unwrap_or(JsValue::NULL)
}

// Get fret and string where given notes appear
#[wasm_bindgen]
pub fn get_fretboard_coordinates(
    tuning_key: &str,
    fret_count: u8,
    note_values: &[u8],
) -> Result<JsValue, JsValue> {
    // Get tuning ref from registry
    let tuning_ref = REGISTRY
        .get(tuning_key)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown tuning: '{}'", tuning_key)))?;

    let config = FretBoardConfig::new(*tuning_ref, fret_count, Orientation::RightHanded);

    // Convert u8 notes to Note enums
    let notes: Vec<Note> = note_values
        .iter()
        .map(|&val| Note::from_midi(val))
        .collect();

    // Find locations of notes on fretboard
    let coords = find_notes_on_fretboard(&notes, &config);

    to_value(&coords).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_tuning_resolves() {
        let registry = TuningRegistry::new();
        assert!(registry.get("guitar/standard").is_some());
        assert!(registry.get("bass/drop_d").is_some());
    }

    #[test]
    fn test_unknown_tuning_returns_none() {
        let registry = TuningRegistry::new();
        assert!(registry.get("banjo/open_g").is_none());
    }

    #[test]
    fn test_all_keys_present() {
        let registry = TuningRegistry::new();
        let keys = registry.keys();
        assert!(keys.contains(&"guitar/standard"));
        assert!(keys.contains(&"ukulele/standard"));
    }
}
