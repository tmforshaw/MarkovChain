use rand::{
    rng,
    seq::{IndexedRandom, SliceRandom},
};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Markov {
    n_gram: usize,
    chains: Vec<HashMap<Vec<char>, Vec<char>>>,
}

impl Markov {
    pub fn new(files: Vec<String>, n_gram: usize) -> Self {
        let mut markov = Self {
            n_gram,
            chains: Vec::new(),
        };

        for file in files {
            // Create a new chain for each file
            let mut chain = HashMap::new();

            // Open a file to use as the data
            let f = File::open(file.as_str()).unwrap();
            let reader = BufReader::new(f);

            let char_iter = reader
                .lines()
                .map_while(Result::ok)
                .flat_map(|line| line.chars().collect::<Vec<_>>());

            // Split the file contents into words
            let mut prev_n = VecDeque::new();
            for token in char_iter {
                // Add to the chain or insert a new entry if the key has no associated chain yet
                if !prev_n.is_empty() {
                    chain
                        .entry(prev_n.clone().into())
                        .or_insert_with(|| vec![token])
                        .push(token);
                }

                // Rotate the previous entries so that there is only ever N_gram amount of entries in the previous N
                if prev_n.len() < markov.n_gram {
                    prev_n.push_back(token);
                } else {
                    prev_n.pop_front();
                    prev_n.push_back(token);
                }
            }

            // Add the chain to the chains Vec
            markov.chains.push(chain);
        }

        markov
    }

    pub fn generate_text(&self, length: usize) -> String {
        // Generate words from a random chain
        let mut rng = rng();
        let mut chain = self.chains.choose(&mut rng).unwrap();
        let keys = chain.keys().cloned().collect::<Vec<_>>();
        let mut key: VecDeque<_> = keys.choose(&mut rng).unwrap().clone().into();

        let mut output: Vec<_> = key.clone().into();

        // let mut chain = HashMap::new();
        for _ in 0..(length - self.n_gram) {
            // Shuffle the indices of the chains, so that, when checking if there is a continuation, we don't check each chain more than once
            let mut shuffled_indices = (0..self.chains.len()).collect::<Vec<_>>();
            shuffled_indices.shuffle(&mut rng);

            // Pick a random chain to continue the generation, choosing only chains which have a continuation
            let mut next_tokens = None;
            for chain_idx in shuffled_indices {
                chain = &self.chains[chain_idx];

                // Chain has a continuation
                if let Some(next_tokens_in_chain) = chain.get(&Into::<Vec<_>>::into(key.clone())) {
                    next_tokens = Some(next_tokens_in_chain);
                    break;
                }
            }

            // Exit the generation if no chains have a continuation
            let Some(next_words) = next_tokens else {
                break;
            };

            // Randomly select a word to come next in the generation
            let Some(next) = next_words.choose(&mut rng) else {
                break;
            };

            // Add the next word to the output
            output.push(*next);

            // Rotate the previous N Vec
            key.pop_front();
            key.push_back(*next);
        }

        // Join the output into a String
        output.into_iter().collect()
    }
}
