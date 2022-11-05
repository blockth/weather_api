use dotenv;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Cli {
    pub city: String,
    pub country_code: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Weather,
    base: String,
    pub main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

// let client = reqwest::Client::new();
// client
//     .get(url)
//     .header(
//         "api-key",
//         dotenv::var("API_KEY") // added api key.it will return an Option,
//             .expect("Could not find API Key"),
//     ) //  if its None => expect err
//     .send()
//     .await //tokio is going to be conducting this
//     .expect("Failed to get response") // if we can not get the response
//     .text() //format it into string representation of json
//     .await
//     .expect("Failed to convert payload")

impl Forecast {
    pub async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        //https://api.openweathermap.org/data/2.5/weather?q=London&appid=1ae1fc95f76d189da6ce081a6073bcea
        let url: String = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city,
            country_code,
            dotenv::var("API").expect("Could not find API Key"),
        );

        println!("{}", url);
        let url = Url::parse(&*url)?; // propage error

        let response = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(response)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    details: Details,
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    pub humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}
