/*
 Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
 SPDX-License-Identifier: BSD-2-Clause
 */

extern crate rand;

use rand::Rng;

struct GeneratorOptions {
    use_upper_case_letters: bool,
    use_lower_case_letters: bool,
    use_digits: bool,
    use_symbols: bool,
    length: i8,
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


fn generate(generator_options: GeneratorOptions) -> String {
    let mut rng = rand::thread_rng();

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

        return password;
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
    let options = GeneratorOptions {
        use_upper_case_letters: true,
        use_lower_case_letters: true,
        use_symbols: true,
        use_digits: true,
        length: 4,
    };

    let password = generate(options);
    println!("password = {password}");
}
