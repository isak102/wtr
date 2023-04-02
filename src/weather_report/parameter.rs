use super::*;

#[derive(Debug)]
pub struct Parameter {
    name: String,
    level_type: String,
    level: u32,
    values: Vec<ParameterValue>,
}

#[derive(Debug)]
enum ParameterValue {
    Decimal(f64),
    Integer(i32),
    Category(PercipitaionCategory),
    Code(WeatherSymbol),
}

#[derive(Debug)]
struct PercipitaionCategory(u8);
#[derive(Debug)]
struct WeatherSymbol(u8);

impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct __ParameterInternal {
            name: String,
            level_type: String,
            level: u32,
            values: Vec<f64>,
        }

        let internal: __ParameterInternal = Deserialize::deserialize(deserializer)?;
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

impl WeatherSymbol {
    fn new(value: u8) -> Self {
        if value > 27 || value == 0 {
            panic!("PercipitaionCategory should be 1-27")
        }
        WeatherSymbol(value)
    }
}

impl PercipitaionCategory {
    fn new(value: u8) -> Self {
        if value > 6 {
            panic!("PercipitaionCategory should be 1-27")
        }
        PercipitaionCategory(value)
    }
}
