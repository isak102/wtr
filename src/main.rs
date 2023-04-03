use weather_report::WeatherReport;

pub mod io;
pub mod weather_report;

#[tokio::main]
async fn main() {
    eprintln!("Starting...");
    let weather_report = io::get_weather_report("https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/16/lat/58/data.json").await;
    
    println!("{:?}", weather_report);
}
