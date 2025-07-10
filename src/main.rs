use clap::Parser;

use crate::{cli::Args, markov::Markov, tts::text_to_speech};

mod cli;
mod markov;
mod tts;

fn main() {
    let args = Args::parse();

    let markov = Markov::new(args.order, args.words, args.files);

    let text = markov.generate_text(args.length);

    println!("{text}");

    text_to_speech(text.clone());
}
