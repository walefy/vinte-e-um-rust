use crate::fetch;

pub fn shuffle_deck() -> String {
  // Retorna o id de um deck
  let mut deck_id: String = String::new();
  
  match fetch::get("https://deckofcardsapi.com/api/deck/new/shuffle/") {
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