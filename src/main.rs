use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to BlackJack are you ready to gamble you life savings away? No? Yes? I DONT CARE WE'RE PLAYING BLAKCJACK");
    println!("SO, WHOS PLAYING? TELL ME. GIVE ME YOUR NAMES (WITH A SPACE IN BETWEEN THEM)! NOW, NOW, NOW!");

    let mut player_names_string: String = String::new();
    io::stdin()
        .read_line(&mut player_names_string)
        .expect("SOMETHING WENT WRONG GETTING YOUR NAMES");
    player_names_string = player_names_string.to_lowercase();
    
    if let None = player_names_string.find(" ") {
        println!("Please make sure to have your names seperated by a space");
    }
    let player_name_vec: Vec<&str> = player_names_string.split_whitespace().collect();

    let first_player = player_name_vec[rand::thread_rng().gen_range(0..2)];
    let second_player = if player_name_vec[0] == first_player {
        player_name_vec[1]
    } else {
        player_name_vec[0]
    };

    let mut player1_total_points: i32 = 0;
    let mut player2_total_points: i32 = 0;

    let mut player1_done: bool = false;
    let mut player2_done: bool = false;
    let mut someone_already_won: bool = false;
    println!("The GAME IS BEGINNING");

    loop {

        if !player1_done {
            if maybe_add_points(first_player) {
                handle_points(&mut player1_total_points);
                println!(
                    "{} now has {} total points",
                    first_player, player1_total_points
                );
            } else {
                player1_done = true;
            }
        }

        if player1_total_points > 21 {
            println!("{} went over 21, {} wins!", first_player, second_player);
            someone_already_won = true;
            break;
        }

        if !player2_done {
            if maybe_add_points(second_player) {
                handle_points(&mut player2_total_points);
                println!(
                    "{} now has {} total points",
                    second_player, player2_total_points
                );
            } else {
                player2_done = true;
            }
        }

        if player2_total_points > 21 {
            println!("{} went over 21, {} wins!", second_player, first_player);
            someone_already_won = true;
            break;
        }

        if player1_done && player2_done {
            break
        }
    }

    if !someone_already_won {
        if 21 - player1_total_points < 21 - player2_total_points {
            println!("{} won! They were closer to 21", first_player);
        } else {
            println!("{} won! They were closer to 21", second_player);
        }
    }
}

fn handle_points(player_points: &mut i32) {
    let random_number = rand::thread_rng().gen_range(0..12);
    *player_points += random_number;
}

fn maybe_add_points(player: &str) -> bool {
    println!(
        "{}, say yes to increase your points by a random amount, or say no if you think you're close enough to 21",
        player
    );

    loop {
        let mut player_answer: String = String::new();
        io::stdin()
            .read_line(&mut player_answer)
            .expect("SOMETHING WENT WRONG WONDERING IF PLAYER1 WANTS TO ADD POINTS");
        player_answer = player_answer.trim().to_lowercase();

        if player_answer == "yes".to_string() {
            return true;
        } else if player_answer == "no".to_string() {
            return false;
        } else {
            println!("Enter yes or no (you may have misstyped your answer)");
            continue;
        }
    }
}
