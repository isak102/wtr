use crate::types::Location;

use super::WeatherReport;
use serde_json::from_str;
use std::error::Error;

impl WeatherReport {
    pub async fn get(location: Location) -> Result<Self, Box<dyn Error>> {
        let (lon, lat) = location.coordinates();
        let url =
        format!("https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json", lon, lat);

        let response = reqwest::get(url).await.unwrap();
        let json = response.text().await.unwrap();

        let mut weather_report: WeatherReport = from_str(&json.as_str())?;
        weather_report.location = Some(location);

        Ok(weather_report)
    }
}
