pub mod music;
pub mod sound;

pub mod backend;

#[test]
fn main() {
    println!("Test!");
}

// pub mod error;

pub fn create() {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(err) = backend::kira::context::AUDIO_CONTEXT.load() {
        macroquad::prelude::warn!("Could not create Kira audio manager with error {}", err);
    }
    #[cfg(target_arch = "wasm32")]
    backend::quadsnd::bind_gamefreak();
}

pub async fn bind_world_music() {
    #[cfg(not(target_arch = "wasm32"))]
    self::backend::kira::bind_world_music();
    #[cfg(target_arch = "wasm32")]
    self::backend::quadsnd::bind_world_music().await;
}

pub fn play_music(music: music::Music) {
    #[cfg(not(target_arch = "wasm32"))]
    self::backend::kira::context::music::MUSIC_CONTEXT.play_music(music);
    #[cfg(target_arch = "wasm32")]
    self::backend::quadsnd::music::play_music(music);
}

pub fn get_music_playing() -> Option<music::Music> {
    #[cfg(not(target_arch = "wasm32"))]
    return self::backend::kira::context::music::MUSIC_CONTEXT.get_music_playing();
    #[cfg(target_arch = "wasm32")]
    return self::backend::quadsnd::music::get_current_music();
}

pub fn play_sound(sound: sound::Sound) {
    // macroquad::prelude::info!("Playing sound {:?}", sound);
    #[cfg(not(target_arch = "wasm32"))]
    self::backend::kira::context::sound::SOUND_CONTEXT.play_sound(sound)
}

pub fn add_sound(sound: sound::Sound, bytes: &[u8]) {
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