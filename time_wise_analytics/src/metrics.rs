/// Calcula o coeficiente de determinação (R²).
pub fn r_squared(x: &[f64], y: &[f64], a: f64, b: f64) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let mean_y: f64 = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
    let ss_residual: f64 = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (a * xi + b)).powi(2))
        .sum();

    Some(1.0 - ss_residual / ss_total)
}

/// Calcula o erro quadrático médio (MSE).
pub fn mean_squared_error(x: &[f64], y: &[f64], a: f64, b: f64) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let mse: f64 = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (a * xi + b)).powi(2))
        .sum::<f64>()
        / x.len() as f64;

    Some(mse)
}

/// Calcula a média de um vetor de valores.
pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(data.iter().sum::<f64>() / data.len() as f64)
    }
}

/// Calcula a mediana de um vetor de valores.
pub fn median(data: &mut [f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        Some((data[mid - 1] + data[mid]) / 2.0)
    } else {
        Some(data[mid])
    }
}

/// Calcula o desvio padrão de um vetor de valores.
pub fn standard_deviation(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mean_value = mean(data)?;
    let variance = data.iter().map(|value| (value - mean_value).powi(2)).sum::<f64>() / data.len() as f64;
    Some(variance.sqrt())
}

/// Retorna o valor mínimo de um vetor de valores.
pub fn min(data: &[f64]) -> Option<f64> {
    data.iter().cloned().fold(None, |min, x| Some(min.map_or(x, |min| min.min(x))))
}

/// Retorna o valor máximo de um vetor de valores.
pub fn max(data: &[f64]) -> Option<f64> {
    data.iter().cloned().fold(None, |max, x| Some(max.map_or(x, |max| max.max(x))))
}