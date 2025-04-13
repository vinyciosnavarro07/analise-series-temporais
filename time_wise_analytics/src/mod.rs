pub mod regression;
pub mod metrics;
pub mod visualization;
pub mod io;

pub use regression::{linear_regression, predict};
pub use metrics::{r_squared, mean_squared_error};
pub use visualization::plot_series_and_regression;
pub use io::read_csv;