use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

const WORD_LIST_PATH: &'static str = "res/words_alpha.txt";
const WORD_LENGTH: usize = 5;

fn has_repeat(a: u32, b: u32) -> bool {
    a & b != 0
}

fn char_to_bit_mask(c: char) -> u32 {
    let mut b = [0];
    c.encode_utf8(&mut b);

    1 << (b[0] - 0x41)
}

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let word_file = File::open(WORD_LIST_PATH)?;
    let word_file = BufReader::new(word_file);

    let word_list = word_file
        .lines()
        .map(|l| l.unwrap())
        .filter(|w| w.len() == WORD_LENGTH)
        .filter_map(|word| {
            let mut bit_rep = 0u32;

            for c in word.chars() {
                let c = c.to_ascii_lowercase();
                let mask = char_to_bit_mask(c);

                if has_repeat(bit_rep, mask) {
                    return None;
                }

                bit_rep ^= mask;
            }

            Some((bit_rep, word))
        });

    log::info!("Processing word list");

    let word_map: HashMap<u32, Vec<String>> = {
        let mut map: HashMap<u32, Vec<String>> = HashMap::new();

        for (bit_rep, word) in word_list {
            if let Some(words) = map.get_mut(&bit_rep) {
                words.push(word);
            } else {
                map.insert(bit_rep, vec![word]);
            }
        }

        map
    };

    let word_map_file = File::create("out/word_map.json")?;
    serde_json::to_writer(&word_map_file, &word_map)?;

    log::info!("Wrote word map");

    log::info!("Finding results");

    let num_unique_bit_rep = word_map.len();

    log::info!("Total Number of Unique Letter Sets: {}", num_unique_bit_rep);

    let mut processed: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut results: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut num_results = 0;

    for ((&bit_rep, word), i) in word_map.iter().zip(1..=num_unique_bit_rep) {
        log::debug!(
            "({} / {}) | {} | {:#028b}: {:?}",
            i,
            num_unique_bit_rep,
            processed.len(),
            bit_rep,
            word
        );

        let prev_keys = processed.keys().map(|&k| k).collect::<Vec<_>>();
        for other_bit_rep in prev_keys {
            if !has_repeat(bit_rep, other_bit_rep) {
                let new_bit_rep = bit_rep | other_bit_rep;
                let mut new_sets = processed[&other_bit_rep].clone();

                for set in new_sets.iter_mut() {
                    set.push(bit_rep);
                }

                if new_sets.first().unwrap().len() == 5 {
                    log::info!("Found {} more!", new_sets.len());
                    num_results += new_sets.len();

                    if let Some(sets) = results.get_mut(&new_bit_rep) {
                        sets.append(&mut new_sets);
                    } else {
                        results.insert(new_bit_rep, new_sets);
                    }
                } else {
                    if let Some(sets) = processed.get_mut(&new_bit_rep) {
                        sets.append(&mut new_sets);
                    } else {
                        processed.insert(new_bit_rep, new_sets);
                    }
                }
            }
        }

        processed.insert(bit_rep, vec![vec![bit_rep]]);
    }

    log::info!("DONE COMPUTING!");
    log::info!("Number of results: {}", num_results);

    let results_file = File::create("out/results.json")?;
    serde_json::to_writer(&results_file, &results)?;

    log::info!("Wrote result to file");

    Ok(())
}
