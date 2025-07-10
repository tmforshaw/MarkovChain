pub fn text_to_speech(text: String) {
    println!("Doing TTS");
    std::process::Command::new("gtts-cli")
        .arg(text)
        .arg("--lang")
        .arg("en")
        .arg("--output")
        .arg("assets/Text_To_Speech.mp3")
        .output()
        .unwrap();

    println!("About to speak");

    let _ = std::process::Command::new("mpv")
        .arg("assets/Text_To_Speech.mp3")
        .spawn();
}
