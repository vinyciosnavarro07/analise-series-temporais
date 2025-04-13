/// Realiza a regressão linear em uma série temporal.
/// Retorna os coeficientes `a` (inclinação) e `b` (intercepto).
pub fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() != y.len() || x.is_empty() {
        return None; // Verifica entradas inválidas
    }

    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominator = n * sum_x2 - sum_x * sum_x;
    if denominator == 0.0 {
        return None; // Evita divisão por zero
    }

    let a = (n * sum_xy - sum_x * sum_y) / denominator;
    let b = (sum_y * sum_x2 - sum_x * sum_xy) / denominator;

    Some((a, b))
}

/// Realiza previsões com base nos coeficientes da regressão linear.
pub fn predict(x: &[f64], a: f64, b: f64) -> Vec<f64> {
    x.iter().map(|xi| a * xi + b).collect()
}