use firecore_util::music::Music;

pub const fn loop_start(music: &Music) -> Option<f64> {
    match music {
        Music::BattleWild => Some(13.15),
        _ => None,
    }
}

pub const fn file_name(music: &Music) -> &'static str {
    match music {

        Music::IntroGamefreak => "gamefreak",
        Music::Title => "title",

        Music::Pallet => "pallet",
        Music::Pewter => "pewter",
        Music::Fuchsia => "fuchsia",
        Music::Lavender => "lavender",
        Music::Celadon => "celadon",
        Music::Cinnabar => "cinnabar",
        Music::Vermilion => "vermilion",

        Music::Route1 => "route1",
        Music::Route2 => "route2",
        Music::Route3 => "route3",
        Music::Route4 => "route4",

        Music::Gym => "gym",
        Music::ViridianForest => "viridian_forest",
        Music::MountMoon => "mt_moon",

        
        Music::EncounterBoy => "encounter_boy",
        Music::EncounterGirl => "encounter_girl",

        Music::BattleWild => "vs_wild",
        Music::BattleTrainer => "vs_trainer",
        Music::BattleGym => "vs_gym",
        
    }
}

pub const fn included_bytes(music: &Music) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
    let mut bytes: Option<&[u8]> = match music {
        Music::IntroGamefreak => Some(include_bytes!("../music/gamefreak.ogg")),
        Music::Title => Some(include_bytes!("../music/title.ogg")),
        Music::Pallet => Some(include_bytes!("../music/pallet.ogg")),
        Music::EncounterBoy => Some(include_bytes!("../music/encounter_boy.ogg")),
        Music::BattleWild => Some(include_bytes!("../music/vs_wild.ogg")),
        Music::BattleTrainer => Some(include_bytes!("../music/vs_trainer.ogg")),
        Music::BattleGym => Some(include_bytes!("../music/vs_gym.ogg")),
        Music::Pewter => Some(include_bytes!("../music/pewter.ogg")),
        Music::Route1 => Some(include_bytes!("../music/route1.ogg")),
        Music::Route2 => Some(include_bytes!("../music/route2.ogg")),
        _ => None,
    };
    #[cfg(any(debug_assertions, target_arch = "wasm32"))] {
        if bytes.is_none() {
            bytes = match music {
                Music::Fuchsia => Some(include_bytes!("../music/fuchsia.ogg")),
                Music::Lavender => Some(include_bytes!("../music/lavender.ogg")),
                Music::Celadon => Some(include_bytes!("../music/celadon.ogg")),
                Music::Cinnabar => Some(include_bytes!("../music/cinnabar.ogg")),
                Music::Vermilion => Some(include_bytes!("../music/vermilion.ogg")),
                Music::Route3 => Some(include_bytes!("../music/route3.ogg")),
                Music::Route4 => Some(include_bytes!("../music/route4.ogg")),
                Music::MountMoon => Some(include_bytes!("../music/mt_moon.ogg")),
                Music::Gym => Some(include_bytes!("../music/gym.ogg")),
                Music::EncounterGirl => Some(include_bytes!("../music/encounter_girl.ogg")),
                Music::ViridianForest => Some(include_bytes!("../music/viridian_forest.ogg")),
                _ => None,
            }
        }
    }
    
    bytes
}