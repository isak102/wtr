use weather_report::WeatherReport;

pub mod io;
pub mod types;
pub mod ui;
pub mod weather_report;

use weather_report::parameter::ParameterName;

#[tokio::main]
async fn main() {
    let weather_report = io::get_weather_report(types::Location::Sollentuna)
        .await
        .unwrap();

    ui::show_forecast(
        &weather_report,
        Some(&[
            ParameterName::t,
            ParameterName::ws,
            ParameterName::Wsymb2,
            ParameterName::tcc_mean,
        ]),
        Some(5),
    );
}
