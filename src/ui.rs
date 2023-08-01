use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, Flight};

pub fn draw_flight_data<B>(app: &App, frame: &mut Frame<B>)
where
    B: Backend,
{
    // Create a data collection to hold flight data
    let flight_data: Vec<_> = app.flights
        .iter()
        .map(|flight| {
            let row = vec![
                Cell::from(flight.id.clone()),
                Cell::from(flight.departure_location.clone()),
                Cell::from(flight.arrival_location.clone()),
                Cell::from(flight.departure_time.to_string()),
                Cell::from(flight.arrival_time.to_string()),
            ];
            Row::new(row)
        })
        .collect();

    // Define a table to hold the flight data
    let flight_table = Table::new(flight_data)
        .header(
            Row::new(vec!["ID", "Departure", "Arrival", "Departure Time", "Arrival time"])
                .style(Style::default().fg(Color::Yellow)),
        )
        .block(Block::default().title("Flight Table").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(30),
            Constraint::Length(30),
            Constraint::Length(30),
            Constraint::Length(30),
        ]);

    // Render the table in the frame
    frame.render_widget(flight_table, frame.size());
}

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    // Draw the flight data table in the upper half of the frame
    draw_flight_data(app, frame);

    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.counter
        ))
    ), chunks[1];
}
