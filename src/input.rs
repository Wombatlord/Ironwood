use crate::cursor_consts::{COLOUR_RESET, CYAN, RED};
use crate::door::Doorgeon;
use crate::player::Player;
use crate::probability::{barz, game_length_chance_percent, map_range};
use std::io;

pub fn game_loop(mut doorgeon: Doorgeon, player: &mut Player) -> () {
    // The Game Loop
    loop {
        let mut input: String = String::new();
        let available_doors: usize = doorgeon.available_doors();
        match available_doors {
            0 => println!("There is a single Door before you, marked with 0.\nWhich Door will you choose?"),
            _ => println!("There are new Doors before you, marked between 0 and {0}.\nWhich Door will you choose?", available_doors)
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => panic!("Failed to read input: {e}"),
        };

        if input.trim() == "exit" {
            break;
        }
        let chance = input.split(" ").collect::<Vec<&str>>();

        let safe_doors = chance_args_as_usize_tuple(&chance);
        
        if chance[0].trim() == "chance" {
            let bars: Vec<String> = (safe_doors.0..=safe_doors.1 + safe_doors.0)
                .map(|i| game_length_chance_percent(i, safe_doors.0))
                .map(|pct| map_range(100.0, (0.0, 50.0), pct, Some(1.0)))
                .map(|(range, pct)| barz(range, pct))
                .collect();
            print!("\x1b[2A");
            print!("\x1b[0K");
            print!("\x1b[1A");
            print!("\x1b[0K");
            for bar in bars {
                println!("{bar}")
            }
            println!();
        } else {
            println!(
                "{CYAN}You attempt to open Door {0}.{COLOUR_RESET}",
                input.trim()
            );
            let choice: Result<usize, _> = input.trim().parse();

            let Ok(input) = choice else { continue };

            if input > available_doors {
                println!("{RED}There aren't that many Doors yet.{COLOUR_RESET}");
                continue;
            }

            let (result, score) = doorgeon.select_door(input);

            player.increase_score(score as i64);
            match result {
                true => doorgeon.populate(),
                false => break,
            }
            player.incr_cleared_tally()
        }
    }
}

fn chance_args_as_usize_tuple(chance: &Vec<&str>) -> (usize, usize) {
    return match chance.len() {
        3 => doors_and_rooms(chance),
        2 => doors_only(chance),
        _ => (1, 5)
    }
}

fn doors_only(chance: &Vec<&str>) -> (usize, usize) {
    match chance[1].trim().parse::<usize>() {
        Ok(given_doors) => {
            return (given_doors, 5);
        },
        Err(e) => {
            panic!("IO Error: {e}");
        }
    }
}

fn doors_and_rooms(chance: &Vec<&str>) -> (usize, usize) {
    let given_doors = chance[1].trim().parse::<usize>().unwrap();
    let total_rooms: usize = chance[2].trim().parse().unwrap();
    return (given_doors, total_rooms);
}
