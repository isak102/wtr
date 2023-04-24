use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer};

mod datetime_format;
pub mod parameter;

pub use parameter::Parameter;

use self::parameter::{ParameterName, ParameterValue};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherReport { // TODO: add location field which is ignored by the deserializer
    #[serde(with = "datetime_format")]
    approved_time: DateTime<Utc>,
    #[serde(with = "datetime_format")]
    reference_time: DateTime<Utc>,
    geometry: Geometry,
    pub time_series: Vec<TimeSeries>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct TimeSeries {
    pub valid_time: DateTime<Utc>,
    pub parameters: HashMap<ParameterName, Parameter>,
}

impl<'de> Deserialize<'de> for TimeSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct __TimeSeriesInternal {
            #[serde(with = "datetime_format")]
            valid_time: DateTime<Utc>,
            parameters: Vec<Parameter>,
        }
        let internal: __TimeSeriesInternal = Deserialize::deserialize(deserializer)?;

        let mut parameters_map: HashMap<ParameterName, Parameter> = HashMap::new();

        for parameter in internal.parameters {
            parameters_map.insert(parameter.name.clone(), parameter);
        }

        Ok(TimeSeries {
            valid_time: internal.valid_time,
            parameters: parameters_map,
        })
    }
}

impl Display for TimeSeries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Valid time: {}. ", self.valid_time));
        for (_, parameter) in &self.parameters {
            write!(output, "{}, ", parameter)?;
        }
        write!(f, "{}", output)
    }
}
