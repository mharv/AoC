use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Cave {
    name: String,
    is_big: bool,
    connections: Vec<String>,
}

struct Agent {
    current_location: String,
    path_history: Vec<Vec<String>>,
}

impl Agent {
    fn new() -> Self {
        Agent { current_location: "start".to_string(), path_history: Vec::new() }
    }

    fn walk_cave(&mut self, cave_network: &HashMap<String, Cave>) {
        let mut new_trip: Vec<String> = Vec::new();
        new_trip.push(cave_network.get(&self.current_location).unwrap().name.to_string());

        while self.current_location != "end" {
            // get current_location options
            // choose an option to navigate to
            // check if option isnt big and hasnt been visited before
            for option in cave_network.get(&self.current_location).unwrap().connections.iter() {
                println!("possible options for {}: {:?}", self.current_location, cave_network.get(&self.current_location).unwrap().connections);
                //println!("option for {}: {}", cave_network.get(&self.current_location).unwrap().name, option);
                if !cave_network.get(option).unwrap().is_big && !new_trip.contains(&option) {
                    self.current_location = cave_network.get(option).unwrap().name.to_string();
                    println!("pushing option {} to trip", cave_network.get(option).unwrap().name.to_string());
                    new_trip.push(cave_network.get(option).unwrap().name.to_string());
                }
            }

            // update new position and push position to new_trip
        }
        self.path_history.push(new_trip);
        println!("{:?}", self.path_history);
    }
}

fn main() {

    let path = String::from("./test_input.txt");
    // let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let caves = construct_cave_network(content_split);

    let mut agent = Agent::new();
    agent.walk_cave(&caves);

    // display cave
    if true {
        for (k, v) in caves.iter() {
            if k == "d" {
                print!("Cave name: {}\t", v.name);
                println!("Cave is big: {}", v.is_big);
                println!("Cave connections:");
                for connection in v.connections.iter() {
                    println!("\t{}", connection);
                }
                print!("\n");
            }
        }
    }

    // try adjacency matrix



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
