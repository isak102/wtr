use clap::Parser;
use weather_report::WeatherReport;

pub mod args;
pub mod io;
pub mod types;
pub mod ui;
pub mod weather_report;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let weather_report = io::get_weather_report(args.location)
        .await
        .expect("Error getting weather report");

    ui::show_forecast(
        &weather_report,
        {
            match &args.parameters {
                Some(parameters) => Some(parameters.as_slice()),
                None => None,
            }
        },
        Some(args.hours.unwrap_or(7)),
    );
}
