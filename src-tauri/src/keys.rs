use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Space,
    Enter,
    Backspace,
}

impl Key {
    pub fn name(self) -> String {
        match self {
            Key::Char(c) => c.to_string(),
            Key::Space => "space".to_string(),
            Key::Enter => "enter".to_string(),
            Key::Backspace => "backspace".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UnknownKey;

impl FromStr for Key {
    type Err = UnknownKey;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "space" => Ok(Key::Space),
            "enter" => Ok(Key::Enter),
            "backspace" => Ok(Key::Backspace),
            _ => {
                let mut chars = name.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => Ok(Key::Char(c.to_lowercase().next().unwrap_or(c))),
                    _ => Err(UnknownKey),
                }
            }
        }
    }
}
