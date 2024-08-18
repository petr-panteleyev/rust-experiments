/*
Copyright © 2024 Petr Panteleyev <petr@panteleyev.org>
SPDX-License-Identifier: BSD-2-Clause
*/
use crate::generator::*;
use gtk4::gio::{ActionEntry, Menu, MenuItem, MenuModel, SimpleActionGroup};
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::Orientation::{Horizontal, Vertical};
use gtk4::{
    Application, ApplicationWindow, Box, Button, CheckButton, DropDown, Entry, Label,
    PopoverMenuBar,
};

const BIG_SPACING: i32 = 10;
const SMALL_SPACING: i32 = 5;

static LENGTH_STRINGS: [&str; 6] = ["4", "8", "16", "24", "32", "48"];
static LENGTHS: [u8; 6] = [4, 8, 16, 24, 32, 48];
const DEFAULT_LENGTH: u8 = 16;
const DEFAULT_LENGTH_INDEX: u32 = 2;

static MEDIUM: GeneratorOptions = GeneratorOptions {
    upper_case: true,
    lower_case: true,
    digits: true,
    symbols: true,
    length: 16,
};

static LONG: GeneratorOptions = GeneratorOptions {
    upper_case: true,
    lower_case: true,
    digits: true,
    symbols: true,
    length: 32,
};

static UNIX: GeneratorOptions = GeneratorOptions {
    upper_case: true,
    lower_case: true,
    digits: true,
    symbols: true,
    length: 8,
};

static PIN: GeneratorOptions = GeneratorOptions {
    upper_case: false,
    lower_case: false,
    digits: true,
    symbols: false,
    length: 4,
};

static DEFAULT_OPTIONS: GeneratorOptions = GeneratorOptions {
    upper_case: true,
    lower_case: true,
    digits: true,
    symbols: false,
    length: DEFAULT_LENGTH,
};

pub fn init_ui(app: &Application) {
    let password_text = Entry::builder().editable(false).build();

    let upper_case_button = charset_button("Upper Case", DEFAULT_OPTIONS.upper_case);
    let lower_case_button = charset_button("Lower Case", DEFAULT_OPTIONS.lower_case);
    let digits_button = charset_button("Digits", DEFAULT_OPTIONS.digits);
    let symbols_button = charset_button("Symbols", DEFAULT_OPTIONS.symbols);

    let generate_button = Button::with_label("Generate");

    let vertical_box = Box::new(Vertical, BIG_SPACING);

    let char_sets_box = Box::new(Horizontal, BIG_SPACING);
    char_sets_box.append(&upper_case_button.clone());
    char_sets_box.append(&lower_case_button.clone());
    char_sets_box.append(&digits_button);
    char_sets_box.append(&symbols_button);

    // Length box
    let length_label = Label::new(Some("Length:"));
    let length_drop_down = DropDown::from_strings(&LENGTH_STRINGS);
    length_drop_down.set_selected(DEFAULT_LENGTH_INDEX);

    let length_box = Box::new(Horizontal, SMALL_SPACING);
    length_box.append(&length_label);
    length_box.append(&length_drop_down);

    let menu_bar = Menu::new();
    menu_bar.insert_submenu(0, Some("File"), &create_file_menu());
    menu_bar.insert_submenu(1, Some("Variants"), &create_variants_menu());

    let variants_actions = SimpleActionGroup::new();
    variants_actions.add_action_entries([
        build_predefined_action(
            "medium",
            &MEDIUM,
            &password_text,
            &upper_case_button,
            &lower_case_button,
            &digits_button,
            &symbols_button,
            &length_drop_down,
        ),
        build_predefined_action(
            "long",
            &LONG,
            &password_text,
            &upper_case_button,
            &lower_case_button,
            &digits_button,
            &symbols_button,
            &length_drop_down,
        ),
        build_predefined_action(
            "unix",
            &UNIX,
            &password_text,
            &upper_case_button,
            &lower_case_button,
            &digits_button,
            &symbols_button,
            &length_drop_down,
        ),
        build_predefined_action(
            "pin",
            &PIN,
            &password_text,
            &upper_case_button,
            &lower_case_button,
            &digits_button,
            &symbols_button,
            &length_drop_down,
        ),
    ]);

    let menu_model: MenuModel = menu_bar.into();
    let menubar = PopoverMenuBar::from_model(Some(&menu_model));

    vertical_box.append(&menubar);
    vertical_box.append(&password_text);
    vertical_box.append(&char_sets_box);
    vertical_box.append(&length_box);
    vertical_box.append(&generate_button);

    generate_button.connect_clicked(clone!(
        #[strong]
        password_text,
        #[strong]
        upper_case_button,
        #[strong]
        lower_case_button,
        #[strong]
        digits_button,
        #[strong]
        symbols_button,
        #[strong]
        length_drop_down,
        move |_| {
            let password = on_generate(&GeneratorOptions {
                upper_case: upper_case_button.is_active(),
                lower_case: lower_case_button.is_active(),
                digits: digits_button.is_active(),
                symbols: symbols_button.is_active(),
                length: LENGTHS[length_drop_down.selected() as usize],
            });
            password_text.set_text(&password);
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Password Generator")
        .child(&vertical_box)
        .build();

    window.insert_action_group("file", Some(&create_file_actions(app)));
    window.insert_action_group("variants", Some(&variants_actions));

    app.set_accels_for_action("variants.medium", &["<Ctrl>M"]);
    app.set_accels_for_action("variants.long", &["<Ctrl>L"]);
    app.set_accels_for_action("variants.unix", &["<Ctrl>U"]);
    app.set_accels_for_action("variants.pin", &["<Ctrl>P"]);

    window.present();
}

fn charset_button(label: &str, active: bool) -> CheckButton {
    CheckButton::builder().label(label).active(active).build()
}

fn on_generate(options: &GeneratorOptions) -> String {
    generate(&options).unwrap_or(String::from(""))
}

fn create_file_menu() -> Menu {
    let file_menu = Menu::new();
    file_menu.insert_item(0, &MenuItem::new(Some("Exit"), Some("file.exit")));
    file_menu
}

fn create_file_actions(app: &Application) -> SimpleActionGroup {
    let file_actions = SimpleActionGroup::new();
    let action_file_exit = ActionEntry::builder("exit")
        .activate(clone!(
            #[strong]
            app,
            move |_group: &SimpleActionGroup, _, _| app.quit()
        ))
        .build();
    file_actions.add_action_entries([action_file_exit]);
    file_actions
}

fn create_variants_menu() -> Menu {
    let var_menu = Menu::new();

    let section_1 = Menu::new();
    section_1.insert_item(
        0,
        &MenuItem::new(Some("Medium Password"), Some("variants.medium")),
    );
    section_1.insert_item(
        1,
        &MenuItem::new(Some("Long Password"), Some("variants.long")),
    );
    let model_1: MenuModel = section_1.into();

    let section_2 = Menu::new();
    section_2.insert_item(0, &MenuItem::new(Some("Unix"), Some("variants.unix")));
    section_2.insert_item(1, &MenuItem::new(Some("PIN"), Some("variants.pin")));
    let model_2: MenuModel = section_2.into();

    var_menu.insert_item(0, &MenuItem::new_section(None, &model_1));
    var_menu.insert_item(1, &MenuItem::new_section(None, &model_2));

    var_menu
}

fn setup_controls(
    ctx: &GeneratorOptions,
    upper_case_button: &CheckButton,
    lower_case_button: &CheckButton,
    digits_button: &CheckButton,
    symbols_button: &CheckButton,
    length_drop_down: &DropDown,
) {
    upper_case_button.set_active(ctx.upper_case);
    lower_case_button.set_active(ctx.lower_case);
    digits_button.set_active(ctx.digits);
    symbols_button.set_active(ctx.symbols);

    match LENGTHS.binary_search(&ctx.length) {
        Ok(index) => length_drop_down.set_selected(index as u32),
        Err(_) => {}
    }
}

fn build_predefined_action(
    action_name: &str,
    ctx: &'static GeneratorOptions,
    password_text: &Entry,
    upper_case_button: &CheckButton,
    lower_case_button: &CheckButton,
    digits_button: &CheckButton,
    symbols_button: &CheckButton,
    length_drop_down: &DropDown,
) -> ActionEntry<SimpleActionGroup> {
    let action = ActionEntry::builder(action_name)
        .activate(clone!(
            #[strong]
            ctx,
            #[strong]
            password_text,
            #[strong]
            upper_case_button,
            #[strong]
            lower_case_button,
            #[strong]
            digits_button,
            #[strong]
            symbols_button,
            #[strong]
            length_drop_down,
            move |_group: &SimpleActionGroup, _, _| {
                setup_controls(
                    &ctx,
                    &upper_case_button,
                    &lower_case_button,
                    &digits_button,
                    &symbols_button,
                    &length_drop_down,
                );

                let password = on_generate(ctx);
                password_text.set_text(&password);
            }
        ))
        .build();

    action
}
