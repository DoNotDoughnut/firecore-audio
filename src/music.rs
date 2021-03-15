use dashmap::DashMap;
use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::music::{MusicId, MusicName};

lazy_static::lazy_static! {
    pub static ref MUSIC_ID_MAP: DashMap<MusicName, MusicId> = DashMap::new();
}

pub fn add_track(music_data: SerializedMusicData) {
    MUSIC_ID_MAP.insert(music_data.music.name.clone(), music_data.music.track);
    #[cfg(not(target_arch = "wasm32"))]
    crate::backend::kira::context::add_track(music_data);
}

pub fn get_music_id(name: &str) -> Option<MusicId> {
    MUSIC_ID_MAP.get(name).map(|id| *id.value())
}

pub fn play_music_id(id: MusicId) {
    #[cfg(not(target_arch = "wasm32"))]
    crate::backend::kira::music::play_music(id);
}

pub fn play_music_named(name: &str) {
    match get_music_id(&name.to_string()) {
        Some(id) => {
            play_music_id(id);
        }
        None => {

        }
    }
    
}

pub fn get_current_music() -> Option<MusicId> {
    #[cfg(not(target_arch = "wasm32"))] {
        crate::backend::kira::music::get_current_music()
    }
    #[cfg(target_arch = "wasm32")] {
        None
    }
}

// pub const fn included_bytes(music: &Music) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
//     match music {
//         Music::IntroGamefreak => Some(include_bytes!("../music/gamefreak.ogg")),
//         Music::Title => Some(include_bytes!("../music/title.ogg")),
//         Music::Pallet => Some(include_bytes!("../music/pallet.ogg")),
//         Music::EncounterBoy => Some(include_bytes!("../music/encounter_boy.ogg")),
//         Music::EncounterGirl => Some(include_bytes!("../music/encounter_girl.ogg")),
//         Music::BattleWild => Some(include_bytes!("../music/vs_wild.ogg")),
//         Music::BattleTrainer => Some(include_bytes!("../music/vs_trainer.ogg")),
//         Music::BattleGym => Some(include_bytes!("../music/vs_gym.ogg")),
//         Music::Pewter => Some(include_bytes!("../music/pewter.ogg")),
//         Music::Route1 => Some(include_bytes!("../music/route1.ogg")),
//         Music::Route2 => Some(include_bytes!("../music/route2.ogg")),
//         Music::Fuchsia => Some(include_bytes!("../music/fuchsia.ogg")),
//         Music::Lavender => Some(include_bytes!("../music/lavender.ogg")),
//         Music::Celadon => Some(include_bytes!("../music/celadon.ogg")),
//         Music::Cinnabar => Some(include_bytes!("../music/cinnabar.ogg")),
//         Music::Vermilion => Some(include_bytes!("../music/vermilion.ogg")),
//         Music::Route3 => Some(include_bytes!("../music/route3.ogg")),
//         Music::Route4 => Some(include_bytes!("../music/route4.ogg")),
//         Music::MountMoon => Some(include_bytes!("../music/mt_moon.ogg")),
//         Music::Gym => Some(include_bytes!("../music/gym.ogg")),
//         Music::ViridianForest => Some(include_bytes!("../music/viridian_forest.ogg")),
//         Music::EncounterRival => Some(include_bytes!("../music/encounter_rival.ogg")),
//         Music::Oak => Some(include_bytes!("../music/oak.ogg")),
//         Music::OaksLab => Some(include_bytes!("../music/oaks_lab.ogg")),
//     }
// }