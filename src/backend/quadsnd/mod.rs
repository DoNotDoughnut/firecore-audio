use macroquad::prelude::info;
use macroquad::prelude::warn;
use self::music::MUSIC_MAP;
use crate::music::Music;

pub mod music;

pub async fn bind_world_music() {
    info!("Loading music...");
    for music in crate::music::MUSIC_LIST {
        if !MUSIC_MAP.contains_key(music) {
            match music.included_bytes() {
                Some(bytes) => {
                    read_ogg(*music, bytes);
                }
                None => {
                    let path = String::from("music/") + music.file_name() + ".ogg";
                    match crate::util::file::load_file(&path).await {
                        Ok(bytes) => {
                            read_ogg(*music, &bytes);
                        }
                        Err(err) => {
                            warn!("Could not load music file {:?} at {:?} with error {}", music, &path, err);
                        }
                    }
                }
            }
        }
    }
}

pub fn bind_gamefreak() {
    if let Some(bytes) = Music::IntroGamefreak.included_bytes() {
        read_ogg(Music::IntroGamefreak, bytes);
    }
}

fn read_ogg(music: Music, bytes: &[u8]) {
    match quad_snd::decoder::read_ogg(bytes) {
        Ok(sound) => {
            MUSIC_MAP.insert(music, sound);
            info!("Loaded {:?} successfully", music);
        }
        Err(err) => {
            warn!("Could not read bytes for music \"{:?}\" with error {}", music, err);
        }
    }
}