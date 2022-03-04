use dotenv;
use std::io;
use ureq;
use chrono::{TimeZone, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Message {
    // coord: Coord,
    weather: Vec<Weather>,
    main: Main,
    dt: i64,
    timezone: i64
}
// #[derive(Deserialize, Debug)]
// struct Coord {
//     lat: f32,
//     lon: f32,
// }

#[derive(Deserialize, Debug)]
struct Weather {
    // id: i32,
    // main: String,
    description: String,
    // icon: String,
}
#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    // pressure: i32,
    humidity: i32,
}

//@Todo
// const COUNTRY_CODES =
fn main() -> Result<(), ureq::Error> {
    println!("What city?");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;
    //@todo get city and country as well.
    let key = dotenv::var("API_KEY").unwrap();
    let url = "http://api.openweathermap.org/data/2.5/weather?q=".to_string()
        + &city
        + "&units=metric&appid="
        + &key;
    let body: Message = ureq::get(&url).call()?.into_json()?;
    // let time = Utc.timestamp(body.dt, 0);
    // println!("{}", time.format("%Y-%m-%d %H:%M:%S"));
    println!("The Current weather is {}", body.weather[0].description);
    println!("The Current temperature is {}", body.main.temp);
    println!("It currently feels like {}", body.main.feels_like);
    println!("The min temp today is {}", body.main.temp_min);
    println!("The max temp today is {}", body.main.temp_max);
    println!("The humidity is {}", body.main.humidity);
    Ok(())
}
