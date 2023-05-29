mod fetch;
mod input;
mod deck;

use deck::shuffle_deck;
use deck::draw_card;
use input::get_input;

struct Player {
    score: i32
}

fn get_points(card: String) -> i32 {
    return match card.as_str() {
        "QUEEN" | "JACK" | "KING" => 10,
        "ACE" => 1,
        _ => card.parse::<i32>().unwrap()
    };
}

fn player_gets_card(deck_id: &str, player: &mut Player) {
    let card: String = draw_card(&deck_id);
    let points: i32 = get_points(card);

    player.score += points;
}

fn main() {
    let mut player1: Player = Player { score: 0 };
    let mut player2: Player = Player { score: 0 };

    let deck_id: String = shuffle_deck();

    for _ in 0..52 {
        let choice: String = get_input("\n[M] More\n[S] Stop\n>> ");

        if choice.to_lowercase() == "m" {
            player_gets_card(&deck_id, &mut player1);
            println!("Você tem: {} pontos", player1.score);
            
            if player1.score > 21 {
                break;
            }
        } else {
            while player2.score < 17 {
                player_gets_card(&deck_id, &mut player2);
                println!("O computador tem: {} pontos", player2.score);

                if player2.score > 21 {
                    break;
                }
            }
            break;
        }
    }

    if (player1.score > player2.score && player1.score <= 21) || player2.score > 21 {
        println!("Você venceu!");
    } else if player1.score == player2.score {
        println!("Empate!");
    }
    else {
        println!("O computador venceu!");
    }
}
