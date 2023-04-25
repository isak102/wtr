use clap::Parser;
use weather_report::parameter::ParameterName;
use weather_report::WeatherReport;

pub mod args;
pub mod types;
pub mod ui;
pub mod weather_report;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let weather_report = WeatherReport::get(args.location)
        .await
        .expect("Error getting weather report");

    let default_parameters = &[
        ParameterName::t,
        ParameterName::ws,
        ParameterName::gust,
        ParameterName::tcc_mean,
        ParameterName::pmin,
        ParameterName::pmax,
        ParameterName::Wsymb2,
    ];
    
    ui::show_forecast(
        &weather_report,
        {
            match &args.parameters {
                Some(parameters) => Some(parameters.as_slice()),
                None => Some(default_parameters),
            }
        },
        Some(args.hours.unwrap_or(7)),
    );
}
