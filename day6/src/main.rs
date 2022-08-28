use std::fs;

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut fish_list: Vec<u8> = Vec::new();
    for item in content_split[0].split(",").collect::<Vec<&str>>().iter() {
        fish_list.push(item.parse().expect("not a number"))
    }

    println!("there are now {} lanternfish", fish_list.len());
    fish_list = simulate_n_days(fish_list, 80);
    println!("there are now {} lanternfish", fish_list.len());

}

fn simulate_n_days(mut fish_list: Vec<u8>, n: i32) -> Vec<u8> {
    for _ in 0..n {
        let mut new_fish: Vec<u8> = Vec::new();
        for fish in fish_list.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        fish_list.append(&mut new_fish);
    }
    fish_list
}
