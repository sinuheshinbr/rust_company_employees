mod fns;
use crate::fns::{add_empoyee, exit, remove_employee};
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;

fn main() {
    println!("Company HR mgmt program");

    let actions = ["Insert new employee", "Remove employee", "Exit"];

    let mut map = HashMap::new();

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&actions[..])
        .interact()
        .unwrap();

    match actions[action] {
        "Insert new employee" => add_empoyee(&mut map),
        "Remove employee" => remove_employee(),
        "Exit" => exit(),
        _ => unreachable!(),
    }

    println!("Actual map: {:?}", map)
}
