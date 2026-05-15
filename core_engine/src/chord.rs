use serde::Deserialize;

// A chord template defined in JSON file chords.json
#[derive(Debug, Deserialize, PartialEq)]
pub struct ChordTemplate {
    pub name: String,
    pub intervals: Vec<u8>,
}

// Holds all locally defined chords

pub struct ChordDatabase {
    pub templates: Vec<ChordTemplate>,
}

impl ChordDatabase {
    // Load JSON at compile time
    pub fn load() -> Self {
        let json_data = include_str!("data/chords.json");

        // Convert string to ChordTemplate
        let templates: Vec<ChordTemplate> =
            serde_json::from_str(json_data).expect("Failed to parse chords.json");

        Self { templates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_load_database() {
        let db = ChordDatabase::load();
        // Verify it loaded the correct amount of chords
        assert_eq!(db.templates.len(), 7);

        // Verify the first chord is a fifth chord
        assert_eq!(db.templates[0].name, "5");
        assert_eq!(db.templates[0].intervals, vec![0, 7]);

        // Verify the second chord is Major
        assert_eq!(db.templates[1].name, "Major");
        assert_eq!(db.templates[1].intervals, vec![0, 4, 7]);
    }
}
