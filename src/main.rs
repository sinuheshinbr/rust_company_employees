mod fns;
use crate::fns::main_loop;
use std::collections::HashMap;

fn main() {
    println!("Company HR mgmt program");

    let mut map = HashMap::new();

    main_loop(&mut map);

    println!("Actual map: {:?}", map)
}
