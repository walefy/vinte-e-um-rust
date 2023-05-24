use reqwest::blocking::Response;
use reqwest::blocking::get;
use serde_json::Value;
use std::io;
use std::io::Write;

struct Player {
    score: i32
}

fn fetch(url: &str) -> Result<Value, String> {
    let response: Response = get(url).unwrap();

    if response.status().is_success() {
        let json: Value = response.json().unwrap();
        return Ok(json);
    }

    let error: String = format!("Um erro ocorreu: {}", response.status());
    Err(error)
}

fn shuffle_deck() -> String {
    // Retorna o id de um deck
    let mut deck_id: String = String::new();
    
    match fetch("https://deckofcardsapi.com/api/deck/new/shuffle/") {
        Ok(json) => {
            let id: Option<&str> = json["deck_id"].as_str();

            match id {
                Some(value) => deck_id = value.to_string(),
                None => println!("Failed"),
            }
        },
        Err(err) => println!("Error: {}", err),
    }

    deck_id
}

fn draw_card(deck_id: &str) -> String {
    let formatted_url: &String = &format!("https://deckofcardsapi.com/api/deck/{}/draw/?count=1", deck_id);
    let response: Result<Value, String> = fetch(formatted_url);
    let mut new_card: String = String::new();

    match response {
        Ok(json) => {
            let card: Option<&str> = json["cards"][0]["value"].as_str();
            
            match card {
                Some(value) => new_card = value.to_string(),
                None => println!("Failed"),
            }
        },
        Err(error) => println!("Error: {}", error)
    }

    new_card
}

fn get_input(msg: &str) -> String {
    print!("{}", msg);

    io::stdout().flush().unwrap();


    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    input.trim().to_string()
}

fn get_points(card: String) -> i32 {
    return match card.as_str() {
        "QUEEN" | "JACK" | "KING" => 10,
        "ACE" => 1,
        _ => card.parse::<i32>().unwrap()
    };
}

fn player_gets_card(deck_id: &str, player: &mut Player, is_computer: bool) {
    let card: String = draw_card(&deck_id);
    let points: i32 = get_points(card);

    player.score += points;
    
    if is_computer {
        println!("O computador tem: {} pontos", player.score);
        return;
    }
    println!("Você tem: {} pontos", player.score);
}

fn main() {
    let mut player1: Player = Player { score: 0 };
    let mut player2: Player = Player { score: 0 };

    let deck_id: String = shuffle_deck();
    let mut remaining: i32 = 52;

    for _ in 0..remaining {
        let choice: String = get_input("\n[M] More\n[S] Stop\n>> ");

        if choice.to_lowercase() == "m" {
            player_gets_card(&deck_id, &mut player1, false);

            if player1.score > 21 {
                break;
            }
        } else {
            while player2.score < 17 {
                player_gets_card(&deck_id, &mut player2, true);

                if player2.score > 21 {
                    break;
                }
            }
            break;
        }

        remaining -= 1;
    }

    if player1.score > player2.score && player1.score <= 21 || player2.score > 21 {
        println!("Você venceu!");
    } else if player1.score == player2.score {
        println!("Empate!");
    }
    else {
        println!("O computador venceu!");
    }
}
