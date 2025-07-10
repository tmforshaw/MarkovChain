pub fn text_to_speech(text: String) {
    println!("Doing TTS");
    std::process::Command::new("gtts-cli")
        .arg(text)
        .arg("--lang")
        .arg("en")
        .arg("--output")
        .arg("assets/test.mp3")
        .output()
        .unwrap();

    println!("About to speak");

    let a = std::process::Command::new("mpv")
        .arg("assets/test.mp3")
        .spawn();
}
