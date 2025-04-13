use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

/// Gera um arquivo PDF com os dados fornecidos.
pub fn generate_pdf(
    file_name: &str,
    mean: f64,
    median: f64,
    std_dev: f64,
    min: f64,
    max: f64,
    regression_a: f64,
    regression_b: f64,
    r_squared: f64,
    mse: f64,
    predictions: &[f64],
) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Relatório de Análise de Séries Temporais", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Carrega uma fonte padrão
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    // Adiciona título
    current_layer.use_text("Relatório de Análise de Séries Temporais", 24.0, Mm(20.0), Mm(280.0), &font);

    // Adiciona estatísticas descritivas
    current_layer.use_text(format!("Média: {:.2}", mean), 14.0, Mm(20.0), Mm(260.0), &font);
    current_layer.use_text(format!("Mediana: {:.2}", median), 14.0, Mm(20.0), Mm(250.0), &font);
    current_layer.use_text(format!("Desvio Padrão: {:.2}", std_dev), 14.0, Mm(20.0), Mm(240.0), &font);
    current_layer.use_text(format!("Valor Mínimo: {:.2}", min), 14.0, Mm(20.0), Mm(230.0), &font);
    current_layer.use_text(format!("Valor Máximo: {:.2}", max), 14.0, Mm(20.0), Mm(220.0), &font);

    // Adiciona informações da regressão linear
    current_layer.use_text(format!("Coeficiente a (inclinação): {:.2}", regression_a), 14.0, Mm(20.0), Mm(200.0), &font);
    current_layer.use_text(format!("Coeficiente b (intercepto): {:.2}", regression_b), 14.0, Mm(20.0), Mm(190.0), &font);
    current_layer.use_text(format!("Coeficiente de Determinação (R²): {:.4}", r_squared), 14.0, Mm(20.0), Mm(180.0), &font);
    current_layer.use_text(format!("Erro Quadrático Médio (MSE): {:.4}", mse), 14.0, Mm(20.0), Mm(170.0), &font);

    // Adiciona previsões
    current_layer.use_text("Previsões para os próximos períodos:", 14.0, Mm(20.0), Mm(150.0), &font);
    for (i, prediction) in predictions.iter().enumerate() {
        current_layer.use_text(
            format!("Período {}: {:.2}", i + 1, prediction),
            12.0,
            Mm(20.0),
            Mm(140.0 - (i as f64 * 10.0)),
            &font,
        );
    }

    // Salva o PDF
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)?;

    Ok(())
}