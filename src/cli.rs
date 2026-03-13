use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The order of each Markov chain
    #[arg(short, long, default_value_t = 4)]
    pub order: usize,

    /// Whether to use words (if false, chars are used)
    #[arg(short, long)]
    pub words: bool,

    /// The length (in tokens) of the output
    #[arg(short, long, default_value_t = 500)]
    pub length: usize,

    /// The temperature of the generation
    #[arg(short, long, default_value_t = 1.0)]
    pub temperature: f64,

    /// Choose to keep punctuation or not
    #[arg(short, long)]
    pub punctuation: bool,

    /// Choose to use TTS or not
    #[arg(long)]
    pub tts: bool,

    /// Files to use as data for the Markov chains
    #[arg()]
    pub files: Vec<String>,
}
