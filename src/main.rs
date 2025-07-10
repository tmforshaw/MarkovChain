use crate::{markov::Markov, tts::text_to_speech};

mod markov;
mod tts;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        let n_gram = args[1].parse().unwrap();

        let files = args[2..args.len()].to_vec();

        let markov = Markov::new(files, n_gram);

        let text = markov.generate_text(2000);

        println!("{text}");

        text_to_speech(text.clone());
    }
}
