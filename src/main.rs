use clap::Parser;
use dirs::home_dir;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::{collections::HashMap, ffi::OsString, fs::{self, File}, io::{self, Write}, path::PathBuf};
#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "This program allows you to convert currencies using the openexchangerates.org API."
)]
struct Cli {
    amount: f32,
    orgin_currency: String,
    final_currency: String,
    #[arg(short, long, help = "Gets latest exchange rates using your API key.")]
    refresh: bool,
}
#[derive(Deserialize, Serialize)]
struct ApiReturn {
    disclaimer: String,
    license: String,
    timestamp: f32,
    base: String,
    rates: HashMap<String, f32>,
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.refresh {
        save_currencies(request_rates().await);
    }
    convert_currencies(load_currencies().rates, cli);
}
async fn request_rates() -> ApiReturn {
    let app_id = app_id();
    let url = format!(
        "https://openexchangerates.org/api/latest.json?app_id={app_id}&base=USD&prettyprint=true&show_alternative=true"
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("accept"),
        HeaderValue::from_static("application/json"),
    );
    let client = Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    serde_json::from_str(response.as_str()).unwrap()
}
fn app_id() -> String {
    let mut path = get_data_dir();
    path.push("app_id");
    let app_id_string = fs::read_to_string(path.as_os_str());
    match app_id_string {
        Ok(app_id_string) => app_id_string,
        _err => {
            let mut app_id = String::new();
            println!("Input your openexchangerates.org app id");
            io::stdin().read_line(&mut app_id).unwrap();
            let mut file = File::create(path).unwrap();
            file.write_all(app_id.as_bytes()).unwrap();
            app_id
        }
    }
}
fn save_currencies(save_json: ApiReturn) {
    let mut path = get_data_dir();
    path.push("currency.json");
    fs::write(path.as_os_str(), to_vec(&save_json).unwrap())
        .expect("Should create or overwrite file.");
}
fn load_currencies() -> ApiReturn {
    let mut path = get_data_dir();
    path.push("currency.json");
    serde_json::from_slice(&fs::read(path).expect("Re-run and refresh your currencies.")).unwrap()
}
fn convert_currencies(currency_map: HashMap<String, f32>, arguments: Cli) {
    let orgin_multiplier = currency_map
        .get(&arguments.orgin_currency)
        .expect("Invalid currency name");
    let final_multiplier = currency_map
        .get(&arguments.final_currency)
        .expect("Invalid currency name");
    let final_amount = orgin_multiplier * final_multiplier * arguments.amount;
    println!(
        "{} {} is {} {}",
        arguments.amount, arguments.orgin_currency, final_amount, arguments.final_currency
    )
}

fn get_data_dir() -> OsString {
    PathBuf::from(
        &[
            home_dir()
                .unwrap_or_else(|| "".into())
                .display()
                .to_string(),
            "/Documents/".to_string(),
        ]
        .join(""),
    ).into_os_string()
}
