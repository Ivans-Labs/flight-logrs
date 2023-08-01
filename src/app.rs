/// State and application logic

use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// All flights
    pub flights: Vec<Flight>,
}

/// Flight struct.
#[derive(Debug)]
pub struct Flight {
    /// Unique Id for each flight
    pub id: String,
    /// Departure and arrival locations
    pub departure_location: String,
    pub arrival_location: String,
    /// Departure and arrival times
    pub departure_time: String,
    pub arrival_time: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            flights: vec![
                Flight {
                    id: "1".to_string(),
                    departure_location: "LAX".to_string(),
                    arrival_location: "SFO".to_string(),
                    departure_time: "2021-01-01 00:00:00".to_string(),
                    arrival_time: "2021-01-01 01:00:00".to_string(),
                },
                Flight {
                    id: "2".to_string(),
                    departure_location: "SFO".to_string(),
                    arrival_location: "LAX".to_string(),
                    departure_time: "2021-01-01 02:00:00".to_string(),
                    arrival_time: "2021-01-01 03:00:00".to_string(),
                },
            ],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
