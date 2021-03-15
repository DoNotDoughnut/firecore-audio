use firecore_audio_lib::serialized::SerializedAudio;

mod music;
mod sound;

pub mod backend;

pub use music::{add_track, get_music_id, play_music_id, play_music_named, get_current_music};
pub use sound::{add_sound, play_sound};

// pub mod error;

pub fn create() {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(err) = backend::kira::context::create() {
        // macroquad::prelude::warn!("Could not create Kira audio manager with error {}", err);
    }
    // #[cfg(target_arch = "wasm32")]
    // backend::quadsnd::bind_gamefreak();
}

pub fn load(data: SerializedAudio) {
    for music_data in data.music {
        add_track(music_data);
    }
    for sound_data in data.sounds {
        add_sound(sound_data);
    }
}