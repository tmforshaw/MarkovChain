use crate::{markov::Markov, tts::text_to_speech};

mod markov;
mod tts;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let n_gram = if args.len() > 2 {
            args[2].parse().unwrap()
        } else {
            5
        };

        let markov = Markov::new(args[1].as_str(), n_gram);

        let text = markov.generate_text(2000);

        println!("{text}");

        text_to_speech(text.clone());
    }
}
