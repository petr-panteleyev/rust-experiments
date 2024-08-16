/*
 Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
 SPDX-License-Identifier: BSD-2-Clause
 */

extern crate rand;

use std::env;
use rand::Rng;

struct GeneratorOptions {
    use_upper_case_letters: bool,
    use_lower_case_letters: bool,
    use_digits: bool,
    use_symbols: bool,
    length: u8,
}

static UPPER_CASE_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

static LOWER_CASE_CHARS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

static DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

static SYMBOLS: [char; 14] = [
    '@', '#', '$', '%', '&', '*', '(', ')', '-', '+', '=', '^', '.', ','
];

static BAD_LETTERS: [char; 3] = [
    'I', 'l', 'O'
];

const MIN_LENGTH: u8 = 4;
const DEFAULT_LENGTH: u8 = 16;


fn generate(generator_options: &GeneratorOptions) -> Result<String, String> {
    if generator_options.length < MIN_LENGTH {
        return Err("Password length must be >= 4".to_string());
    }

    let mut character_sets: Vec<&[char]> = Vec::new();
    if generator_options.use_upper_case_letters {
        character_sets.push(&UPPER_CASE_CHARS)
    }
    if generator_options.use_lower_case_letters {
        character_sets.push(&LOWER_CASE_CHARS)
    }
    if generator_options.use_digits {
        character_sets.push(&DIGITS)
    }
    if generator_options.use_symbols {
        character_sets.push(&SYMBOLS)
    }

    if character_sets.is_empty() {
        return Err("At least one character set must be selected".to_string());
    }

    let mut rng = rand::thread_rng();

    let mut password = String::new();

    'generation: loop {
        for _ in 0..generator_options.length {
            let character_set = character_sets.get(rng.gen_range(0..character_sets.len()))
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

fn check_password(bucket: &[char], password: &String) -> bool {
    for ch in password.chars() {
        if bucket.contains(&ch) {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: password_generator <length> <character_sets>");
        println!("character_sets:");
        println!("\tu: upper case\n\tl: lower case\n\td: digits\n\ts: symbols");
        println!("Example: 32 ulds");
        return;
    }

    let password_length = args[1].parse().unwrap_or(DEFAULT_LENGTH);
    let options = &args[2];

    let options = GeneratorOptions {
        use_upper_case_letters: options.contains(&"u".to_string()),
        use_lower_case_letters: options.contains(&"l".to_string()),
        use_symbols: options.contains(&"s".to_string()),
        use_digits: options.contains(&"d".to_string()),
        length: password_length,
    };

    let password = generate(&options).unwrap_or_else(|error| {
        println!("Not generated: \"{error}\"");
        String::from("")
    });

    if !password.is_empty() {
        println!("password = {password}");
    }
}
