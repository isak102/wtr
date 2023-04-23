use crate::types::Location;

use super::WeatherReport;
use serde_json::from_str;
use std::error::Error;

pub async fn get_weather_report(location: Location) -> Result<WeatherReport, Box<dyn Error>> {
    let (lon, lat) = location.coordinates();
    let url =
        format!("https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json", lon, lat);

    let response = reqwest::get(url).await.unwrap();
    let json = response.text().await.unwrap();

    match from_str(json.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e)),
    }
}
