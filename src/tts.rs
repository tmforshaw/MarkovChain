pub fn text_to_speech(text: String) {
    println!("Doing TTS");
    std::process::Command::new("gtts-cli")
        .arg("--lang")
        .arg("en")
        .arg("--output")
        .arg("assets/Text_To_Speech.mp3")
        .arg(text)
        .output()
        .unwrap_or_else(|e| panic!("Could not run TTS: {e}"));

    println!("About to speak");

    let mut handle = std::process::Command::new("mpv")
        .arg("assets/Text_To_Speech.mp3")
        .spawn()
        .unwrap_or_else(|e| panic!("Could not play mp3: {e}"));

    handle.wait().unwrap_or_else(|e| panic!("{e}"));
}
