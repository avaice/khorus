use serde::{Deserialize, Serialize};

use crate::keys::Key;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Volumes {
    pub enter: f32,
    pub space: f32,
    pub other: f32,
}

impl Default for Volumes {
    fn default() -> Self {
        Self {
            enter: 1.0,
            space: 1.0,
            other: 1.0,
        }
    }
}

impl Volumes {
    pub fn clamped(self) -> Self {
        let clamp = |value: f32| {
            if value.is_nan() {
                1.0
            } else {
                value.clamp(0.0, 1.0)
            }
        };
        Self {
            enter: clamp(self.enter),
            space: clamp(self.space),
            other: clamp(self.other),
        }
    }

    pub fn for_key(&self, key: Key) -> f32 {
        match key {
            Key::Enter => self.enter,
            Key::Space => self.space,
            Key::Char(_) | Key::Backspace => self.other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_volume_for_each_group() {
        let volumes = Volumes {
            enter: 0.1,
            space: 0.2,
            other: 0.3,
        };
        assert_eq!(volumes.for_key(Key::Enter), 0.1);
        assert_eq!(volumes.for_key(Key::Space), 0.2);
        assert_eq!(volumes.for_key(Key::Backspace), 0.3);
        assert_eq!(volumes.for_key(Key::Char('a')), 0.3);
    }

    #[test]
    fn clamps_out_of_range_values() {
        let volumes = Volumes {
            enter: -1.0,
            space: 5.0,
            other: f32::NAN,
        }
        .clamped();
        assert_eq!(volumes.enter, 0.0);
        assert_eq!(volumes.space, 1.0);
        assert_eq!(volumes.other, 1.0);
    }

    #[test]
    fn missing_fields_fall_back_to_full_volume() {
        let volumes: Volumes = serde_json::from_str(r#"{"space":0.5}"#).unwrap();
        assert_eq!(volumes.space, 0.5);
        assert_eq!(volumes.enter, 1.0);
        assert_eq!(volumes.other, 1.0);
    }
}
