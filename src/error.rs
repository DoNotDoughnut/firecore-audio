use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum SetupError {
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    SetupError(kira::manager::error::SetupError),
}

#[derive(Debug)]
pub enum AddAudioError {
    #[cfg(feature = "play")]
    NoManager,
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    DecodeError(kira::sound::error::SoundFromFileError),
    // #[cfg(target_arch = "wasm32")]
    // DecodeError(Box<dyn core::fmt::Debug>),
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[derive(Debug)]
pub enum PlayAudioError {
    #[cfg(feature = "play")]
    Missing,
    #[cfg(feature = "play")]
    CurrentLocked,
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    CurrentError(kira::instance::handle::InstanceHandleError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    PlayError(kira::sound::handle::SoundHandleError),
    #[cfg(all(target_arch = "wasm32", feature = "play"))]
    PlayError(),
}

impl Error for SetupError {}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "play")] {
            match self {
                Self::NoManager => write!(f, "Audio manager could not be found. (You probably forgot to initialize it)"),
                #[cfg(not(target_arch = "wasm32"))]
                Self::DecodeError(err) => write!(f, "{}", err),
                #[cfg(not(target_arch = "wasm32"))]
                Self::ManagerAddError(err) => write!(f, "{}", err),
            }
        }
        #[cfg(not(feature = "play"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "play")] {
            match self {
                PlayAudioError::Missing => write!(f, "Could not find music with specified id!"),
                PlayAudioError::CurrentLocked => write!(f, "Could not unlock current music mutex!"),
                #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
                PlayAudioError::CurrentError(err) => write!(f, "Could not stop audio instance with error {}", err),
                #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
                PlayAudioError::PlayError(err) => write!(f, "Could not play audio with error {}", err),
                #[cfg(all(target_arch = "wasm32", feature = "play"))]
                PlayAudioError::PlayError() => write!(f, "Could not play audio with error"),
            }
        }
        #[cfg(not(feature = "play"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}