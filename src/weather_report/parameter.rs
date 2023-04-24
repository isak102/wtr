use super::*;
use enum_iterator::Sequence;

#[derive(Debug)]
pub struct Parameter {
    pub name: ParameterName,
    pub level_type: String,
    pub unit: Option<&'static str>,
    pub level: u32,
    pub values: Vec<ParameterValue>,
}

#[allow(non_camel_case_types)]
#[derive(
    Deserialize, Debug, strum_macros::Display, Clone, Eq, PartialEq, Hash, Sequence, clap::ValueEnum,
)]
pub enum ParameterName {
    t,
    ws,
    gust,
    tcc_mean,
    lcc_mean,
    mcc_mean,
    hcc_mean,
    pmin,
    pmax,
    pmean,
    pmedian,
    pcat,
    spp,
    tstm,
    msl,
    vis,
    wd,
    r,
    Wsymb2,
}

#[derive(Debug, Clone)]
pub enum ParameterValue {
    Decimal(f64),
    Integer(i32),
    PercipitationCategory(u8),
    WeatherSymbol(u8),
}

impl ParameterName {
    fn get_unit(self) -> Option<&'static str> {
        match self {
            ParameterName::msl => Some("hPa"),
            ParameterName::t => Some("C"),
            ParameterName::vis => Some("km"),
            ParameterName::wd => Some("degree"),
            ParameterName::ws => Some("m/s"),
            ParameterName::r => Some("%"),
            ParameterName::tstm => Some("%"),
            ParameterName::tcc_mean => Some("octas"),
            ParameterName::lcc_mean => Some("octas"),
            ParameterName::mcc_mean => Some("octas"),
            ParameterName::hcc_mean => Some("octas"),
            ParameterName::gust => Some("m/s"),
            ParameterName::pmin => Some("mm/h"),
            ParameterName::pmax => Some("mm/h"),
            ParameterName::spp => Some("%"),
            ParameterName::pcat => None,
            ParameterName::pmean => Some("mm/h"),
            ParameterName::pmedian => Some("mm/h"),
            ParameterName::Wsymb2 => None,
        }
    }

    pub fn count() -> usize {
        enum_iterator::all::<ParameterName>().count()
    }
}

impl Display for ParameterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn pcat_to_str(code: &u8) -> &'static str {
            match code {
                0 => "No precipitation",
                1 => "Snow",
                2 => "Snow and rain",
                3 => "Rain",
                4 => "Drizzle",
                5 => "Freezing rain",
                6 => "Freezing drizzle",
                _ => panic!("Unknown precipitation code {}", code),
            }
        }

        fn wsymb_to_str(code: &u8) -> &'static str {
            match code {
                1 => "Clear sky",
                2 => "Nearly clear sky",
                3 => "Variable cloudiness",
                4 => "Halfclear sky",
                5 => "Cloudy sky",
                6 => "Overcast",
                7 => "Fog",
                8 => "Light rain showers",
                9 => "Moderate rain showers",
                10 => "Heavy rain showers",
                11 => "Thunderstorm",
                12 => "Light sleet showers",
                13 => "Moderate sleet showers",
                14 => "Heavy sleet showers",
                15 => "Light snow showers",
                16 => "Moderate snow showers",
                17 => "Heavy snow showers",
                18 => "Light rain",
                19 => "Moderate rain",
                20 => "Heavy rain",
                21 => "Thunder",
                22 => "Light sleet",
                23 => "Moderate sleet",
                24 => "Heavy sleet",
                25 => "Light snowfall",
                26 => "Moderate snowfall",
                27 => "Heavy snowfall",
                _ => panic!("Unknown weather code {}", code),
            }
        }

        match self {
            Self::Decimal(v) => write!(f, "{}", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::PercipitationCategory(c) => write!(f, "{}", pcat_to_str(c)),
            Self::WeatherSymbol(c) => write!(f, "{}", wsymb_to_str(c)),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        assert!(
            self.values.len() == 1,
            "Parameters should always have exactly one value"
        );

        write!(
            f,
            "{}{}",
            self.values[0].to_string(),
            self.unit.unwrap_or("")
        )
    }
}

impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct __ParameterInternal {
            name: ParameterName,
            level_type: String,
            level: u32,
            values: Vec<f64>,
        }

        let internal: __ParameterInternal = Deserialize::deserialize(deserializer)?;
        let mut new_values = Vec::new();

        match internal.name {
            ParameterName::msl
            | ParameterName::t
            | ParameterName::vis
            | ParameterName::ws
            | ParameterName::gust
            | ParameterName::pmin
            | ParameterName::pmax
            | ParameterName::pmean
            | ParameterName::pmedian => {
                for value in internal.values {
                    new_values.push(ParameterValue::Decimal(value));
                }
            }
            ParameterName::pcat => {
                for value in internal.values {
                    new_values.push(ParameterValue::PercipitationCategory(value as u8));
                }
            }
            ParameterName::Wsymb2 => {
                for value in internal.values {
                    new_values.push(ParameterValue::WeatherSymbol(value as u8));
                }
            }
            _ => {
                for value in internal.values {
                    new_values.push(ParameterValue::Integer(value as i32));
                }
            }
        }

        Ok(Parameter {
            name: internal.name.clone(),
            level_type: internal.level_type,
            unit: internal.name.get_unit(),
            level: internal.level,
            values: new_values,
        })
    }
}
