use std::fs;

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not &read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut fish_list: Vec<u8> = Vec::new();
    for item in content_split[0].split(",").collect::<Vec<&str>>().iter() {
        fish_list.push(item.parse().expect("not a number"))
    }

    println!("there are now {} lanternfish", fish_list.len());
    let number_of_fish = simulate_n_days(&mut fish_list, 256);
    println!("there are now {} lanternfish", number_of_fish );

}

fn simulate_n_days(fish_list: &mut Vec<u8>, n: i32) -> i64 {
    let mut lifetime_tracker: Vec<i64> = vec![0; 9];

    for fish in fish_list.iter() {
        match fish {
            0 => lifetime_tracker[0] += 1,
            1 => lifetime_tracker[1] += 1,
            2 => lifetime_tracker[2] += 1,
            3 => lifetime_tracker[3] += 1,
            4 => lifetime_tracker[4] += 1,
            5 => lifetime_tracker[5] += 1,
            6 => lifetime_tracker[6] += 1,
            7 => lifetime_tracker[7] += 1,
            8 => lifetime_tracker[8] += 1,
            _ => panic!("derp number in input found!"),
        }
    }

    let mut last_lifetime_tracker = lifetime_tracker.clone();
    for _ in 0..n {
        for i in (1..last_lifetime_tracker.len()).rev() {
            lifetime_tracker[i-1] = last_lifetime_tracker[i];
        }
        lifetime_tracker[6] += last_lifetime_tracker[0];
        lifetime_tracker[8] = last_lifetime_tracker[0];
        last_lifetime_tracker = lifetime_tracker.clone();
    }
    last_lifetime_tracker.iter().sum()
}
