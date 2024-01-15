use std::collections::HashMap;
use std::env;

const YEAR: u32 = 2023;
const SESSION_COOKIE_ENV_VAR: &str = "SESSION_COOKIE";

pub enum Part {
    ONE,
    TWO,
}

impl Part {
    fn value(&self) -> u32 {
        match *self {
            Part::ONE => 1,
            Part::TWO => 2,
        }
    }
}

pub fn download_input(day: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://adventofcode.com/{year}/day/{day}/input",
        year = YEAR,
        day = day
    );
    let response = reqwest::blocking::get(url)?;
    let input_text = response.text()?;
    Ok(input_text)
}

pub fn submit_answer(
    day: u32,
    part: Part,
    answer: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://adventofcode.com/{year}/day/{day}/answer",
        year = YEAR,
        day = day
    );
    let mut body: HashMap<&str, String> = HashMap::new();
    body.insert("level", part.value().to_string());
    body.insert("answer", answer);

    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(&body).send()?;
    let response_text = response.text()?;

    Ok(response_text)
}

fn get_session_cookie() -> String {
    env::var(SESSION_COOKIE_ENV_VAR).unwrap_or_else(|_error| {
        panic!(
            "No session cookie provided via {:?} env var",
            SESSION_COOKIE_ENV_VAR
        )
    })
}
