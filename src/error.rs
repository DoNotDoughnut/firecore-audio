use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum SetupError {
    #[cfg(not(target_arch = "wasm32"))]
    SetupError(kira::manager::error::SetupError),
}

#[derive(Debug)]
pub enum AddAudioError {
    NoManager,
    #[cfg(not(target_arch = "wasm32"))]
    DecodeError(kira::sound::error::SoundFromFileError),
    #[cfg(target_arch = "wasm32")]
    DecodeError(Box<dyn core::fmt::Debug>),
    #[cfg(not(target_arch = "wasm32"))]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[derive(Debug)]
pub enum PlayAudioError {
    Missing,
    CurrentLocked,
    #[cfg(not(target_arch = "wasm32"))]
    CurrentError(kira::instance::handle::InstanceHandleError),
    #[cfg(not(target_arch = "wasm32"))]
    PlayError(kira::sound::handle::SoundHandleError),
    #[cfg(target_arch = "wasm32")]
    PlayError(),
}

impl Error for SetupError {}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::SetupError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoManager => write!(f, "Audio manager could not be found. (You probably forgot to initialize it)"),
            Self::DecodeError(err) => write!(f, "{}", err),
            #[cfg(not(target_arch = "wasm32"))]
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
            PlayAudioError::CurrentError(err) => write!(f, "Could not stop audio instance with error {}", err),
            PlayAudioError::PlayError(err) => write!(f, "Could not play audio with error {}", err)
        }
    }
}