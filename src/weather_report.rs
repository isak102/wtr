use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer};

mod datetime_format;
mod parameter;

use parameter::Parameter;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherReport {
    #[serde(with = "datetime_format")]
    approved_time: DateTime<Utc>,
    #[serde(with = "datetime_format")]
    reference_time: DateTime<Utc>,
    geometry: Geometry,
    time_series: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeSeries {
    #[serde(with = "datetime_format")]
    valid_time: DateTime<Utc>,
    parameters: Vec<Parameter>,
}
