use firecore_audio_lib::serialized::SerializedMusicData;
use kira::manager::AudioManager;
use parking_lot::Mutex;

pub mod sound;

lazy_static::lazy_static! {
    pub static ref AUDIO_CONTEXT: Mutex<Option<AudioManager>> = Mutex::new(None);
}

pub fn create() -> Result<(), kira::manager::error::SetupError> {
    *AUDIO_CONTEXT.lock() = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
        Ok(am) => Some(am),
        Err(err) => return Err(err),
    };

    Ok(())
} 

// pub fn bind_gamefreak() {
//     bind(&Music::IntroGamefreak);
// }

pub fn add_track(track: SerializedMusicData) {
    match super::from_ogg_bytes(&track.bytes, kira::sound::SoundSettings::default()) {
        Ok(sound) => match AUDIO_CONTEXT.lock().as_mut() {
            Some(manager) => {
                match manager.add_sound(sound) {
                    Ok(sound) => {
                        // println!("Added music");
                        super::music::MUSIC_MAP.insert(track.data.track_id, (track.data.data, sound));
                        // debug!("Loaded music \"{:?}\" successfully", music);
                    }
                    Err(err) => {
                        // eprintln!("{}", err);
                        // errors.push(AudioError::AddSoundError(err));
                        // warn!("Problem loading music \"{:?}\" with error {}", music, err);
                    }
                }
            }
            None => {
                // eprintln!("No audio manager");
            }
        }
        Err(err) => {
            // eprintln!("{}", err);
            // errors.push(AudioError::DecodeError(err));
            // warn!("Problem decoding bytes of \"{:?}\" in executable with error {}", music, err);
        }
    }
}