use super::WeatherReport;
use serde_json::from_str;
use std::error::Error;

pub async fn get_weather_report(url: &str) -> Result<WeatherReport, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let json = response.text().await?;

    match from_str(json.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}
