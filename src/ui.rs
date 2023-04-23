use prettytable::{self, Cell, Row};
use enum_iterator;

use super::weather_report::parameter::ParameterName;
use super::weather_report::WeatherReport;

pub fn show_forecast(
    weather_report: &WeatherReport,
    parameters: Option<&[ParameterName]>,
    hours_to_show: Option<usize>,
) {
    let all_parameters = enum_iterator::all::<ParameterName>().collect::<Vec<_>>();

    let parameters = parameters.unwrap_or(all_parameters.as_slice());
    let hours_to_show = hours_to_show.unwrap_or(weather_report.time_series.len());

    let mut table = prettytable::Table::new();

    let mut titles: Vec<Cell> = Vec::new();
    titles.push(Cell::new("Time"));

    for parameter in parameters {
        titles.push(Cell::new(parameter.to_string().as_str()));
    }

    table.set_titles(Row::new(titles));

    for time_series in weather_report.time_series.iter().take(hours_to_show) {
        let mut row = Row::new(Vec::new());
        row.add_cell(Cell::new(time_series.valid_time.to_string().as_str()));

        for key in parameters {
            let mut f = |s| {
                row.add_cell(Cell::new(s));
            };

            if let Some(value) = time_series.parameters.get(key) {
                f(value.values[0].to_string().as_str())
            } else {
                f("N/A")
            }
        }
        table.add_row(row);
    }

    table.printstd();
}
