pub mod regression;
pub mod metrics;
pub mod visualization;
pub mod io;
pub mod pdf; // Adicione o módulo pdf

pub use regression::{linear_regression, predict};
pub use metrics::{r_squared, mean_squared_error, mean, median, standard_deviation, min, max};
pub use visualization::plot_series_and_regression;
pub use io::{read_csv, read_json, read_file};
pub use pdf::generate_pdf; // Corrija a exportação da função generate_pdf

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (a, b) = linear_regression(&x, &y).unwrap();

        assert!((a - 2.0).abs() < 1e-6, "Inclinação incorreta");
        assert!((b - 0.0).abs() < 1e-6, "Intercepto incorreto");
    }

    #[test]
    fn test_r_squared() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (a, b) = linear_regression(&x, &y).unwrap();
        let r2 = r_squared(&x, &y, a, b).unwrap();

        assert!((r2 - 1.0).abs() < 1e-6, "R² incorreto");
    }

    #[test]
    fn test_mean_squared_error() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (a, b) = linear_regression(&x, &y).unwrap();
        let mse = mean_squared_error(&x, &y, a, b).unwrap();

        assert!((mse - 0.0).abs() < 1e-6, "MSE incorreto");
    }

    #[test]
    fn test_predict() {
        let future_x = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        let a = 2.0;
        let b = 0.0;
        let predictions = predict(&future_x, a, b);

        assert_eq!(predictions, vec![12.0, 14.0, 16.0, 18.0, 20.0], "Previsões incorretas");
    }
}