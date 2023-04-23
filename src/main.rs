use weather_report::WeatherReport;

pub mod io;
pub mod types;
pub mod ui;
pub mod weather_report;

#[tokio::main]
async fn main() {
    eprintln!("Starting...");
    let weather_report = io::get_weather_report(types::Location::Sollentuna)
        .await
        .unwrap();

    ui::show_forecast(
        &weather_report,
        &[
            weather_report::parameter::ParameterName::t,
            weather_report::parameter::ParameterName::pcat,
            weather_report::parameter::ParameterName::Wsymb2,
        ],
        None,
    );
    //
    // for time_series in weather_report.time_series {
    //     println!("{}", time_series);
    // }
}
