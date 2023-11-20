use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    collections::{hash_map::Entry, HashMap},
    io,
};

pub fn add_empoyee(
    map: &mut HashMap<String, Vec<String>>,
    on_end: fn(map: &mut HashMap<String, Vec<String>>) -> (),
) {
    let mut command = String::new();

    println!("Insert the name of the employee and the department in the following format:");
    println!("Add [name] to [department]:");

    match io::stdin().read_line(&mut command) {
        Ok(_) => (),
        Err(_) => add_empoyee(map, on_end),
    }

    let words: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    let name = words[1].clone();

    let department = words[3].clone();

    match map.entry(department.clone()) {
        Entry::Vacant(e) => {
            e.insert(vec![name.clone()]);
        }
        Entry::Occupied(mut e) => {
            e.get_mut().push(name.clone());
        }
    }

    println!("Employee {} added to {} with success!", name, department);
    on_end(map);
}

pub fn exit() {
    std::process::exit(exitcode::OK);
}

pub fn list_employees(
    map: &mut HashMap<String, Vec<String>>,
    on_end: fn(map: &mut HashMap<String, Vec<String>>) -> (),
) {
    let departments: Vec<String> = map.keys().map(|s| s.to_string()).collect();

    if departments.len() < 1 {
        println!("There are no employees at your company. Register someone first!");
        return main_loop(map);
    }

    let department = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the department")
        .default(0)
        .items(&departments[..])
        .interact()
        .unwrap();

    let department = departments[department].clone();

    match map.get(&department) {
        Some(val) => println!("Empolyees at {} are: {:?}", department, val.join(", ")),
        None => exit(),
    }

    on_end(map)
}

pub fn remove_employee() {
    println!("Removing empolyee")
}

pub fn main_loop(map: &mut HashMap<String, Vec<String>>) {
    let actions = [
        "Insert new employee",
        "Remove employee",
        "List employees",
        "Exit",
    ];
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&actions[..])
        .interact()
        .unwrap();

    match actions[action] {
        "Insert new employee" => add_empoyee(map, main_loop),
        "Remove employee" => remove_employee(),
        "List employees" => list_employees(map, main_loop),
        "Exit" => exit(),
        _ => unreachable!(),
    }
}
