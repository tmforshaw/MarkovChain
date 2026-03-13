use rand::{Rng, rng};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Markov {
    order: usize,
    use_words: bool,
    keep_punctuation: bool,
    chain: HashMap<Vec<String>, HashMap<String, usize>>,
}

impl Markov {
    pub fn new(order: usize, use_words: bool, keep_punctuation: bool, files: Vec<String>) -> Self {
        let mut markov = Self {
            order,
            use_words,
            keep_punctuation,
            chain: HashMap::new(),
        };

        for file in files {
            // Open a file to use as the data
            let f = File::open(file.as_str()).unwrap_or_else(|_| panic!("Could not open file {file}"));
            let reader = BufReader::new(f);

            // Split the file contents into tokens
            let token_iter = reader.lines().map_while(Result::ok).flat_map(|line| {
                // Clean the line up
                let line = line
                    .to_lowercase()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .chars()
                    .filter_map(|c| {
                        if c.is_alphanumeric() || c.is_whitespace() || (markov.keep_punctuation && ",.!?;:".contains(c)) {
                            Some(c)
                        } else if markov.use_words {
                            Some(' ')
                        } else {
                            None
                        }
                    })
                    .collect::<String>();

                // Choose between using words or chars
                if markov.use_words {
                    line.split_whitespace().map(ToString::to_string).collect::<Vec<_>>()
                } else {
                    line.chars().map(|c| c.to_string()).collect::<Vec<_>>()
                }
            });

            // Build a chain from the tokens, keeping track of the N previous
            let mut prev_n = VecDeque::new();
            for token in token_iter {
                // Add to the chain or insert a new entry if the key has no associated chain yet
                if !prev_n.is_empty() {
                    markov
                        .chain
                        .entry(prev_n.clone().into())
                        .or_default()
                        .entry(token.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }

                // Rotate the previous entries so that there is only ever N_gram amount of entries in the previous N
                if prev_n.len() >= markov.order {
                    prev_n.pop_front();
                }

                prev_n.push_back(token);
            }
        }

        markov
    }

    fn weighted_choice_with_temperature(next_words: &HashMap<String, usize>, temperature: f64, rng: &mut impl Rng) -> String {
        let adjusted: Vec<(String, f64)> = next_words
            .iter()
            .map(|(word, &count)| (word.clone(), (count as f64).powf(1.0 / temperature)))
            .collect();

        let total: f64 = adjusted.iter().map(|(_, w)| *w).sum();
        let mut threshold = rng.random_range(0.0..total);

        for (word, weight) in adjusted {
            if threshold < weight {
                return word;
            }
            threshold -= weight;
        }

        unreachable!()
    }

    pub fn generate_text(&self, length: usize, temperature: f64) -> String {
        let mut rng = rng();

        let keys: Vec<&Vec<String>> = self.chain.keys().collect();
        let mut current = keys[rng.random_range(0..keys.len())].clone();
        let mut output = current.clone();

        for _ in 0..length {
            if let Some(next_map) = self.chain.get(&current) {
                let next_token = Self::weighted_choice_with_temperature(next_map, temperature, &mut rng);
                output.push(next_token.clone());
                current = output[output.len() - self.order..].to_vec(); // slide window
            } else {
                break; // dead end
            }
        }

        output.join(if self.use_words { " " } else { "" })
    }
}
