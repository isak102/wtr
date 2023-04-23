use prettytable::{self, Cell, Row};

use super::weather_report::parameter::ParameterName;
use super::weather_report::WeatherReport;

pub fn show_forecast(
    weather_report: &WeatherReport,
    parameters: &[ParameterName],
    hours_to_show: Option<usize>,
) {
    let mut table = prettytable::Table::new();

    table.set_titles(Row::new(
        parameters
            .iter()
            .map(|p| Cell::new(p.to_string().as_str()))
            .collect(),
    ));

    let mut testrow = Row::new(Vec::with_capacity(2));
    testrow.insert_cell(1, Cell::new("test"));
    table.add_row(testrow);

    // let hours_to_show = hours_to_show.unwrap_or(weather_report.time_series.len());
    //
    // for time_series in weather_report.time_series.iter().take(hours_to_show) {
    //     table.add_row(row![
    //         time_series.time,
    //         time_series.temperature,
    //         time_series.precipitation,
    //         time_series.wind,
    //         time_series.clouds,
    //         time_series.weather
    //     ]);
    // }

    table.printstd();
}
