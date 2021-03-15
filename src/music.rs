use dashmap::DashMap;
use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::music::{MusicId, MusicName};

use crate::error::PlayAudioError;

lazy_static::lazy_static! {
    pub static ref MUSIC_ID_MAP: DashMap<MusicName, MusicId> = DashMap::new();
}

pub fn add_track(music_data: SerializedMusicData) -> Result<(), crate::error::AddAudioError> {
    MUSIC_ID_MAP.insert(music_data.music.name.clone(), music_data.music.track);
    #[cfg(not(target_arch = "wasm32"))]
    crate::backend::kira::context::add_track(music_data)?;
    Ok(())
}

pub fn get_music_id(name: &str) -> Option<MusicId> {
    MUSIC_ID_MAP.get(name).map(|id| *id.value())
}

pub fn play_music_id(id: MusicId) -> Result<(), PlayAudioError> {
    #[cfg(not(target_arch = "wasm32"))]
    crate::backend::kira::music::play_music(id)?;
    Ok(())
}

pub fn play_music_named(name: &str) -> Result<(), PlayAudioError> {
    match get_music_id(&name.to_string()) {
        Some(id) => {
            play_music_id(id)?;
            Ok(())
        }
        None => {
            Err(PlayAudioError::Missing)
        }
    }
    
}

pub fn get_current_music() -> Option<MusicId> {
    #[cfg(not(target_arch = "wasm32"))] {
        crate::backend::kira::music::get_current_music()
    }
    #[cfg(target_arch = "wasm32")] {
        None
    }
}