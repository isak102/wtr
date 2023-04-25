use crate::{types::Location, weather_report::parameter::ParameterName};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// How many hours to show
    #[arg(short = 'c', long)]
    pub hours: Option<u8>,
    /// List of parameters to show
    // TODO: add default value here, make the possible values list shorter
    #[arg(short, long, num_args(1..ParameterName::count()), value_name = "PARAMETER", value_enum)]
    pub parameters: Option<Vec<ParameterName>>,
    /// The location to use
    #[arg(short, default_value_t=crate::types::Location::Uppsala)]
    pub location: Location,
}
