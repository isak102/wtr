use reqwest::Error;
use serde_json::from_str;
use weather_report::WeatherReport;

pub mod weather_report;

async fn fetch_json(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let json = response.text().await?;
    Ok(json)
}

#[tokio::main]
async fn main() {
    eprintln!("Starting...");
    let api_response =
        fetch_json("https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/16/lat/58/data.json")
            .await
            .unwrap();

    let parsed_response: WeatherReport = from_str(api_response.as_str()).unwrap();
    println!("{:?}", parsed_response);
}
