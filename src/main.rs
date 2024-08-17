/*
 Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
 SPDX-License-Identifier: BSD-2-Clause
 */

use std::env;

pub mod generator;

use crate::generator::*;

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
