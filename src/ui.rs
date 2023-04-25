use enum_iterator;
use prettytable::{self, format, Attr, Cell, Row};

use chrono::{DateTime, Local};

use super::weather_report::parameter::ParameterName;
use super::weather_report::WeatherReport;

pub fn show_forecast(
    weather_report: &WeatherReport,
    parameters: Option<&[ParameterName]>,
    hours_to_show: Option<u8>,
) {
    let all_parameters = enum_iterator::all::<ParameterName>().collect::<Vec<_>>();

    let parameter_names = parameters.unwrap_or(all_parameters.as_slice());
    let hours_to_show = hours_to_show.unwrap_or(weather_report.time_series.len() as u8);

    let mut table = prettytable::Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    let mut titles: Vec<Cell> = Vec::new();
    titles.push(Cell::new("Time"));

    for parameter in parameter_names {
        titles.push(Cell::new(parameter.to_string().as_str()));
    }

    table.set_titles(Row::new(titles));

    for time_series in weather_report
        .time_series
        .iter()
        .take(hours_to_show as usize)
    {
        let mut row = Row::new(Vec::new());

        let local_time: DateTime<Local> = time_series.valid_time.into();
        row.add_cell(Cell::new(
            local_time
                .format_localized("%a %d/%m %H:%M", chrono::Locale::sv_SE)
                .to_string()
                .as_str(),
        ));

        for name in parameter_names {
            let mut f = |s, c: Option<(u32, u32)>| {
                let cell = Cell::new(s);

                let cell = match c {
                    Some(color) => cell
                        .with_style(Attr::ForegroundColor(color.0))
                        .with_style(Attr::Bold),
                    None => cell,
                };

                row.add_cell(cell);
            };

            if let Some(parameter) = time_series.parameters.get(name) {
                f(parameter.to_string().as_str(), parameter.get_color());
            } else {
                f("N/A", None)
            }
        }
        table.add_row(row);
    }

    println!(
        "Location: {}",
        &weather_report
            .location
            .as_ref()
            .expect("Location should always be set")
    );
    table.printstd();
}
