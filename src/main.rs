use core::panic;

use chrono::{DateTime, TimeZone, Utc};
use reqwest::Error;
use serde::{self, Deserialize, Deserializer};
use serde_json::from_str;

#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<f64>>,
}

#[derive(Debug)]
struct Parameter {
    name: String,
    level_type: String,
    level: u32,
    values: Vec<ParameterValue>,
}
impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ParameterInternal {
            name: String,
            #[serde(rename = "levelType")]
            level_type: String,
            level: u32,
            values: Vec<f64>,
        }

        let internal: ParameterInternal = Deserialize::deserialize(deserializer)?;
        let mut new_values = Vec::new();

        match internal.name.as_str() {
            "msl" | "t" | "vis" | "ws" | "gust" | "pmin" | "pmax" | "pmean" | "pmedian" => {
                for value in internal.values {
                    new_values.push(ParameterValue::Decimal(value));
                }
            }
            "pcat" => {
                for value in internal.values {
                    new_values.push(ParameterValue::Category(PercipitaionCategory::new(
                        value as u8,
                    )));
                }
            }
            "Wsymb2" => {
                for value in internal.values {
                    new_values.push(ParameterValue::Code(WeatherSymbol::new(value as u8)));
                }
            }
            _ => {
                for value in internal.values {
                    new_values.push(ParameterValue::Integer(value as i32));
                }
            }
        }

        Ok(Parameter {
            name: internal.name,
            level_type: internal.level_type,
            level: internal.level,
            values: new_values,
        })
    }
}

#[derive(Debug)]
struct WeatherSymbol(u8);
impl WeatherSymbol {
    fn new(value: u8) -> Self {
        if value > 27 || value == 0 {
            panic!("PercipitaionCategory should be 1-27")
        }
        WeatherSymbol(value)
    }
}

#[derive(Debug)]
struct PercipitaionCategory(u8);
impl PercipitaionCategory {
    fn new(value: u8) -> Self {
        if value > 6 {
            panic!("PercipitaionCategory should be 1-27")
        }
        PercipitaionCategory(value)
    }
}

#[derive(Debug)]
enum ParameterValue {
    Decimal(f64),
    Integer(i32),
    Category(PercipitaionCategory),
    Code(WeatherSymbol),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeSeries {
    #[serde(with = "datetime_format")]
    valid_time: DateTime<Utc>,
    parameters: Vec<Parameter>,
}

mod datetime_format {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(s.as_str())
            .unwrap()
            .with_timezone(&Utc))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    #[serde(with = "datetime_format")]
    approved_time: DateTime<Utc>,
    #[serde(with = "datetime_format")]
    reference_time: DateTime<Utc>,
    geometry: Geometry,
    time_series: Vec<TimeSeries>,
}

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

    let parsed_response: ApiResponse = from_str(api_response.as_str()).unwrap();
    println!("{:?}", parsed_response);
}
