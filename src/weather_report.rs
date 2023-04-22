use std::fmt::{Display, Formatter, Write};

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
    pub time_series: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    #[serde(with = "datetime_format")]
    valid_time: DateTime<Utc>,
    parameters: Vec<Parameter>,
}

impl Display for TimeSeries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Valid time: {}. ", self.valid_time));
        for parameter in &self.parameters {
            write!(output, "{}, ", parameter)?;
        }
        write!(f, "{}", output)
    }
}
