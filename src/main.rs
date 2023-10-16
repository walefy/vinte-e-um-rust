mod deck;
mod fetch;
mod input;

use deck::draw_card;
use deck::shuffle_deck;
use input::get_input;

struct Player {
    score: i32,
}

impl Player {
    fn add_score(&mut self, points: i32) {
        self.score += points;
    }
}

fn get_points(card: String) -> i32 {
    return match card.as_str() {
        "QUEEN" | "JACK" | "KING" => 10,
        "ACE" => 1,
        _ => card.parse::<i32>().unwrap(),
    };
}

fn player_gets_card(deck_id: &str, player: &mut Player) {
    let card: String = draw_card(&deck_id);
    let points: i32 = get_points(card);

    player.add_score(points);
}

fn main() {
    let mut global_player_score: Player = Player { score: 0 };
    let mut global_computer_score: Player = Player { score: 0 };

    let mut control_loop: bool = true;

    while control_loop {
        let mut local_player_score: Player = Player { score: 0 };
        let mut local_computer_score: Player = Player { score: 0 };

        let deck_id: String = shuffle_deck();

        println!("{}", "-".repeat(30));
        println!("Você: {} pontos", global_player_score.score);
        println!("Computador: {} pontos", global_computer_score.score);
        println!("{}", "-".repeat(30));

        for _ in 0..52 {
            let choice: String = get_input("\n[M] More\n[S] Stop\n[E] exit\n>> ");

            if choice.to_lowercase() == "m" {
                player_gets_card(&deck_id, &mut local_player_score);
                println!("Você tem: {} pontos", local_player_score.score);

                if local_player_score.score > 21 {
                    break;
                }
            } else if choice.to_lowercase() == "e" {
                println!("Você saiu do jogo!");
                control_loop = false;
                break;
            } else {
                while local_computer_score.score < 17 {
                    player_gets_card(&deck_id, &mut local_computer_score);
                    println!("O computador tem: {} pontos", local_computer_score.score);

                    if local_computer_score.score > 21
                        || local_computer_score.score > local_player_score.score
                    {
                        break;
                    }
                }
                break;
            }
        }

        if !control_loop {
            break;
        }

        if (local_player_score.score > local_computer_score.score && local_player_score.score <= 21)
            || local_computer_score.score > 21
        {
            println!("Você venceu!");
            global_player_score.add_score(1);
        } else if local_player_score.score == local_computer_score.score {
            println!("Empate!");
        } else {
            println!("O computador venceu!");
            global_computer_score.add_score(1);
        }
    }
}
