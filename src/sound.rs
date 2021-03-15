use firecore_audio_lib::serialized::SerializedSoundData;
use firecore_audio_lib::sound::Sound;

pub fn play_sound(sound: Sound) {
    // macroquad::prelude::info!("Playing sound {:?}", sound);
    #[cfg(not(target_arch = "wasm32"))]
    super::backend::kira::sound::play_sound(&sound)
}

pub fn add_sound(sound_data: SerializedSoundData) {
    #[cfg(not(target_arch = "wasm32"))]
    super::backend::kira::context::add_sound(sound_data);
}