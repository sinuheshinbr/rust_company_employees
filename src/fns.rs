use std::{
    collections::{hash_map::Entry, HashMap},
    io,
};

pub fn add_empoyee(map: &mut HashMap<String, Vec<String>>) {
    let mut command = String::new();

    println!("Insert the name of the employee and the department in the following format:");
    println!("Add [name] to [department]:");

    match io::stdin().read_line(&mut command) {
        Ok(_) => (),
        Err(_) => add_empoyee(map),
    }

    let words: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    let name = words[1].clone();

    let department = words[3].clone();

    match map.entry(department) {
        Entry::Vacant(e) => {
            e.insert(vec![name]);
        }
        Entry::Occupied(mut e) => {
            e.get_mut().push(name);
        }
    }
}

pub fn exit() {
    std::process::exit(exitcode::OK);
}

pub fn remove_employee() {
    println!("Removing empolyee")
}
