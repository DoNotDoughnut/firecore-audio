pub enum PlayAudioException {
    
}

#[derive(Debug)]
pub enum SetupError {

    // kira errors

    #[cfg(not(target_arch = "wasm32"))]
    ManagerSetup(kira::manager::error::SetupError),
    #[cfg(not(target_arch = "wasm32"))]
    NoAudioManager,

    #[cfg(not(target_arch = "wasm32"))]
    LoadSound(kira::manager::error::LoadSoundError),
    #[cfg(not(target_arch = "wasm32"))]
    AddSound(kira::manager::error::AddSoundError),

    #[cfg(not(target_arch = "wasm32"))]
    Decode(kira::sound::error::SoundFromFileError),
    #[cfg(target_arch = "wasm32")]
    Decode(),
}

#[derive(Debug)]
pub enum PlayError {
    #[cfg(not(target_arch = "wasm32"))]
    NoAudioManager,
    Missing,
    Locked,
}

impl std::error::Error for PlayError {}

impl std::fmt::Display for PlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PlayError::SetupError(err) => {
                &err.to_string()
            }
            PlayError::NoAudioManager => {
                "Audio manager could not be found"
            },
            PlayError::DecodeError(err) => {
                &err.to_string()
            }
            PlayError::AddSoundError(err) => {
                &err.to_string()
            },
            PlayError::LoadSoundError(err) => {
                &err.to_string()
            }
            PlayError::Missing => {
                "No sound was found"
            },
            PlayError::Inaccessable(string) => {
                &string
            }
        })
    }
}