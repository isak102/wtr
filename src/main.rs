use weather_report::WeatherReport;

pub mod io;
pub mod types;
pub mod weather_report;

#[tokio::main]
async fn main() {
    eprintln!("Starting...");
    let weather_report = io::get_weather_report(types::Location::Sollentuna).await.unwrap();

    for time_series in weather_report.time_series {
        println!("{}", time_series);
    }
}
