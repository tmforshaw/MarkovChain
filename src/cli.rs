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

    /// Files to use as data for the Markov chains
    #[arg()]
    pub files: Vec<String>,
}
