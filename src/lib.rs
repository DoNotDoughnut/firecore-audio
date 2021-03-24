use error::*;
use firecore_audio_lib::serialized::SerializedAudio;

mod music;
mod sound;

pub mod error;
pub mod backend;

pub use firecore_audio_lib::music::*;
pub use firecore_audio_lib::sound::*;

pub use firecore_audio_lib::serialized::SerializedSoundData;

pub use music::{add_track, get_music_id, play_music_id, play_music_named, get_current_music};
pub use sound::{add_sound, play_sound};

// pub mod error;

pub fn create() -> Result<(), SetupError> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))] {
        if let Err(err) = backend::kira::context::create() {
            Err(SetupError::SetupError(err))
        } else {
            Ok(())
        }
    }
    #[cfg(any(target_arch = "wasm32", not(feature = "kira")))] {
        Ok(())
    }
    
    // #[cfg(target_arch = "wasm32")]
    // backend::quadsnd::bind_gamefreak();
    
}

pub fn load(data: SerializedAudio) -> Result<(), AddAudioError> {
    for music_data in data.music {
        add_track(music_data)?;
    }
    for sound_data in data.sounds {
        add_sound(sound_data)?;
    }
    Ok(())
}