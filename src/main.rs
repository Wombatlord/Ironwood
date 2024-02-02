mod cursor_consts;
mod door;
mod input;
mod player;
mod probability;
use cursor_consts::{CLEAR, COLOUR_RESET, DOORMOJI, GREEN, HOME, RED, SKULL};

use crate::input::game_loop;
use door::Doorgeon;
use player::Player;

fn box_str(str_to_box: &str) {
    let mut built: String = String::from(format!("{CLEAR}{HOME}{GREEN}╭"));
    let str_size: usize = str_to_box.len();
    for _ in 0..str_size {
        built.push('─')
    }
    built.push_str("╮\n");
    let mut mid_str: String = format!("│{:>width$}{str_to_box}", "", width = 9);
    mid_str.push_str(&String::from(format!("{:>w2$}{GREEN}│", "", w2 = 8)) as &str);
    built.push_str(&mid_str);
    built.push_str("\n╰");
    for _ in 0..str_size {
        built.push('─')
    }
    built.push('╯');

    println!("{built}{COLOUR_RESET}");
}

fn main() {
    let banner: &str = &String::from(format!(
        "{GREEN}{SKULL} GAME OF DOORS: IRONWOOD EDITION {SKULL}{COLOUR_RESET}"
    ));
    box_str(banner);
    println!("\nWelcome back to {DOORMOJI}{RED}The Doorgeon{COLOUR_RESET}{DOORMOJI}.\n");

    let mut doorgeon: Doorgeon = Doorgeon::new();
    let mut player: Player = Player::new();

    doorgeon.populate();
    game_loop(doorgeon, &mut player);

    println!(
        "\n{GREEN}DOORSCORE: {0}, CLEARED: {1}{SKULL}{COLOUR_RESET}\n",
        player.get_score(),
        player.cleared_as_doormoji()
    );
}
