use rand::Rng;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

const SAVEDATA_FILE_NAME: &str = "savedata.save";

fn main() {
    println!("=== Ghost Door Game ===\n\n");
    println!("There are three doors. Pick a door to go throug. If you take the door where the ghost is behind, you are dead.\n");

    let mut is_alive: bool = true;
    let mut score: u32 = 0;

    while is_alive {
        println!("Pick a door from 1 to 3.");

        let ghost_door_number = get_random_ghost_door();
        let player_door_number = ask_player_for_door_number();

        // check if player picked a valid number
        if player_door_number == u8::from(0) {
            continue;
        } else if player_door_number < 1 || player_door_number > 3 {
            println!(
                "You only got three choices! You cant pick {}.\n",
                player_door_number
            );
            continue;
        }

        // check for reward or death
        if player_door_number != ghost_door_number {
            score += 1;

            println!("Phew! You didnt pick the ghost door. You advance one room further.\n")
        } else {
            is_alive = false;
            let high_score = load_saved_score();

            println!("Boo! The ghost scared you! You got cancer from it. You are dead now.");
            if score > high_score {
                println!("\nYOUR NEW HIGH SCORE IS {}!", score);
                save_high_score(score);
            }
            println!("\nScore: {}\nHigh Score: {}", score, high_score);

            sleep(Duration::from_secs(3));
        }
    }
}

fn load_saved_score() -> u32 {
    let savedata_file_path = get_savedata_file_path();

    let mut file;

    if !Path::is_file(&savedata_file_path) {
        file = File::create(savedata_file_path).expect("Could not create save file!");
        file.write_all(b"0").expect("Could not write to save file!");
        return 0;
    } else {
        file = File::open(savedata_file_path).expect("Could not open save data file!");
        let mut saved_score = String::new();

        file.read_to_string(&mut saved_score)
            .expect("Could not read save file!");

        let saved_score = saved_score
            .trim()
            .parse::<u32>()
            .expect("Could not convert score!");

        return saved_score;
    }
}

fn save_high_score(high_score: u32) {
    let savedata_file_path = get_savedata_file_path();
    let high_score = high_score.to_string();

    let mut file;

    if !Path::is_file(&savedata_file_path) {
        file = File::create(savedata_file_path).expect("Could not create save file!");
        file.write_all(high_score.as_bytes())
            .expect("Could not write to save file!");
    } else {
        file = File::create(savedata_file_path).expect("Could not open save data file!");

        file.write_all(high_score.as_bytes())
            .expect("Could not save high score!")
    }
}

fn get_savedata_file_path() -> PathBuf {
    let savedata_file_path = Path::new(".");
    let savedata_file_path = savedata_file_path.join(SAVEDATA_FILE_NAME);
    return savedata_file_path;
}

fn get_random_ghost_door() -> u8 {
    return rand::thread_rng().gen_range(1..=3);
}

fn ask_player_for_door_number() -> u8 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Something went wrong by reading input!");

    let door_number = user_input.trim().parse::<u8>();

    let door_number = match door_number {
        Ok(door_number) => door_number,
        Err(_) => {
            println!(
                "There is no \"{}\" door! Pick a valid one.\n",
                user_input.trim()
            );
            return 0;
        }
    };

    return door_number;
}
