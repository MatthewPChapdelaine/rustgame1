mod game_state;
mod input_handler;
mod modules;
mod save_load;

use game_state::GameState;
use input_handler::get_input;
use save_load::{save_game, load_game};
use modules::module1;
use modules::module2;
use modules::module3;
use modules::module4;
use modules::module5;

fn main() {
    println!("Welcome to Automated University Instructor: Interconnected Systems!");
    let mut game_state: Option<GameState> = None;

    loop {
        println!("\nMain Menu:");
        println!("1. Start New Game");
        println!("2. Load Game");
        println!("3. Save Game");
        println!("4. Exit");
        let choice = get_input("Choose an option:");

        match choice.as_str() {
            "1" => {
                let name = get_input("Enter your name:");
                game_state = Some(GameState::new(name));
                run_game(game_state.as_mut().unwrap());
            }
            "2" => {
                if let Some(loaded_state) = load_game() {
                    game_state = Some(loaded_state);
                    println!("Welcome back, {}!", game_state.as_ref().unwrap().player_name);
                    run_game(game_state.as_mut().unwrap());
                }
            }
            "3" => {
                if let Some(ref mut state) = game_state {
                    save_game(state);
                } else {
                    println!("No game to save. Start a new game first.");
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn run_game(game_state: &mut GameState) {
    loop {
        println!("\nAvailable Modules (Knowledge Level: {})", game_state.knowledge_level);
        println!("1. Property and Labor Structure");
        println!("2. University Integration");
        println!("3. Sustainable City Design");
        println!("4. Metaverse: The Nexus");
        println!("5. Interconnected Systems Theory (IST)");
        println!("Type 'menu' to return.");
        let choice = get_input("Choose a module:");

        if choice == "menu" {
            break;
        }

        if game_state.completed_modules.contains(&choice) {
            println!("Module already completed!");
            continue;
        }

        match choice.as_str() {
            "1" => module1::run(game_state),
            "2" => module2::run(game_state),
            "3" => module3::run(game_state),
            "4" => module4::run(game_state),
            "5" => module5::run(game_state),
            _ => println!("Invalid module."),
        }

        if game_state.completed_modules.len() == 5 {
            println!("Congratulations, {}! You've completed all modules!", game_state.player_name);
            break;
        }
    }
}
