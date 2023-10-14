use reqwest::blocking::get as blocking_get;
use reqwest::blocking::Response;
use serde_json::Value;

pub fn get(url: &str) -> Result<Value, String> {
    let response: Response = blocking_get(url).unwrap();

    if response.status().is_success() {
        let json: Value = response.json().unwrap();
        return Ok(json);
    }

    let error: String = format!("Um erro ocorreu: {}", response.status());
    Err(error)
}
