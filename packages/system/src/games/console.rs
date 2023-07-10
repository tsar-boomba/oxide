use enum_iterator::Sequence;
use fixed_map::Key;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Key, Sequence)]
pub enum Console {
    NES,
    Snes,
    GB,
    GBC,
    GBA,
}

impl Console {
    pub fn iter() -> enum_iterator::All<Self> {
        enum_iterator::all()
    }

    pub fn from_str(str: impl AsRef<str>) -> Option<Self> {
        let str = str.as_ref();
        match str {
            "fc" => Some(Self::NES),
            "sfc" => Some(Self::Snes),
            "gb" => Some(Self::GB),
            "gbc" => Some(Self::GBC),
            "gba" => Some(Self::GBA),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::NES => "NES",
            Self::Snes => "SNES",
            Self::GB => "GameBoy",
            Self::GBC => "GB Color",
            Self::GBA => "GBA",
        }
    }

    pub fn default_core(&self) -> &str {
        match self {
            Self::GB => "gambatte",
            Self::NES => "",
            Self::Snes => "snes9x2010",
            Self::GBC => "gambatte",
            Self::GBA => "vbam",
        }
    }
}
