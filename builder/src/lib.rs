use std::io::Write;

use firecore_audio_lib::serialized::*;

pub fn compile<P: AsRef<std::path::Path>>(music_folder: P, output_file: P) {
    let music_folder = music_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut music = Vec::with_capacity(20);

    for dir in std::fs::read_dir(music_folder).unwrap() {
        let path = dir.unwrap().path();
        if path.is_file() {
            let content = std::fs::read_to_string(&path).unwrap();
            let ser_music_file: SerializedMusicFile = ron::from_str(&content).map_err(|err| format!("File: {:?} had error {}", path, err)).unwrap();
            let music_bytes = std::fs::read(music_folder.join(ser_music_file.file)).unwrap();
            music.push(SerializedMusicData {
                bytes: music_bytes,
                music: ser_music_file.music,
            });
        }
    }

    for curr_music in music.iter().map(|data| &data.music) {
        for music in music.iter().map(|data| &data.music).filter(|music| music.name.ne(&curr_music.name)) {
            if music.track.eq(&curr_music.track) {
                eprintln!("Music {} and {} have equal track IDs! Aborting!", music.name, curr_music.name);
                return;
            }
        }
    }

    let data = firecore_audio_lib::serialized::SerializedAudio {
        music,
        sounds: Vec::new(),
    };

    if let Some(parent) = std::path::Path::new(output_file).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
        }
    }

    let mut file = std::fs::File::create(output_file).unwrap();
    file.write(&postcard::to_allocvec(&data).unwrap()).unwrap();
}