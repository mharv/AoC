use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Cave {
    name: String,
    is_big: bool,
    connections: Vec<String>,
}
fn main() {

    let path = String::from("./test_input.txt");
    // let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let caves = construct_cave_network(content_split);

    // display cave
    if true {
        display_cave_network(&caves);
    }

    // try adjacency matrix
}

fn display_cave_network(cave_network: &HashMap<String, Cave>) {
    for (_k, v) in cave_network.iter() {
        print!("Cave name: {}\t", v.name);
        println!("Cave is big: {}", v.is_big);
        println!("Cave connections:");
        for connection in v.connections.iter() {
            println!("\t{}", connection);
        }
        print!("\n");
    }
}


fn construct_cave_network(content_split: Vec<&str> ) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();

    for row in content_split.iter() {
        let temp_caves: Vec<&str> = row.split("-").collect();

        if caves.contains_key(temp_caves[0]) {
            if !caves.get(temp_caves[0]).unwrap().connections.contains(&temp_caves[1].to_string()) {
                caves.get_mut(temp_caves[0]).unwrap().connections.push(temp_caves[1].to_string());
            }
        } else {
            let mut is_big = false;

            if temp_caves[0].to_uppercase() == temp_caves[0] { is_big = true; }
            let mut connections: Vec<String> = Vec::new();
            connections.push(temp_caves[1].to_string());

            caves.insert(temp_caves[0].to_string(), Cave { name: temp_caves[0].to_string(), is_big, connections });
        }

        if caves.contains_key(temp_caves[1]) {
            if !caves.get(temp_caves[1]).unwrap().connections.contains(&temp_caves[0].to_string()) {
                caves.get_mut(temp_caves[1]).unwrap().connections.push(temp_caves[0].to_string());
            }
        } else {
            let mut is_big = false;

            if temp_caves[1].to_uppercase() == temp_caves[1] { is_big = true; }
            let mut connections: Vec<String> = Vec::new();
            connections.push(temp_caves[0].to_string());

            caves.insert(temp_caves[1].to_string(), Cave { name: temp_caves[1].to_string(), is_big, connections });
        }
    }
    caves
}
