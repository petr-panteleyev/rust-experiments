/*
Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
SPDX-License-Identifier: BSD-2-Clause
*/
use gtk4::prelude::*;
use gtk4::Application;

pub mod controller;
pub mod generator;

use crate::controller::*;

const APP_ID: &str = "org.panteleyev.PasswordGenerator";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(init_ui);
    app.run();
}
