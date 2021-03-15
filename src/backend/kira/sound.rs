use dashmap::DashMap;
use firecore_audio_lib::sound::Sound;
use kira::sound::handle::SoundHandle;

lazy_static::lazy_static! {
    pub static ref SOUND_MAP: DashMap<Sound, SoundHandle> = DashMap::new();
}

pub fn play_sound(sound: &Sound) {
    match SOUND_MAP.get_mut(sound) {
        Some(mut handle) => {
            match handle.value_mut().play(kira::instance::InstanceSettings::default()) {
                Ok(_) => {
                    // return ok
                }
                Err(err) => {
                    // return err
                }
            }
        }
        None => {
            // return error
        }
    }
}