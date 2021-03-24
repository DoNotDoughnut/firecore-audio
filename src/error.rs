use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum SetupError {
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    SetupError(kira::manager::error::SetupError),
}

#[derive(Debug)]
pub enum AddAudioError {
    NoManager,
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    DecodeError(kira::sound::error::SoundFromFileError),
    // #[cfg(target_arch = "wasm32")]
    // DecodeError(Box<dyn core::fmt::Debug>),
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[derive(Debug)]
pub enum PlayAudioError {
    Missing,
    CurrentLocked,
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    CurrentError(kira::instance::handle::InstanceHandleError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
    PlayError(kira::sound::handle::SoundHandleError),
    #[cfg(target_arch = "wasm32")]
    PlayError(),
}

impl Error for SetupError {}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
        //     Self::SetupError(err) => write!(f, "{}", err),
        //     _ => write!(f, "SetupError unknown"),
        // }
        std::fmt::Debug::fmt(&self, f)
    }
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoManager => write!(f, "Audio manager could not be found. (You probably forgot to initialize it)"),
            #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
            Self::DecodeError(err) => write!(f, "{}", err),
            #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
            Self::ManagerAddError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayAudioError::Missing => write!(f, "Could not find music with specified id!"),
            PlayAudioError::CurrentLocked => write!(f, "Could not unlock current music mutex!"),
            #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
            PlayAudioError::CurrentError(err) => write!(f, "Could not stop audio instance with error {}", err),
            #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
            PlayAudioError::PlayError(err) => write!(f, "Could not play audio with error {}", err)
        }
    }
}