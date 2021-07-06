use serde::{Deserialize, Serialize};

use crate::Audio;

pub type SoundCategory = u16;
pub type SoundVariant = Option<u16>;

pub type Sound = Audio<SoundId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SoundId {
    pub category: SoundCategory,
    pub variant: SoundVariant,
}

impl SoundId {

    pub fn new(category: SoundCategory) -> Self {
        Self {
            category,
            variant: None,
        }
    }

    pub fn variant(category: SoundCategory, variant: SoundVariant) -> Self {
        Self {
            category,
            variant,
        }
    }

}

impl core::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.variant {
            Some(variant) => write!(f, "{} #{}", self.category, variant),
            None => core::fmt::Display::fmt(&self.category, f)
        }
    }
}