use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

const WORD_LIST_PATH: &'static str = "res/words_alpha.txt";
const WORD_LENGTH: usize = 5;

const NUM_POSSIBLE_CHARACTER_SETS: usize = 67_108_864;

fn has_repeat(a: u32, b: u32) -> bool {
    a & b != 0
}

fn char_to_bit_mask(c: char) -> u32 {
    let mut b = [0];
    c.encode_utf8(&mut b);

    1 << (b[0] - 0x41)
}

fn main() -> io::Result<()> {
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

    let mut processed: Vec<Vec<String>> = vec![vec![]; NUM_POSSIBLE_CHARACTER_SETS];
    let mut populated: HashSet<usize> = HashSet::with_capacity(NUM_POSSIBLE_CHARACTER_SETS);

    for (bit_rep, word) in word_list {
        println!("{:#028b}: {}", bit_rep, word);

        let mut added = HashSet::new();
        for &target in populated
            .iter()
            .filter(|&&t| !has_repeat(bit_rep, t as u32))
        {
            let new_bit_rep = (bit_rep ^ target as u32) as usize;
            let mut new_words = processed[target]
                .iter()
                .map(|words| format!("{} {}", words, word))
                .collect::<Vec<_>>();

            processed[new_bit_rep].append(&mut new_words);
            added.insert(new_bit_rep);
        }

        processed[bit_rep as usize].push(word);
        populated.insert(bit_rep as usize);
        for &n in &added {
            populated.insert(n);
        }
    }

    let out_file = File::create("out/output.json")?;
    serde_json::to_writer(&out_file, &processed)?;

    Ok(())
}
