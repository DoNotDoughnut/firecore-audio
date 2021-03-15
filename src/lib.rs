use firecore_audio_lib::serialized::SerializedAudio;
use firecore_util::{sound::Sound};

mod music;
pub mod sound;

pub mod backend;

pub use music::{add_track, get_music_id, play_music_id, play_music_named, get_current_music};

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
}

// pub async fn bind_world_music() {
//     #[cfg(not(target_arch = "wasm32"))]
//     self::backend::kira::bind_world_music();
//     #[cfg(target_arch = "wasm32")]
//     self::backend::quadsnd::bind_world_music().await;
// }

// pub fn play_music(music: Music) {
//     #[cfg(not(target_arch = "wasm32"))]
//     self::backend::kira::context::music::MUSIC_CONTEXT.play_music(music);
//     #[cfg(target_arch = "wasm32")]
//     self::backend::quadsnd::music::play_music(music);
// }

// pub fn get_music_playing() -> Option<Music> {
//     #[cfg(not(target_arch = "wasm32"))]
//     return self::backend::kira::context::music::MUSIC_CONTEXT.get_music_playing();
//     #[cfg(target_arch = "wasm32")]
//     return self::backend::quadsnd::music::get_current_music();
// }

pub fn play_sound(sound: Sound) {
    // macroquad::prelude::info!("Playing sound {:?}", sound);
    #[cfg(not(target_arch = "wasm32"))]
    self::backend::kira::context::sound::SOUND_CONTEXT.play_sound(sound)
}

pub fn add_sound(sound: Sound, bytes: &[u8]) {
    #[cfg(not(target_arch = "wasm32"))]
    backend::kira::context::sound::SOUND_CONTEXT.add_sound(sound, bytes);
}

// pub fn stop_sound(sound: Sound) {
//     let mut instances = SOUND_INSTANCE_MAP.lock();
//     match instances.remove(&sound) {
//         Some(instance) => {
//             stop_instance(sound, instance);
//         },
//         None => warn!("Could not get sound instance handle for {}, probably not playing", sound),
//     }
// }


// pub fn stop_all_sounds() {
//     let sound_keys: Vec<Sound> = SOUND_INSTANCE_MAP.lock().keys().into_iter().map(|music|*music).collect();
//     for sound in sound_keys {
//         stop_sound(sound);
//     }
// }