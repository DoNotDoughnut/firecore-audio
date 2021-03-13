use kira::manager::AudioManager;
use macroquad::prelude::debug;
use macroquad::prelude::warn;
use parking_lot::Mutex;
use firecore_util::music::MUSIC_LIST;
use firecore_util::music::Music;

use crate::music::included_bytes;

pub mod music;
pub mod sound;

lazy_static::lazy_static! {
    pub static ref AUDIO_CONTEXT: AudioContext = AudioContext::default();
}

#[derive(Default)]
pub struct AudioContext {

    audio_manager: Mutex<Option<AudioManager>>,

}

impl AudioContext {

    pub fn load(&self) -> Result<(), kira::manager::error::SetupError> {
        *self.audio_manager.lock() = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
            Ok(am) => Some(am),
            Err(err) => return Err(err),
        };

        self.bind_gamefreak();
        Ok(())
    }    

    pub fn bind_music(&self) {
        // let mut errors = Vec::new();
        for music in MUSIC_LIST {
            if !self::music::MUSIC_CONTEXT.music_map.contains_key(music) {
                self.bind(music);
            }
        }
    }

    pub fn bind_gamefreak(&self) {
        self.bind(&Music::IntroGamefreak);
    }

    fn bind(&self, music: &Music) {
        match included_bytes(music) {
            Some(bytes) => {
                match super::from_ogg_bytes(bytes, music::settings(music)) {
                    Ok(sound) => match self.audio_manager.lock().as_mut() {
                        Some(manager) => {
                            match manager.add_sound(sound) {
                                Ok(sound) => {
                                    self::music::MUSIC_CONTEXT.music_map.insert(*music, sound);
                                    debug!("Loaded music \"{:?}\" successfully", music);
                                }
                                Err(err) => {
                                    // errors.push(AudioError::AddSoundError(err));
                                    warn!("Problem loading music \"{:?}\" with error {}", music, err);
                                }
                            }
                        }
                        None => {}
                    }
                    Err(err) => {
                        // errors.push(AudioError::DecodeError(err));
                        warn!("Problem decoding bytes of \"{:?}\" in executable with error {}", music, err);
                    }
                }
            }
            None => {
                #[cfg(not(debug_assertions))] {
                    match self.audio_manager.lock().as_mut() {
                        Some(manager) => match manager.load_sound(String::from("music/") + crate::music::file_name(&music) + ".ogg", kira::sound::SoundSettings::default()) {
                            Ok(sound) => {
                                self::music::MUSIC_CONTEXT.music_map.insert(*music, sound);
                                debug!("Loaded \"{:?}\" successfully", music);
                            }
                            Err(err) => {
                                // errors.push(AudioError::LoadSoundError(err));
                                warn!("Problem loading music \"{:?}\" with error {}", music, err);
                            }
                        }
                        None => {
                            // errors.push(AudioError::NoAudioManager);
                            warn!("Could not get audio manager from audio context while loading music \"{:?}\"!", music);
                        }
                    }
                }
            }
        }
    }

}

fn stop_instance(name: impl std::fmt::Debug, mut instance: kira::instance::handle::InstanceHandle) {
    if let Err(err) = instance.stop(kira::instance::StopInstanceSettings::default().fade_tween(kira::parameter::tween::Tween::linear(0.75))) {
        warn!("Problem stopping audio instance {:?} with error {}", name, err);
    }
}