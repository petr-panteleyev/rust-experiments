/*
Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
SPDX-License-Identifier: BSD-2-Clause
*/

extern crate rand;
use rand::Rng;

pub struct GeneratorOptions {
    pub upper_case: bool,
    pub lower_case: bool,
    pub digits: bool,
    pub symbols: bool,
    pub length: u8,
}

impl GeneratorOptions {
    fn get_character_sets(&self) -> Vec<&[char]> {
        let mut character_sets: Vec<&[char]> = Vec::new();
        if self.upper_case {
            character_sets.push(&UPPER_CASE_CHARS)
        }
        if self.lower_case {
            character_sets.push(&LOWER_CASE_CHARS)
        }
        if self.digits {
            character_sets.push(&DIGITS)
        }
        if self.symbols {
            character_sets.push(&SYMBOLS)
        }
        character_sets
    }
}

pub const DEFAULT_LENGTH: u8 = 16;

static UPPER_CASE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

static LOWER_CASE_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

static SYMBOLS: [char; 14] = [
    '@', '#', '$', '%', '&', '*', '(', ')', '-', '+', '=', '^', '.', ',',
];

static BAD_LETTERS: [char; 3] = ['I', 'l', 'O'];

const MIN_LENGTH: u8 = 4;

pub fn generate(generator_options: &GeneratorOptions) -> Result<String, String> {
    if generator_options.length < MIN_LENGTH {
        return Err("Password length must be >= 4".to_string());
    }

    let character_sets = generator_options.get_character_sets();
    if character_sets.is_empty() {
        return Err("At least one character set must be selected".to_string());
    }

    let mut rng = rand::thread_rng();

    let mut password = String::new();

    'generation: loop {
        for _ in 0..generator_options.length {
            let character_set = character_sets
                .get(rng.gen_range(0..character_sets.len()))
                .unwrap();

            let character = loop {
                let character = character_set[rng.gen_range(0..character_set.len())];
                if check_character(character) {
                    break character;
                }
            };

            password.push(character);
        }

        for character_set in &character_sets {
            if !check_password(character_set, &password) {
                password.clear();
                continue 'generation;
            }
        }

        return Ok(password);
    }
}

fn check_character(ch: char) -> bool {
    !BAD_LETTERS.contains(&ch)
}

fn check_password(charset: &[char], password: &String) -> bool {
    for ch in password.chars() {
        if charset.contains(&ch) {
            return true;
        }
    }
    false
}
