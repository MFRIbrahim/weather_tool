use anyhow::bail;
use clap::{App, Arg};
use isahc::prelude::*;
use lazy_static::lazy_static;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::process;

lazy_static! {
    static ref WEATHER_CODE: HashMap<usize, &'static str> = [
        (4201, "Heavy Rain"),
        (4001, "Rain"),
        (4200, "Light Rain"),
        (6201, "Heavy Freezing Rain"),
        (6001, "Freezing Rain"),
        (6200, "Light Freezing Rain"),
        (6000, "Freezing Drizzle"),
        (4000, "Drizzle"),
        (7101, "Heavy Ice Pellets"),
        (7000, "Ice Pellets"),
        (7102, "Light Ice Pellets"),
        (5101, "Heavy Snow"),
        (5000, "Snow"),
        (5100, "Light Snow"),
        (5001, "Flurries"),
        (8000, "Thunderstorm"),
        (2100, "Light Fog"),
        (2000, "Fog"),
        (1001, "Cloudy"),
        (1102, "Mostly Cloudy"),
        (1101, "Partly Cloudy"),
        (1100, "Mostly Clear"),
        (1000, "Clear, Sunny"),
    ]
    .iter()
    .cloned()
    .collect();
}

fn main() {
    let matches = App::new("Weather Tool")
    .author("MFRIbrahim")
    .about(
"This tool gets the current weather conditions for the given latitude and longitude with the
provided API key for the tomorrow.io weather api."
    )
    .arg(
        Arg::with_name("LAT")
            .help(
"The latitude of the location. "
            )
            .required(true)
    )
    .arg(
        Arg::with_name("LONG")
            .help(
"The longitude of the location. "
            )
            .required(true)
    )
    .arg(
        Arg::with_name("KEY")
            .help(
"The tomorrow.io weather api key. "
            )
            .required(true)
    )
    .get_matches();

    let latitude = matches.value_of("LAT").unwrap();
    let longitude = matches.value_of("LONG").unwrap();
    let api_key = matches.value_of("KEY").unwrap();

    let url = format!(
        "https://api.tomorrow.io/v4/timelines?location={},{}&fields=temperature&fields=humidity&fields=windSpeed&fields=weatherCode&timesteps=1m&units=metric&apikey={}",
        latitude,
        longitude,
        api_key,
    );

    let weather_data = get_weather_data(&url[..]).unwrap_or_else(|err| {
        eprintln!("Problem getting the weather data from tomorrow.io: {}", err);
        process::exit(1);
    });

    let parsed_data = parse_weather_data(&weather_data[..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing the weather data: {}", err);
        process::exit(1);
    });

    println!(
        "weather: {}\ntemperature: {}\nhumidity: {}\nwindspeed: {}",
        WEATHER_CODE[&(parsed_data["data"]["timelines"][0]["intervals"][0]["values"]["weatherCode"]
            .as_u64()
            .unwrap() as usize)],
        parsed_data["data"]["timelines"][0]["intervals"][0]["values"]["temperature"],
        parsed_data["data"]["timelines"][0]["intervals"][0]["values"]["humidity"],
        parsed_data["data"]["timelines"][0]["intervals"][0]["values"]["windSpeed"]
    )
}

fn get_weather_data(url: &str) -> Result<String, anyhow::Error> {
    let mut res = isahc::get(url)?;
    let weather_data = res.text()?;
    Ok(weather_data)
}

fn parse_weather_data(weather_data: &str) -> Result<Map<String, Value>, anyhow::Error> {
    let parsed: Value = serde_json::from_str(weather_data)?;
    let map: Map<String, Value> = match parsed.as_object() {
        Some(obj) => obj.clone(),
        None => bail!("Parsed data can not be converted into an object"),
    };
    Ok(map)
}
