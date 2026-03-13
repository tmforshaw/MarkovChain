#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

use clap::Parser;

use crate::{cli::Args, markov::Markov, tts::text_to_speech};

mod cli;
mod markov;
mod tts;

fn main() {
    let args = Args::parse();

    let markov = Markov::new(args.order, args.words, args.punctuation, args.files);

    let text = markov.generate_text(args.length, args.temperature);

    println!("{text}");

    if args.tts {
        text_to_speech(text);
    }
}
