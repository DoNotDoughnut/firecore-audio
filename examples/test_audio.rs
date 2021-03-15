use firecore_audio_lib::serialized::SerializedAudio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("examples/audio.bin")?;
    let ser_audio: SerializedAudio = bincode::deserialize(&bytes)?;
    println!("{}", ser_audio.music.len());
    firecore_audio::create();
    firecore_audio::load(ser_audio);
    firecore_audio::play_music_named("IGF");
    std::thread::sleep(std::time::Duration::from_millis(500));
    // let mut audio = firecore_audio::backend::kira::music::MUSIC_MAP.iter_mut().next().unwrap();
    // let handle = audio.1.play(kira::instance::InstanceSettings::default())?;
    // while handle.state() == kira::instance::InstanceState::Playing {
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    // }
    Ok(())
}