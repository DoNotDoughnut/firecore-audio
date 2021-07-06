use serde::{Deserialize, Serialize};

use crate::Audio;

pub type MusicId = u8;
pub type MusicName = String;

pub type Music = Audio<MusicId>;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MusicData {

    pub loop_start: Option<f64>,

}