use rand::{rng, seq::IndexedRandom};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Markov {
    n_gram: usize,
    chain: HashMap<Vec<char>, Vec<char>>,
    // chain: HashMap<Vec<String>, Vec<String>>,
}

impl Markov {
    pub fn new(files: Vec<String>, n_gram: usize) -> Self {
        let mut markov = Self {
            n_gram,
            chain: HashMap::new(),
        };

        for file in files {
            // Open a file to use as the data
            let f = File::open(file.as_str()).unwrap();
            let reader = BufReader::new(f);

            // let words_iter = reader.lines().map_while(Result::ok).flat_map(|line| {
            //     line.split_whitespace()
            //         .map(String::from)
            //         .collect::<Vec<_>>()
            // });

            let char_iter = reader
                .lines()
                .map_while(Result::ok)
                .flat_map(|line| line.chars().collect::<Vec<_>>());

            // Split the file contents into words
            let mut prev_n = VecDeque::new();
            for current in char_iter {
                // for current in words_iter {
                // Add to the chain or insert a new entry if the key has no associated chain yet
                if !prev_n.is_empty() {
                    markov
                        .chain
                        .entry(prev_n.clone().into())
                        .or_insert_with(|| vec![current.clone()])
                        .push(current.clone());
                }

                // Rotate the previous entries so that there is only ever N_gram amount of entries in the previous N
                if prev_n.len() < markov.n_gram {
                    prev_n.push_back(current.clone());
                } else {
                    prev_n.pop_front();
                    prev_n.push_back(current.clone());
                }
            }
        }

        markov
    }

    pub fn generate_text(&self, length: usize) -> String {
        // Generate words from the chain
        let mut rng = rng();
        let keys = self.chain.keys().cloned().collect::<Vec<_>>();
        let mut key: VecDeque<_> = keys.choose(&mut rng).unwrap().clone().into();

        let mut output: Vec<_> = key.clone().into();

        for _ in 0..(length - self.n_gram) {
            let Some(next_words) = self.chain.get(&Into::<Vec<_>>::into(key.clone())) else {
                break;
            };

            let Some(next) = next_words.choose(&mut rng) else {
                break;
            };

            output.push(next.clone());

            key.pop_front();
            key.push_back(next.clone());
        }

        output.into_iter().collect()
        // output.join(" ")
    }
}
