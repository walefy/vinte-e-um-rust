use crate::fetch;
use serde_json::Value;

pub fn draw_card(deck_id: &str) -> String {
  let formatted_url: &String = &format!("https://deckofcardsapi.com/api/deck/{}/draw/?count=1", deck_id);
  let response: Result<Value, String> = fetch::get(formatted_url);
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