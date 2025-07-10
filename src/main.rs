use crate::{markov::Markov, tts::text_to_speech};

mod markov;
mod tts;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 3 {
        let order = args[1].parse().unwrap();
        let use_words = args[2].parse().unwrap();

        let files = args[3..args.len()].to_vec();

        let markov = Markov::new(order, use_words, files);

        let text = markov.generate_text(2000);

        println!("{text}");

        text_to_speech(text.clone());
    }
}
