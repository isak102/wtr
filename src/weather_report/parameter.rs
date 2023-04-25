use super::*;
use enum_iterator::Sequence;
use prettytable::color::{self, Color};

#[derive(Debug)]
pub struct Parameter {
    pub name: ParameterName,
    pub level_type: String,
    pub unit: Option<&'static str>,
    pub level: u32,
    pub values: Vec<ParameterValue>,
}

impl Parameter {
    /// Returns a optional tuple on the form of (Primary, Secondary). Secondary should be used as FG is
    /// primary is used as BG
    pub fn get_color(&self) -> Option<(Color, Color)> {
        // Define an updated ColorCutoffs struct
        struct ColorCutoffs {
            min: f64,
            max: f64,
            high_better: bool, // if a high value is better than a low value, for example temperature
        }

        macro_rules! color_cutoffs {
            ($min:expr, $max:expr, $high_better:expr) => {
                ColorCutoffs {
                    min: $min,
                    max: $max,
                    high_better: $high_better,
                }
            };
        }

        // TODO: read this from args or file
        fn get_color_cutoffs(param: &Parameter) -> ColorCutoffs {
            match param.name {
                // TODO: complete the questionable ones here, dont return color cutoff
                // for all colors
                ParameterName::t => color_cutoffs!(5.0, 10.0, true),
                ParameterName::ws => color_cutoffs!(0.0, 1.0, false),
                ParameterName::gust => color_cutoffs!(0.0, 1.0, false),
                ParameterName::tcc_mean => color_cutoffs!(0.0, 1.0, false),
                ParameterName::lcc_mean => color_cutoffs!(0.0, 1.0, false),
                ParameterName::mcc_mean => color_cutoffs!(0.0, 1.0, false),
                ParameterName::hcc_mean => color_cutoffs!(0.0, 1.0, false),
                ParameterName::pmin => color_cutoffs!(0.0, 1.0, false),
                ParameterName::pmax => color_cutoffs!(0.0, 1.0, false),
                ParameterName::pmean => color_cutoffs!(0.0, 1.0, false),
                ParameterName::pmedian => color_cutoffs!(0.0, 1.0, false),
                ParameterName::pcat => color_cutoffs!(0.0, 1.0, true), // IDK
                ParameterName::spp => color_cutoffs!(0.0, 1.0, false),
                ParameterName::tstm => color_cutoffs!(0.0, 1.0, false),
                ParameterName::msl => color_cutoffs!(0.0, 1.0, false), // IDK
                ParameterName::vis => color_cutoffs!(0.0, 1.0, true),
                ParameterName::wd => color_cutoffs!(0.0, 1.0, false), // IDK
                ParameterName::r => color_cutoffs!(0.0, 1.0, true),   // IDK
                ParameterName::Wsymb2 => color_cutoffs!(0.0, 1.0, false),
            }
        }

        let cutoffs = get_color_cutoffs(&self);
        let value = match &self.values.first().expect("All values only have one value") {
            ParameterValue::Decimal(value) => *value,
            ParameterValue::Integer(value) => *value as f64,
            ParameterValue::PercipitationCategory(_) => return None, // TODO: fix these
            ParameterValue::WeatherSymbol(_) => return None,
        };

        let color = {
            if cutoffs.high_better {
                if value > cutoffs.max {
                    (color::GREEN, color::BLACK)
                } else if value > cutoffs.min {
                    (color::YELLOW, color::BLACK)
                } else {
                    (color::RED, color::BLACK)
                }
            } else {
                if value < cutoffs.min {
                    (color::GREEN, color::BLACK)
                } else if value < cutoffs.max {
                    (color::YELLOW, color::BLACK)
                } else {
                    (color::RED, color::BLACK)
                }
            }
        };
        Some(color)
    }
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
