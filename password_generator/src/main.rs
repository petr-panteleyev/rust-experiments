/*
Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
SPDX-License-Identifier: BSD-2-Clause
*/
use std::env::Args;
use std::iter::Skip;
use std::process::exit;
use gtk4::prelude::*;
use gtk4::Application;

pub mod controller;
pub mod generator;

use crate::controller::*;
use crate::generator::{GeneratorOptions, DEFAULT_LENGTH};

const APP_ID: &str = "org.panteleyev.PasswordGenerator";

const USAGE_MESSAGE: &str = r#"
Password Generator
(c) 2024, Petr Panteleyev
Usage:
  -u        - use upper case letters
  -l        - use lower case letters
  -d        - use digits
  -s        - use symbols
  -n <n>    - password length, default: 16, allowed range: [4..32]
  --pin     - PIN: -d -n 4
  --unix    - Unix password: -ulds -n 8
  --help    - print this help message and exit
"#;

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() == 0 {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(init_ui);
        app.run();
    } else {
        let options = build_options(&mut args);
        let password = generator::generate(&options).unwrap_or(String::from("ERROR!"));
        println!("{password}");
    }
}

fn build_options(args: &mut Skip<Args>) -> GeneratorOptions {
    let mut use_upper_case = false;
    let mut use_lower_case = false;
    let mut use_digits = false;
    let mut use_symbols = false;
    let mut length: u8 = DEFAULT_LENGTH;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => {
                println!("{}", USAGE_MESSAGE);
                exit(0);
            }
            "--unix" => {
                use_upper_case = true;
                use_lower_case = true;
                use_digits = true;
                use_symbols = true;
                length = 8;
                break;
            }
            "--pin" => {
                use_upper_case = false;
                use_lower_case = false;
                use_digits = true;
                use_symbols = false;
                length = 4;
                break;
            }
            "-n" => {
                if let Some(arg_length) = args.next() {
                    length = arg_length
                        .parse::<u8>()
                        .unwrap_or(DEFAULT_LENGTH);
                } else {
                    panic!("Length argument expects an u8 value.");
                }
            }
            _ => {
                if arg.starts_with('-') {
                    for ch in arg.chars().skip(1) {
                        match ch {
                            'u' => { use_upper_case = true; }
                            'l' => { use_lower_case = true; }
                            'd' => { use_digits = true; }
                            's' => { use_symbols = true; }
                            _ => {
                                panic!("Invalid option -{ch}")
                            }
                        }
                    }
                } else {
                    panic!("Unrecognized argument {}", arg);
                }
            }
        }
    }

    GeneratorOptions {
        upper_case: use_upper_case,
        lower_case: use_lower_case,
        digits: use_digits,
        symbols: use_symbols,
        length,
    }
}
